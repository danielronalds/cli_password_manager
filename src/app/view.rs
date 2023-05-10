use colored::Colorize;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
    Result,
};
use std::io::stdout;

use crate::account::Account;
use crate::terminal_drawing;

use AccountValue::*;

#[derive(Clone, Copy)]
enum AccountValue {
    Label,
    Username,
    Email,
    Password,
}

impl AccountValue {
    fn next(&self, account: &Account) -> AccountValue {
        match self {
            Label => match account.username() {
                Some(_) => Username,
                None => match account.email() {
                    Some(_) => Email,
                    None => Password,
                },
            },
            Username => match account.email() {
                Some(_) => Email,
                None => Password,
            },
            Email => Password,
            Password => Label,
        }
    }

    fn prev(&self, account: &Account) -> AccountValue {
        match self {
            Label => Password,
            Username => Label,
            Email => match account.username() {
                Some(_) => Username,
                None => Label,
            },
            Password => match account.email() {
                Some(_) => Email,
                None => match account.username() {
                    Some(_) => Username,
                    None => Label,
                },
            },
        }
    }
}

pub fn view(account: Account) -> Result<Account> {
    let mut current_value = Label;
    loop {
        draw_view(&account, current_value)?;
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('j') =>current_value = current_value.next(&account),
                KeyCode::Char('k') =>current_value = current_value.prev(&account),
                KeyCode::Esc | KeyCode::Char('q') => break,
                _ => (),
            }
        }
    }
    Ok(account)
}

fn draw_view(account: &Account, current_value: AccountValue) -> Result<()> {
    fn active_label<T: ToString>(label: T, value: T) -> Result<()> {
        let label = format!(" {} ", label.to_string().black()).on_bright_white();
        terminal_drawing::println(format!("{} {}", label, value.to_string()))?;
        Ok(())
    }

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

    match current_value {
        Label => {
            active_label("Label", &account.label())?;

            if let Some(username) = account.username() {
                terminal_drawing::println(format!(" Username: {}", username))?;
            }

            if let Some(email) = account.email() {
                terminal_drawing::println(format!(" Email: {}", email))?;
            }

            terminal_drawing::println(format!(" Password  {}", hidden_password))?;
        }
        Username => {
            terminal_drawing::println(format!(" Label  {}", account.label()))?;

            if let Some(username) = account.username() {
                active_label("Username", &username)?;
            }

            if let Some(email) = account.email() {
                terminal_drawing::println(format!(" Email: {}", email))?;
            }

            terminal_drawing::println(format!(" Password  {}", hidden_password))?;
        }
        Email => {
            terminal_drawing::println(format!(" Label  {}", account.label()))?;

            if let Some(username) = account.username() {
                terminal_drawing::println(format!(" Username: {}", username))?;
            }

            if let Some(email) = account.email() {
                active_label("Email", &email)?;
            }

            terminal_drawing::println(format!(" Password  {}", hidden_password))?;
        }
        Password => {
            terminal_drawing::println(format!(" Label  {}", account.label()))?;

            if let Some(username) = account.username() {
                terminal_drawing::println(format!(" Username: {}", username))?;
            }

            if let Some(email) = account.email() {
                terminal_drawing::println(format!(" Email: {}", email))?;
            }

            active_label("Password", &hidden_password)?;
        }
    }

    Ok(())
}
