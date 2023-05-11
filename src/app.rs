mod search;
mod view;

use colored::Colorize;
use crossterm::{
    cursor::{self, RestorePosition, SavePosition},
    event::{read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    Result,
};
use std::io::stdout;

use crate::account::Account;

use search::{search, SearchAction};

pub struct PasswordManagerApp {
    accounts: Vec<Account>,
}

impl PasswordManagerApp {
    pub fn new(accounts: Vec<Account>) -> Self {
        Self { accounts }
    }

    pub fn run(&mut self) -> Result<Vec<Account>> {
        enable_raw_mode()?;
        loop {
            let search_result = search(&self.accounts)?;
            execute!(stdout(), cursor::MoveTo(0, 0), Clear(ClearType::All))?;
            match search_result {
                SearchAction::ViewAccount(account_label) => {
                    let index = self
                        .accounts
                        .iter()
                        .position(|x| x.label() == account_label)
                        .unwrap();
                    self.accounts[index] = view::view(self.accounts[index].clone())?;
                }
                SearchAction::NewAccount(new_account_label) => {
                    let new_account =
                        view::view(Account::builder().label(new_account_label).build())?;
                    self.accounts.push(new_account);
                }
                SearchAction::Exit => break,
            };
        }
        disable_raw_mode()?;
        Ok(self.accounts.clone())
    }
}
