// PvP game loop, callable from the menu display loops.
//
// Call site:
//   crate::gameloops::pvp(terminal)?;
// from inside `menu.rs` (`run_new_game_menu`).
//
// Owns a fresh `Board::new_board()` and a text-input loop: the player types
// moves like `e2 e4` (or `e2e4`, `e2-e4`, `e2xe4`, plus `=Q` promotion) and
// presses Enter. Moves go through `Board::try_move`, which relocates the
// `Piece` (updating `locx`/`locy`), promotes via `Piece::pawn_to_x`, and
// records via `Log::add_move_to_log`. History is read back with
// `Log::movesearch`. Rendering reuses `display::board_text`.
// Esc clears the input (or exits when empty); `q` with empty input also exits.

use crossterm::event::{self, Event, KeyCode};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::board::Board;
use crate::display;

type Term = Terminal<CrosstermBackend<std::io::Stdout>>;

const PVP_TITLE: &str = "rustychess: player vs player";
const PVP_HELP: &str =
    "type e2 e4 + Enter • Backspace edits • Esc clears/exits • q exits when input empty";
const PVP_START_NOTICE: &str = "White to move. Type e.g. `e2 e4`, then Enter.";
const MAX_INPUT: usize = 16;
const HISTORY_SHOWN: usize = 5;

/// Last `n` logged moves, newest at the bottom, via `Log::movesearch`.
fn recent_moves_text(game: &Board, n: usize) -> String {
    if game.log.num_moves == 0 {
        return "no moves yet".to_string();
    }
    let start = game.log.num_moves.saturating_sub(n);
    let mut out = String::new();
    for i in start..game.log.num_moves {
        match game.log.movesearch(i) {
            Some(m) => out.push_str(&format!("{}: {}\n", i + 1, m)),
            None => out.push_str(&format!("{}: <missing>\n", i + 1)),
        }
    }
    out.trim_end().to_string()
}

/// Instantiate a new PvP chess game and run its input + display loop.
///
/// Takes over `terminal` until the user quits, then returns `Ok(())` so the
/// caller's menu loop can resume.
pub fn pvp(terminal: &mut Term) -> Result<(), Box<dyn std::error::Error>> {
    // Fresh PvP game: standard starting position, empty move log.
    let mut game = Board::new_board();
    let mut input = String::new();
    let mut notice = PVP_START_NOTICE.to_string();

    loop {
        let board_text = display::board_text(&game);
        let status = format!(
            "{} to move • moves played: {} • last: {}",
            game.turn_name(),
            game.log.num_moves,
            game.last_move().map(|s| s.as_str()).unwrap_or("-"),
        );
        let history = recent_moves_text(&game, HISTORY_SHOWN);
        let input_line = format!("> {input}");

        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(3),
                    Constraint::Length((HISTORY_SHOWN + 2) as u16),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(frame.area());

            frame.render_widget(
                Paragraph::new(board_text.clone())
                    .block(Block::default().title(PVP_TITLE).borders(Borders::ALL))
                    .wrap(Wrap { trim: false }),
                chunks[0],
            );
            frame.render_widget(
                Paragraph::new(status.clone())
                    .block(Block::default().title("status").borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                chunks[1],
            );
            frame.render_widget(
                Paragraph::new(history.clone())
                    .block(Block::default().title("moves").borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                chunks[2],
            );
            frame.render_widget(
                Paragraph::new(input_line.clone())
                    .block(
                        Block::default()
                            .title(format!("move ({})", notice))
                            .borders(Borders::ALL),
                    )
                    .wrap(Wrap { trim: true }),
                chunks[3],
            );
            frame.render_widget(
                Paragraph::new(PVP_HELP)
                    .block(Block::default().borders(Borders::ALL))
                    .wrap(Wrap { trim: true }),
                chunks[4],
            );
        })?;

        if !event::poll(std::time::Duration::from_millis(100))? {
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };

        match key.code {
            KeyCode::Esc => {
                if input.is_empty() {
                    return Ok(());
                }
                input.clear();
                notice = "input cleared".to_string();
            }
            KeyCode::Enter => {
                let trimmed = input.trim().to_string();
                if trimmed.is_empty() {
                    notice = "type a move first (e.g. e2 e4)".to_string();
                } else {
                    match game.try_move(&trimmed) {
                        Ok(alg) => {
                            notice = format!("played {alg} ({})", game.turn_name());
                        }
                        Err(e) => {
                            notice = format!("illegal: {e}");
                        }
                    }
                    input.clear();
                }
            }
            KeyCode::Backspace => {
                input.pop();
            }
            KeyCode::Char(c) => {
                // `q` on an empty buffer quits (legacy menu behaviour);
                // otherwise it is move text (e.g. `=Q` promotion).
                if (c == 'q' || c == 'Q') && input.is_empty() {
                    return Ok(());
                }
                if input.len() < MAX_INPUT && (c.is_ascii_graphic() || c == ' ') {
                    input.push(c);
                }
            }
            _ => {}
        }
    }
}
