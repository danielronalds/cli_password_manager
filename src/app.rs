mod search;

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
use crate::terminal_drawing;

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
        let search_result = search(&self.accounts)?;
        execute!(stdout(), cursor::MoveTo(0, 0), Clear(ClearType::All))?;
        disable_raw_mode()?;
        match search_result {
            SearchAction::ViewAccount(_) => println!("View"),
            SearchAction::NewAccount(_) => println!("Create new"),
            SearchAction::Exit => println!("Exit"),
        };
        Ok(vec![])
    }
}
