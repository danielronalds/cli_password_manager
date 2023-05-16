//! This module contains the entry point for the CLI application
mod search;
mod view;

use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};

use crate::account::Account;

use search::{search, SearchAction};
use view::view;

use std::io::Write;

/// Prompts the user to login
///
/// # Returns
///
/// Either the password the user entered, or an io error
pub fn login() -> std::io::Result<String> {
    print!("{} ", view::box_label("Enter Password"));
    std::io::stdout().flush().expect("Failed to flush");

    let mut password = String::new();
    std::io::stdin().read_line(&mut password)?;
    Ok(password)
}

/// The entry point for the password manager application
///
/// # Arguments
///
/// # `accounts` - The accounts to run the application with
pub fn run(accounts: Vec<Account>) -> crossterm::Result<Vec<Account>> {
    let mut accounts = accounts;

    enable_raw_mode()?;

    loop {
        let search_result = search(&accounts)?;
        match search_result {
            SearchAction::ViewAccount(account_label) => {
                let index = accounts
                    .iter()
                    .position(|x| x.label() == account_label)
                    .unwrap();
                match view(accounts[index].clone())? {
                    Some(account) => accounts[index] = account,
                    None => {
                        accounts.remove(index);
                    }
                }
            }
            SearchAction::NewAccount(new_account_label) => {
                let new_account = view::view(Account::builder().label(new_account_label).build())?;

                if let Some(new_account) = new_account {
                    accounts.push(new_account);
                }
            }
            SearchAction::Exit => break,
        };
    }

    crossterm::execute!(std::io::stdout(), terminal::Clear(terminal::ClearType::All))?;

    disable_raw_mode()?;

    Ok(accounts)
}
