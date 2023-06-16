//! This module contains everything thing to do with viewing an account in the application

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType},
    Result,
};
use std::io::stdout;

use arboard::Clipboard;

use crate::account::Account;
use crate::terminal_drawing::{box_label, get_confirmation, print, println, textfield};

use AccountField::*;

/// Enum that contains all the fields in the Account struct
#[derive(Clone, Copy)]
enum AccountField {
    Label,
    Username,
    Email,
    Password,
}

impl AccountField {
    /// Returns what the next field is
    fn next(&self) -> AccountField {
        match self {
            Label => Username,
            Username => Email,
            Email => Password,
            Password => Label,
        }
    }

    /// Returns what the previous field is
    fn prev(&self) -> AccountField {
        match self {
            Label => Password,
            Username => Label,
            Email => Username,
            Password => Email,
        }
    }
}

/// Entry point for the view page of the application. Allows the user to yank details like a
/// password into their system's clipboard, edit details to do with that account, and delete it.
///
/// # Arguments
///
/// * `account` - The account to view
///
/// # Returns
///
/// Either the account with the changes made, None if the user deletes it, or an IO error
pub fn view(account: Account) -> Result<Option<Account>> {
    let mut account = account;
    let mut current_field = Label;
    loop {
        draw_view(&account, current_field)?;
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('j') => current_field = current_field.next(),
                KeyCode::Char('k') => current_field = current_field.prev(),
                KeyCode::Char('e') => account = edit(account, current_field)?,
                KeyCode::Char('y') => yank_current_field(&account, current_field)?,
                KeyCode::Char('D') => {
                    if confirm_delete_list()? {
                        return Ok(None);
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') => break,
                _ => (),
            }
        }
    }
    Ok(Some(account))
}

/// Prompts the user to confirm whether they'd actually like to delete the account being viewed
///
/// # Returns
/// `true` if the user presses y or Y, any other key results in `false`. Otherwise an IO error
fn confirm_delete_list() -> Result<bool> {
    execute!(stdout(), cursor::MoveTo(0, 5))?;
    println("Are you sure you want to delete this account? [y/N]")?;
    get_confirmation()
}

/// Yanks the given field in the Account into the clipboard. For some reason this only works
/// while this function doesn't return, so at the end of a function there is a `read()` call, to
/// prevent the function from returning until a key is pressed.
///
/// # Arguments
///
/// * `account` - The account to yank the field from
/// * `field`   - The field to yank
///
/// # Returns
///
/// Can return an IO error
fn yank_current_field(account: &Account, field: AccountField) -> Result<()> {
    let mut clipboard = Clipboard::new().expect("Couldn't access the clipboard");
    // Yanking the given field to the clipboard
    match field {
        Label => return Ok(()),
        Username => {
            if let Some(username) = account.username() {
                clipboard
                    .set_text(username)
                    .expect("Couldn't access the clipboard")
            }
        }
        Email => {
            if let Some(email) = account.email() {
                clipboard
                    .set_text(email)
                    .expect("Couldn't access the clipboard")
            }
        }
        Password => clipboard
            .set_text(account.password())
            .expect("Couldn't access the clipboard"),
    }

    // Pausing execution so that the field stays in the clipboard
    execute!(stdout(), cursor::MoveTo(0, 5))?;

    println("Yanked! Press 'y' to wipe the clipboard")?;

    while !get_confirmation()? {} // While loop runs until the user press y

    Ok(())
}

/// Allows the user to edit the given field with a textfield
///
/// # Arguments
///
/// * `account`       - The account of the field to edit
/// * `current_field` - The field to edit
///
/// # Returns
///
/// An `Account` with the field with the user's edits, or an IO error
fn edit(account: Account, current_field: AccountField) -> Result<Account> {
    let mut account = account;

    // Figuring out what field to edit, getting the details to pass to the textfield and moving the
    // cursor to the fields line
    let (label, content) = match current_field {
        Label => {
            execute!(stdout(), cursor::MoveTo(0, 0))?;
            ("Label", account.label())
        }
        Username => {
            execute!(stdout(), cursor::MoveTo(0, 1))?;
            ("Username", account.username().unwrap_or("".to_string()))
        }
        Email => {
            execute!(stdout(), cursor::MoveTo(0, 2))?;
            ("Email", account.email().unwrap_or("".to_string()))
        }
        Password => {
            execute!(stdout(), cursor::MoveTo(0, 3))?;
            ("Password", account.password())
        }
    };

    let new_value = textfield(
        format!("{} ", box_label(label)),
        (label.len() + 3) as u16,
        content,
        false,
    )?;

    // Making the edit if an edit was made
    if let Some(new_value) = new_value {
        match current_field {
            Label => account.set_label(new_value),
            Username => match !new_value.is_empty() {
                true => account.set_username(Some(new_value)),
                false => account.set_username(None),
            },
            Email => match !new_value.is_empty() {
                true => account.set_email(Some(new_value)),
                false => account.set_email(None),
            },
            Password => account.set_password(new_value),
        }
    }

    Ok(account)
}

/// Draws the view page to the terminal
///
/// # Arguments
///
/// * `account`       - The account to view
/// * `current_field` - The current selected field
fn draw_view(account: &Account, current_field: AccountField) -> Result<()> {
    // Setting up the terminal screen
    execute!(
        stdout(),
        cursor::MoveTo(0, 0),
        Clear(ClearType::FromCursorDown),
        cursor::Hide
    )?;

    // Drawing the initial list
    println(format!(" Label  {}", account.label()))?;
    println(format!(
        " Username  {}",
        match account.username() {
            Some(username) => username,
            None => "".to_string(),
        }
    ))?;
    println(format!(
        " Email  {}",
        match account.email() {
            Some(email) => email,
            None => "".to_string(),
        }
    ))?;
    println(format!(
        " Password  {}",
        account.password().chars().map(|_| '*').collect::<String>()
    ))?;

    // Replacing the current fields normal label with the selected field version
    match current_field {
        Label => {
            execute!(stdout(), cursor::MoveTo(0, 0))?;
            print(box_label("Label"))?;
        }
        Username => {
            execute!(stdout(), cursor::MoveTo(0, 1))?;
            print(box_label("Username"))?;
        }
        Email => {
            execute!(stdout(), cursor::MoveTo(0, 2))?;
            print(box_label("Email"))?;
        }
        Password => {
            execute!(stdout(), cursor::MoveTo(0, 3))?;
            print(box_label("Password"))?;
        }
    };

    Ok(())
}
