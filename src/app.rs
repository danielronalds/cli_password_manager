//! This module contains the entry point for the CLI application
mod change_password;
mod home;
mod search;
mod view;

use crate::account::Account;
use colored::Colorize;
use crossterm::{
    cursor,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};

use change_password::change_password;
use home::{home, PageOption};
use search::{search, SearchAction};
use view::view;

use crate::serialisation::{deserialise, read_password_file, DeserialisationResult};
use crate::terminal_drawing::{box_label, textfield};

/// Prompts the user to login
///
/// # Returns
///
/// Either the password the user entered, or an io error
pub fn login() -> crossterm::Result<String> {
    enable_raw_mode()?;
    let prompt = format!("{} ", box_label("Enter Password"));
    let password = textfield(prompt, 17, "".to_string(), true)?;
    disable_raw_mode()?;
    Ok(password.unwrap_or(String::new()))
}

/// Setup function, that reads the file, confirms the password, and returns the password submitted,
/// magic_crypt codec, and the vector of accounts. If the user has not got a password file, then
/// the program suggests creating a new one.
///
/// # Arguments
///
/// * `password_file` - The path of the password file
///
/// # Returns
///
/// `None` if the user enters the wrong password or chooses to not create a password file.
/// Otherwise a tuple with a vectors of Accounts, the entered password, then the codec used.
pub fn setup(password_file: &str) -> Option<(Vec<Account>, String)> {
    match read_password_file(password_file) {
        Ok(password_file) => {
            let password = login().unwrap();

            match deserialise(password_file, password.trim()) {
                DeserialisationResult::WrongPassword => {
                    eprintln!(
                        "\r{} Thats the wrong password!",
                        " WARNING ".black().on_yellow()
                    );
                    None
                }
                DeserialisationResult::Ok(accounts) => Some((accounts, password)),
            }
        }
        Err(_) => {
            eprintln!(
                "{} Password file not found! create one? (y/N)",
                " Warning ".black().on_yellow()
            );
            enable_raw_mode().unwrap();
            let confirmation = crate::terminal_drawing::get_confirmation().unwrap();
            disable_raw_mode().unwrap();

            if confirmation {
                let password = login().unwrap();
                return Some((vec![], password));
            }

            None
        }
    }
}

/// The entry point for the password manager application
///
/// # Arguments
///
/// * `accounts` - The accounts to run the application with
/// * `password` - The password to the vault
///
/// # Returns
///
/// A tuple with a Vec of accounts and the vaults password
pub fn run(accounts: Vec<Account>, password: String) -> crossterm::Result<(Vec<Account>, String)> {
    let mut accounts = accounts;
    let mut password = password.to_string();

    enable_raw_mode()?;

    loop {
        match home()? {
            PageOption::Search => loop {
                let search_result = search(&accounts)?;
                match search_result {
                    SearchAction::ViewAccount(account_label) => {
                        let index = accounts
                            .iter()
                            .position(|x| x.label() == account_label)
                            .unwrap();
                        match view(accounts[index].clone())? {
                            Some(account) => {
                                accounts[index] = account;
                            }
                            None => {
                                accounts.remove(index);
                            }
                        }
                    }
                    SearchAction::NewAccount(new_account_label) => {
                        let new_account =
                            view::view(Account::builder().label(new_account_label).build())?;

                        if let Some(new_account) = new_account {
                            if !account_with_label(&accounts, &new_account.label()) {
                                accounts.push(new_account);
                            }
                        }
                    }
                    SearchAction::Exit => break,
                };
            },
            PageOption::ChangePassword => {
                if let Some(new_password) = change_password(&password.trim())? {
                    password = new_password;
                }
            }
            PageOption::Help => unimplemented!(),
            PageOption::Exit => break,
        }
    }

    crossterm::execute!(
        std::io::stdout(),
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All),
        cursor::Show
    )?;

    disable_raw_mode()?;

    Ok((accounts, password))
}

/// Returns whether the given accounts slice has an account with the given label
///
/// # Arguments
///
/// * `accounts` - The slice of accounts
/// * `new_label` - The label to search for a clash with
fn account_with_label(accounts: &[Account], new_label: &str) -> bool {
    for account in accounts {
        if account.label() == new_label {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::account_with_label;
    use crate::account::Account;
    #[test]
    /// This function tests that the account_with_label() function works as expected
    fn account_with_label_works() {
        let accounts = vec![
            Account::builder().label("Test 1").build(),
            Account::builder().label("Test 2").build(),
            Account::builder().label("Test 3").build(),
        ];

        assert!(account_with_label(&accounts, "Test 1"));
        assert!(account_with_label(&accounts, "Test 2"));
        assert!(!account_with_label(&accounts, "Test 4"));
    }
}
