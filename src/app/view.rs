use colored::Colorize;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType},
    Result,
};
use std::io::stdout;

use crate::account::Account;
use crate::terminal_drawing;

use AccountValue::*;

/// Returns the given label with a whitebox around it as a String
///
/// # Arguments
///
/// * `label` - The label in the box
pub fn box_label<T: ToString>(label: T) -> String {
    format!(" {} ", label.to_string().black()).on_bright_white().to_string()
}

#[derive(Clone, Copy)]
enum AccountValue {
    Label,
    Username,
    Email,
    Password,
}

impl AccountValue {
    fn next(&self) -> AccountValue {
        match self {
            Label => Username,
            Username => Email,
            Email => Password,
            Password => Label,
        }
    }

    fn prev(&self) -> AccountValue {
        match self {
            Label => Password,
            Username => Label,
            Email => Username,
            Password => Email,
        }
    }
}

pub fn view(account: Account) -> Result<Account> {
    let mut current_value = Label;
    loop {
        draw_view(&account, current_value)?;
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('j') => current_value = current_value.next(),
                KeyCode::Char('k') => current_value = current_value.prev(),
                KeyCode::Esc | KeyCode::Char('q') => break,
                _ => (),
            }
        }
    }
    Ok(account)
}

fn draw_view(account: &Account, current_value: AccountValue) -> Result<()> {
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

    match current_value {
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
