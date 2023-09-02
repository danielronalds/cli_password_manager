//! This module contains the change password screen logic

use crate::terminal_drawing::{box_label, textfield};
use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

pub enum PasswordResult {
    NewPassword(String),
    Error(String),
    None,
}

/// Entry point to changing the password of the app
///
/// # Arguments
///
/// * `old_password` - The old password
pub fn change_password(old_password: &str) -> crossterm::Result<PasswordResult> {
    clear_screen()?;

    let entered_old_password = match textfield(
        format!("{} ", box_label("Enter Old Password")),
        21,
        "".to_string(),
        true,
    )? {
        Some(entered_old_password) => entered_old_password.trim().to_string(),
        None => return Ok(PasswordResult::None),
    };

    if entered_old_password != old_password {
        return Ok(PasswordResult::Error("Incorrect password!".to_string()));
    }

    let new_password = match textfield(
        format!("{} ", box_label("Enter New Password")),
        21,
        "".to_string(),
        true,
    )? {
        Some(new_password) => new_password.trim().to_string(),
        None => return Ok(PasswordResult::None),
    };

    let confirmed_new_password = match textfield(
        format!("{} ", box_label("Confirmed New Password")),
        25,
        "".to_string(),
        true,
    )? {
        Some(confirmed_new_password) => confirmed_new_password.trim().to_string(),
        None => return Ok(PasswordResult::None),
    };

    if confirmed_new_password != new_password {
        return Ok(PasswordResult::Error("Passwords do not match!".to_string()));
    }

    Ok(PasswordResult::NewPassword(new_password))
}

fn clear_screen() -> crossterm::Result<()> {
    execute!(stdout(), cursor::MoveTo(0, 0), Clear(ClearType::All))
}
