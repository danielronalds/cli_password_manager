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

pub enum SearchAction {
    NewAccount(String),
    ViewAccount(String),
    Exit,
}

pub fn search(accounts: &Vec<Account>) -> Result<SearchAction> {
    let mut search_term = String::new();
    let mut filtered_accounts = accounts.clone();
    let mut search_over = false;
    loop {
        draw_search_results(&filtered_accounts)?;
        let prompt = format!("{} ", " Search ".black().on_bright_white());
        search_term = match search_textfield(prompt, 9, search_term)? {
            SearchResult::ContinueSearch(search_term) => search_term,
            SearchResult::SearchFinished(search_term) => {
                search_over = true;
                search_term
            }
            SearchResult::Exit => return Ok(SearchAction::Exit),
        };
        if search_term.is_empty() && !search_over {
            filtered_accounts = accounts.clone();
            continue;
        }
        // This one really needs some work... DON'T FORGET TO CHANGE THIS
        filtered_accounts = accounts
            .iter()
            .filter(|x| {
                for letter in search_term.chars() {
                    if !x.label().contains(letter) {
                        return false;
                    }
                }
                true
            })
            .map(|x| x.to_owned())
            .collect();

            if search_over {
                break;
            }
    }
    match filtered_accounts.first() {
        Some(account) => Ok(SearchAction::ViewAccount(account.label())),
        None => Ok(SearchAction::NewAccount(search_term)),
    }
}

enum SearchResult {
    ContinueSearch(String),
    SearchFinished(String),
    Exit
}

fn search_textfield(prompt: String, prompt_len: u16, content: String) -> Result<SearchResult> {
    execute!(stdout(), cursor::Show, cursor::SetCursorStyle::SteadyBlock)?;

    let mut output = content;
    let prompt = prompt.to_string();

    let cursor = output.len();

    execute!(
        stdout(),
        Clear(ClearType::CurrentLine),
        Print(format!("\r{}{}", prompt, &output)),
        cursor::MoveToColumn(0),
        cursor::MoveRight(prompt_len + (cursor as u16))
    )?;
    if let Event::Key(key) = read()? {
        match key.code {
            KeyCode::Char(char) => {
                output.insert(cursor, char);
            }
            KeyCode::Backspace => {
                if !output.is_empty() {
                    output.remove(cursor - 1);
                }
            }
            KeyCode::Enter => return Ok(SearchResult::SearchFinished(output)),
            KeyCode::Esc => return Ok(SearchResult::Exit),
            _ => (),
        }
    }

    Ok(SearchResult::ContinueSearch(output))
}

fn draw_search_results(accounts: &[Account]) -> Result<()> {
    execute!(
        stdout(),
        cursor::MoveTo(0, 1),
        Clear(ClearType::FromCursorDown)
    )?;

    let mut accounts_iter = accounts.iter();

    if let Some(account) = accounts_iter.next() {
        terminal_drawing::println(format!("> {}", account.label()))?;
    }

    for account in accounts_iter {
        terminal_drawing::println(format!("  {}", account.label()))?;
    }

    execute!(stdout(), cursor::MoveTo(0, 0))?;

    Ok(())
}
