// main function 
// orchestrates the main conduction of the things.
mod board;
mod constants;
mod display;
mod log_move;
mod pieces;

mod menu;
mod gameloops;


// use std::io::{self, stdout};
use std::io::{ stdout};


use crossterm::{
    // event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    // widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = menu::start_menu(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

    