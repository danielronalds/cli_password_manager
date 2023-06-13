//! This module contains the change password screen logic

use crate::terminal_drawing::{box_label, textfield};
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

/// Entry point to changing the password of the app
///
/// # Arguments
///
/// * `old_password` - The old password
pub fn change_password(old_password: &str) -> crossterm::Result<Option<String>> {
    clear_screen()?;

    let entered_old_password = match textfield(
        format!("{} ", box_label("Enter Old Password")),
        21,
        "".to_string(),
    )? {
        Some(entered_old_password) => entered_old_password.trim().to_string(),
        None => return Ok(None),
    };

    if entered_old_password != old_password {
        return Ok(None);
    }

    let new_password = match textfield(
        format!("{} ", box_label("Enter New Password")),
        21,
        "".to_string(),
    )? {
        Some(new_password) => new_password.trim().to_string(),
        None => return Ok(None),
    };

    let confirmed_new_password = match textfield(
        format!("{} ", box_label("Confirmed New Password")),
        25,
        "".to_string(),
    )? {
        Some(confirmed_new_password) => confirmed_new_password.trim().to_string(),
        None => return Ok(None),
    };

    if confirmed_new_password != new_password {
        return Ok(None);
    }

    Ok(Some(new_password))
}

fn clear_screen() -> crossterm::Result<()> {
    execute!(stdout(), cursor::MoveTo(0, 0), Clear(ClearType::All))
}
