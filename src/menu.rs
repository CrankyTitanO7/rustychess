// they call me the menu

// use std::env;

// use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    // execute,
    // terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, ListState},
};

// const VEGETABLES: &[&str] = &["carrot", "potato", "broccoli", "spinach"];

const SELECTED_STR: &str = ">> ";

const START_ITEMS: &[&str] = &["New Game", "Replay Saved Game", "Settings", "Quit"];
const START_DIALOGUE: &[&str] = &["Starting...", "Projects", "Settings"];

const NEWGAME_ITEMS: &[&str] = &["PvP", "PvC", "CvC", "back to main", "Quit"];
const NEWGAME_DIALOGUE: &[&str] = &["Starting...", "Starting...", "Starting..."];

const REPLAY_ITEMS: &[&str] = &["select game from file", "back to main", "Quit"];
const REPLAY_DIALOGUE: &[&str] = &["SEARCHING...", "...", "..."];

const SETTINGS_ITEMS: &[&str] = &["", "back to main", "Quit"];
const SETTINGS_DIALOGUE: &[&str] = &["...", "...", "..."];

fn menu(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    options: &[&str],
    selected_dialogue: &[&str],
    title: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let items = options;

    let mut selected = 0;

    loop {
        terminal.draw(|frame| {
            let list_items = items
                .iter()
                .map(|item| ListItem::new(*item))
                .collect::<Vec<_>>();

            let list = List::new(list_items)
                .block(Block::default().title(title).borders(Borders::ALL))
                .highlight_symbol(SELECTED_STR);

            let mut state = ListState::default();
            state.select(Some(selected));

            frame.render_stateful_widget(list, frame.area(), &mut state);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }

                    KeyCode::Down => {
                        if selected < items.len() - 1 {
                            selected += 1;
                        }
                    }

                    KeyCode::Enter => match selected {
                        0 => println!("{}", selected_dialogue[0]),
                        1 => println!("{}", selected_dialogue[1]),
                        2 => println!("{}", selected_dialogue[2]),
                        3 => break,
                        _ => {}
                    },

                    KeyCode::Char('q') => break,

                    _ => {}
                }
            }
        }
    }

    Ok(())
}

pub fn start_menu(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    menu(terminal, START_ITEMS, START_DIALOGUE, "rustychess: a beginner's project")
}

pub fn replay_menu(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    menu(terminal,  REPLAY_ITEMS, REPLAY_DIALOGUE, "rustychess: replay a game ?")
}

pub fn settings_menu(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    menu(terminal, SETTINGS_ITEMS, SETTINGS_DIALOGUE, "rustychess: settings ?")
}

pub fn new_game(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    menu(terminal, NEWGAME_ITEMS, NEWGAME_DIALOGUE, "rustychess: new game ?")
}

// pub fn new_game ()
