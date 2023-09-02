//! This module contains the neccesary code for the help page
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
    Result,
};

use std::io::stdout;

use crate::terminal_drawing;

const MAX_KEYBIND_LENGTH: usize = 5;

/// Displays the help menu
pub fn show_help_menu() -> Result<()> {
    let keybinds = vec![
        ("j", "Move cursor up"),
        ("k", "Move cursor down"),
        ("q", "Exit (Sometimes)"),
        ("ESC", "Exit"),
        ("ENTER", "Select"),
        ("", ""),
        ("Account View", ""),
        ("e", "Edit field"),
        ("G", "Generate random password"),
    ];

    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    terminal_drawing::println(terminal_drawing::box_label("Help"))?;

    for keybind in keybinds {
        let padded_key = {
            let mut padded_key = keybind.0.to_string();
            for _ in 0..(MAX_KEYBIND_LENGTH.saturating_sub(keybind.0.len())) {
                padded_key.push(' ');
            }
            padded_key
        };
        terminal_drawing::println(format!(" {}   {}", padded_key, keybind.1))?;
    }

    terminal_drawing::get_confirmation()?;
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    Ok(())
}
