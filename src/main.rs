mod account;

use account::Account;
use magic_crypt::new_magic_crypt;

use std::fs::File;
use std::io::{
    prelude::{Read, Write},
    Result
};

fn main() {
    let magic_crypt = new_magic_crypt!("thisIsMyPasswordKeyItsSuperLong", 256);

    let accounts = deserialise(&magic_crypt).unwrap();

    let mut file = File::create("testing.txt").unwrap();

    for account in accounts.iter() {
        println!("{:#?}", account);
        writeln!(file, "{}", account.encrypt(&magic_crypt)).expect("Failed to write to file");
    }

    file.flush().unwrap();
}

fn deserialise(magic_crypt: &magic_crypt::MagicCrypt256) -> Result<Vec<Account>> {
    let mut file = File::open("testing.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut lines = contents.lines();

    let mut accounts: Vec<Account> = vec![];

    for _ in 0..(lines.clone().count()/3) {
        let mut username = Some(lines.next().expect("Should be safe to unwrap").to_string());
        if username.clone().unwrap().is_empty() {
            username = None;
        }
        let mut email = Some(lines.next().expect("Should be safe to unwrap").to_string());
        if email.clone().unwrap().is_empty() {
            email = None;
        }
        let password = lines.next().expect("Should be safe to unwrap").to_string();
        accounts.push(Account::new(&magic_crypt, username, email, password));
    }
    Ok(accounts)
}
