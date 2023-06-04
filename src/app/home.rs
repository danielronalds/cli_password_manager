//! This module contains the home page of the application

use colored::Colorize;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType},
    Result,
};

use std::io::stdout;

use crate::terminal_drawing;

use PageOption::*;


/// Returns the given label with a whitebox around it as a String
///
/// # Arguments
///
/// * `label` - The label in the box
fn box_label<T: ToString>(label: T) -> String {
    format!(" {} ", label.to_string().black())
        .on_bright_white()
        .to_string()
}

#[derive(Clone, Copy)]
pub enum PageOption {
    Search,
    Settings,
    Help,
    Exit,
}

impl PageOption {
    pub fn next(&self) -> Self {
        match self {
            Search => Settings,
            Settings => Help,
            Help => Exit,
            Exit => Search
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Search => Exit,
            Settings => Search,
            Help => Settings,
            Exit => Help
        }
    }
}

/// Entry point for the home page of the application. Allows the user to decide what page they'd
/// like to enter
pub fn home() -> crossterm::Result<PageOption> {
    let mut current_option = PageOption::Search;

    loop {
        draw_home(current_option)?;
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('j') => current_option = current_option.next(),
                KeyCode::Char('k') => current_option = current_option.prev(),
                KeyCode::Esc | KeyCode::Char('q') => return Ok(Exit),
                KeyCode::Enter => break,
                _ => (),
            }
        }
    }

    Ok(current_option)
}

fn draw_home(current_option: PageOption) -> Result<()> {
    execute!(
        stdout(),
        cursor::MoveTo(0, 0),
        Clear(ClearType::FromCursorDown),
        cursor::Hide
    )?;

    terminal_drawing::println(" Search Accounts ")?;
    terminal_drawing::println(" Settings ")?;
    terminal_drawing::println(" Help ")?;
    terminal_drawing::println(" Exit ")?;

    let current_option_line = match current_option {
        Search => 0,
        Settings => 1,
        Help => 2,
        Exit => 3,
    };

    let selected_text = match current_option {
        Search => box_label("Search Accounts"), 
        Settings => box_label("Settings"), 
        Help => box_label("Help"), 
        Exit => box_label("Exit"), 
    };

    execute!(stdout(), cursor::MoveTo(0, current_option_line))?;

    terminal_drawing::print(selected_text)?;

    Ok(())
}
