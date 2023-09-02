//! This module contains the neccesary code for display a message on its own screen
use crossterm::{
    cursor,
    execute,
    terminal::{Clear, ClearType},
    Result,
};

use std::io::stdout;

use crate::terminal_drawing;

/// Clears the screen and displays a message until the user presses any key
///
/// # Arguments
///
/// * `message` - The message to display
pub fn show_notification<T: ToString>(message: T) -> Result<()> {
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    terminal_drawing::println(message.to_string())?;
    terminal_drawing::get_confirmation()?;
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    Ok(())
}
