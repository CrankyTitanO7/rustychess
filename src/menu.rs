// Menu screens for rustychess (ratatui + crossterm).
//
// `start_menu` is the entry point called from `main.rs`. It owns the
// top-level loop and dispatches into the sub-menus. Each sub-menu loops
// until the user goes "back" (returns to its caller) or "quits" (exits
// the whole menu system).

use crossterm::event::{self, Event, KeyCode};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::gameloops;

type Term = Terminal<CrosstermBackend<std::io::Stdout>>;
type MenuResult<T> = Result<T, Box<dyn std::error::Error>>;

const SELECTED_STR: &str = ">> ";
const HELP_TEXT: &str = "↑/↓ or j/k: move • Enter/Space: select • q/Esc: back/quit";

const START_TITLE: &str = "rustychess: a beginner's project";
const START_ITEMS: &[&str] = &["New Game", "Replay Saved Game", "Settings", "Quit"];

const NEWGAME_TITLE: &str = "rustychess: new game ?";
const NEWGAME_ITEMS: &[&str] = &["PvP", "PvC", "CvC", "Back to Main", "Quit"];
const NEWGAME_MESSAGES: &[&str] = &[
    "Starting Player vs Computer...\n\n(Gameplay not wired up yet. Press any key.)",
    "Starting Computer vs Computer...\n\n(Gameplay not wired up yet. Press any key.)",
];

const REPLAY_TITLE: &str = "rustychess: replay a game ?";
const REPLAY_ITEMS: &[&str] = &["Select Game From File", "Back to Main", "Quit"];
const REPLAY_MESSAGE: &str =
    "Searching for saved games...\n\n(Replay from file is not wired up yet. Press any key.)";

const SETTINGS_TITLE: &str = "rustychess: settings ?";
const SETTINGS_ITEMS: &[&str] = &["Flip Board", "Toggle Coordinates", "Back to Main", "Quit"];
const SETTINGS_MESSAGES: &[&str] = &[
    "Board orientation flipped.\n\n(Placeholder — no persistent setting yet. Press any key.)",
    "Coordinate display toggled.\n\n(Placeholder — no persistent setting yet. Press any key.)",
];

/// Outcome of a sub-menu: go back to the caller, or quit the app entirely.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SubMenuOutcome {
    Back,
    QuitApp,
}

/// Generic single-level selector.
///
/// Renders `items` with `title`, lets the user move with Up/Down (or
/// j/k, Home/End) with wrap-around, and returns:
/// - `Ok(Some(i))` when Enter/Space (or a `1`-`9` shortcut) picks item `i`,
/// - `Ok(None)` when the user presses `q`/`Esc` (cancel / back).
fn run_select_menu(terminal: &mut Term, title: &str, items: &[&str]) -> MenuResult<Option<usize>> {
    if items.is_empty() {
        show_message(terminal, title, "Nothing to select.\n\nPress any key.")?;
        return Ok(None);
    }

    let mut selected: usize = 0;

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(frame.area());

            let list_items: Vec<ListItem> =
                items.iter().map(|item| ListItem::new(*item)).collect();

            let list = List::new(list_items)
                .block(Block::default().title(title).borders(Borders::ALL))
                .highlight_symbol(SELECTED_STR)
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .repeat_highlight_symbol(false);

            let mut state = ListState::default();
            state.select(Some(selected));
            frame.render_stateful_widget(list, chunks[0], &mut state);

            let help = Paragraph::new(HELP_TEXT)
                .block(Block::default().borders(Borders::ALL))
                .wrap(Wrap { trim: true });
            frame.render_widget(help, chunks[1]);
        })?;

        if !event::poll(std::time::Duration::from_millis(100))? {
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };

        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                selected = selected.checked_sub(1).unwrap_or(items.len() - 1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                selected = (selected + 1) % items.len();
            }
            KeyCode::Home => selected = 0,
            KeyCode::End => selected = items.len() - 1,
            KeyCode::Enter | KeyCode::Char(' ') => return Ok(Some(selected)),
            // Number shortcuts: `1`..=`9` jumps straight to that row.
            KeyCode::Char(c @ '1'..='9') => {
                let idx = (c as usize) - ('1' as usize);
                if idx < items.len() {
                    return Ok(Some(idx));
                }
            }
            KeyCode::Char('q') | KeyCode::Esc => return Ok(None),
            _ => {}
        }
    }
}

/// Full-screen notice. Draws `message` under `title` and returns on any
/// key press. Used to acknowledge a choice whose real action (gameplay,
/// file picker, persistent settings) is not implemented yet.
fn show_message(terminal: &mut Term, title: &str, message: &str) -> MenuResult<()> {
    loop {
        terminal.draw(|frame| {
            let paragraph = Paragraph::new(message)
                .block(Block::default().title(title).borders(Borders::ALL))
                .wrap(Wrap { trim: false });
            frame.render_widget(paragraph, frame.area());
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if matches!(event::read()?, Event::Key(_)) {
                return Ok(());
            }
        }
    }
}

fn run_new_game_menu(terminal: &mut Term) -> MenuResult<SubMenuOutcome> {
    loop {
        match run_select_menu(terminal, NEWGAME_TITLE, NEWGAME_ITEMS)? {
            // PvP: hand the terminal to the PvP game loop, then stay in this menu.
            Some(0) => {
                gameloops::pvp(terminal)?;
            }
            // PvC / CvC: acknowledge, then stay in this menu.
            Some(i @ 1..=2) => {
                let message = NEWGAME_MESSAGES
                    .get(i - 1)
                    .copied()
                    .unwrap_or("Starting...");
                show_message(terminal, NEWGAME_TITLE, message)?;
            }
            // Back to Main (or q/Esc): return to the caller.
            Some(3) | None => return Ok(SubMenuOutcome::Back),
            // Quit: exit the whole menu system.
            Some(4) => return Ok(SubMenuOutcome::QuitApp),
            // Defensive: ignore any future out-of-range index.
            Some(_) => {}
        }
    }
}

fn run_replay_menu(terminal: &mut Term) -> MenuResult<SubMenuOutcome> {
    loop {
        match run_select_menu(terminal, REPLAY_TITLE, REPLAY_ITEMS)? {
            Some(0) => show_message(terminal, REPLAY_TITLE, REPLAY_MESSAGE)?,
            Some(1) | None => return Ok(SubMenuOutcome::Back),
            Some(2) => return Ok(SubMenuOutcome::QuitApp),
            Some(_) => {}
        }
    }
}

fn run_settings_menu(terminal: &mut Term) -> MenuResult<SubMenuOutcome> {
    loop {
        match run_select_menu(terminal, SETTINGS_TITLE, SETTINGS_ITEMS)? {
            Some(i @ 0..=1) => {
                let message = SETTINGS_MESSAGES.get(i).copied().unwrap_or("...");
                show_message(terminal, SETTINGS_TITLE, message)?;
            }
            Some(2) | None => return Ok(SubMenuOutcome::Back),
            Some(3) => return Ok(SubMenuOutcome::QuitApp),
            Some(_) => {}
        }
    }
}

/// Top-level menu. Loops until the user picks Quit (or presses q/Esc).
pub fn start_menu(terminal: &mut Term) -> MenuResult<()> {
    loop {
        match run_select_menu(terminal, START_TITLE, START_ITEMS)? {
            Some(0) => match run_new_game_menu(terminal)? {
                SubMenuOutcome::Back => {}
                SubMenuOutcome::QuitApp => break,
            },
            Some(1) => match run_replay_menu(terminal)? {
                SubMenuOutcome::Back => {}
                SubMenuOutcome::QuitApp => break,
            },
            Some(2) => match run_settings_menu(terminal)? {
                SubMenuOutcome::Back => {}
                SubMenuOutcome::QuitApp => break,
            },
            // Quit, q/Esc, or any unknown index: leave the menu system.
            Some(_) | None => break,
        }
    }
    Ok(())
}

/// Standalone New Game menu (also reachable via `start_menu`).
/// `Quit` here just returns; when driven from `start_menu` the inner
/// runner above propagates a full-app quit instead.
pub fn new_game(terminal: &mut Term) -> MenuResult<()> {
    let _ = run_new_game_menu(terminal)?;
    Ok(())
}

/// Standalone Replay menu.
pub fn replay_menu(terminal: &mut Term) -> MenuResult<()> {
    let _ = run_replay_menu(terminal)?;
    Ok(())
}

/// Standalone Settings menu.
pub fn settings_menu(terminal: &mut Term) -> MenuResult<()> {
    let _ = run_settings_menu(terminal)?;
    Ok(())
}
