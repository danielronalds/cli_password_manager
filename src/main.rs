mod account;

use account::Account;
use magic_crypt::new_magic_crypt;

use std::fs::File;
use std::io::prelude::{Read, Write};

fn main() {
    let magic_crypt = new_magic_crypt!("thisIsMyPasswordKeyItsSuperLong", 256);

    let mut file = File::open("testing.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lines = contents.lines();

    let mut accounts: Vec<Account> = vec![];

    for _ in 0..(lines.clone().count()/3) {
        let username = lines.next().expect("Should be safe to unwrap").to_string();
        let email = lines.next().expect("Should be safe to unwrap").to_string();
        let password = lines.next().expect("Should be safe to unwrap").to_string();
        accounts.push(Account::new(&magic_crypt, Some(username), Some(email), password));
    }

    let mut file = File::create("testing.txt").unwrap();

    for account in accounts.iter() {
        println!("{:#?}", account);
        writeln!(file, "{}", account.encrypt(&magic_crypt)).expect("Failed to write to file");
    }

    file.flush().unwrap();
}
