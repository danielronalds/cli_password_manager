//! This module contains the home page of the application

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType},
    Result,
};

use std::io::stdout;

use crate::terminal_drawing::{box_label, print, println};

use PageOption::*;

#[derive(Clone, Copy)]
pub enum PageOption {
    Search,
    ChangePassword,
    Help,
    Exit,
}

impl PageOption {
    pub fn next(&self) -> Self {
        match self {
            Search => ChangePassword,
            ChangePassword => Help,
            Help => Exit,
            Exit => Search,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Search => Exit,
            ChangePassword => Search,
            Help => ChangePassword,
            Exit => Help,
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

    println(format!("Password Manager v{}", env!("CARGO_PKG_VERSION")))?;
    println(" Search Accounts ")?;
    println(" Change Password ")?;
    println(" Help ")?;
    println(" Exit ")?;

    let current_option_line = match current_option {
        Search => 1,
        ChangePassword => 2,
        Help => 3,
        Exit => 4,
    };

    let selected_text = match current_option {
        Search => box_label("Search Accounts"),
        ChangePassword => box_label("Change Password"),
        Help => box_label("Help"),
        Exit => box_label("Exit"),
    };

    execute!(stdout(), cursor::MoveTo(0, current_option_line))?;

    print(selected_text)?;

    Ok(())
}
