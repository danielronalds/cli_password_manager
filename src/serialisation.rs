use magic_crypt::MagicCrypt256;

use std::fs::File;
use std::io::{
    prelude::{Read, Write},
    Result,
};

use crate::account::Account;

/// Deserialises and decrypts the password file and returns a vector of Accounts
///
/// # Arguments
/// * `decrypter`     - The thing to decrypt with
/// * `password_file` - The name of the file containing the encrypted passwords
///
/// # Returns
/// A result either containing the vector of accounts or an IO error
pub fn deserialise(decrypter: &MagicCrypt256, password_file: &str) -> Result<Vec<Account>> {
    let mut file = File::open(password_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut lines = contents.lines();

    let mut accounts: Vec<Account> = vec![];

    for _ in 0..(lines.clone().count() / 3) {
        let mut username = Some(lines.next().expect("Should be safe to unwrap").to_string());
        if username.clone().unwrap().is_empty() {
            username = None;
        }
        let mut email = Some(lines.next().expect("Should be safe to unwrap").to_string());
        if email.clone().unwrap().is_empty() {
            email = None;
        }
        let password = lines.next().expect("Should be safe to unwrap").to_string();
        accounts.push(Account::new(Some(&decrypter), username, email, password));
    }
    Ok(accounts)
}

/// Serialises the given vector of Accounts in an encrypted format
///
/// # Arguments
/// * `encrypter`     - The thing to decrypt with
/// * `accounts`      - The accounts to serialise
/// * `password_file` - The name of the file to serialise to
pub fn serialise(
    encrypter: &MagicCrypt256,
    accounts: Vec<Account>,
    password_file: &str,
) -> Result<()> {
    let mut file = File::create(password_file)?;

    for account in accounts {
        writeln!(file, "{}", account.encrypt(&encrypter))?;
    }

    file.flush()?;

    Ok(())
}
