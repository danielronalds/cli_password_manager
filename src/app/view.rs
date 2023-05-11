use colored::Colorize;
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
use crate::terminal_drawing;

use AccountField::*;

/// Returns the given label with a whitebox around it as a String
///
/// # Arguments
///
/// * `label` - The label in the box
pub fn box_label<T: ToString>(label: T) -> String {
    format!(" {} ", label.to_string().black())
        .on_bright_white()
        .to_string()
}

#[derive(Clone, Copy)]
enum AccountField {
    Label,
    Username,
    Email,
    Password,
}

impl AccountField {
    fn next(&self) -> AccountField {
        match self {
            Label => Username,
            Username => Email,
            Email => Password,
            Password => Label,
        }
    }

    fn prev(&self) -> AccountField {
        match self {
            Label => Password,
            Username => Label,
            Email => Username,
            Password => Email,
        }
    }
}

pub fn view(account: Account) -> Result<Account> {
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
                KeyCode::Esc | KeyCode::Char('q') => break,
                _ => (),
            }
        }
    }
    Ok(account)
}

fn yank_current_field(account: &Account, field: AccountField) -> Result<()> {
    let mut clipboard = Clipboard::new().expect("Couldn't access the clipboard");
    match field {
        Label => (),
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

    execute!(stdout(), cursor::MoveTo(0, 5))?;

    terminal_drawing::println("Yanked! Press any key to wipe the clipboard")?;

    read()?;

    Ok(())
}

fn edit(account: Account, current_field: AccountField) -> Result<Account> {
    let mut account = account;

    let (label, content) = match current_field {
        Label => {
            cursor::MoveTo(0, 0);
            ("Label", account.label())
        }
        Username => {
            cursor::MoveTo(0, 1);
            ("Username", account.username().unwrap_or("".to_string()))
        }
        Email => {
            cursor::MoveTo(0, 2);
            ("Email", account.email().unwrap_or("".to_string()))
        }
        Password => {
            cursor::MoveTo(0, 3);
            ("Password", account.password())
        }
    };

    let new_value = terminal_drawing::textfield(
        format!("{} ", box_label(label)),
        (label.len() + 3) as u16,
        content,
    )?;

    if let Some(new_value) = new_value {
        match current_field {
            Label => account.set_label(new_value),
            Username => match new_value.is_empty() {
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

fn draw_view(account: &Account, current_field: AccountField) -> Result<()> {
    execute!(
        stdout(),
        cursor::MoveTo(0, 0),
        Clear(ClearType::FromCursorDown),
        cursor::Hide
    )?;

    let mut hidden_password = String::new();

    for _ in account.password().chars() {
        hidden_password.push('*');
    }

    terminal_drawing::println(format!(" Label  {}", account.label()))?;
    terminal_drawing::println(format!(
        " Username  {}",
        match account.username() {
            Some(username) => username,
            None => "".to_string(),
        }
    ))?;
    terminal_drawing::println(format!(
        " Email  {}",
        match account.email() {
            Some(email) => email,
            None => "".to_string(),
        }
    ))?;
    terminal_drawing::println(format!(" Password  {}", hidden_password))?;

    match current_field {
        Label => {
            execute!(stdout(), cursor::MoveTo(0, 0))?;
            terminal_drawing::print(box_label("Label"))?;
        }
        Username => {
            execute!(stdout(), cursor::MoveTo(0, 1))?;
            terminal_drawing::print(box_label("Username"))?;
        }
        Email => {
            execute!(stdout(), cursor::MoveTo(0, 2))?;
            terminal_drawing::print(box_label("Email"))?;
        }
        Password => {
            execute!(stdout(), cursor::MoveTo(0, 3))?;
            terminal_drawing::print(box_label("Password"))?;
        }
    };

    Ok(())
}
