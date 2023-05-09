//! This module contains the code for reading and writing the stored passwords in the manager to
//! the user's local storage.
//!
//! # File Format
//!
//! 1. file_password
//! 2. account_one_label
//! 3. account_one_username
//! 4. account_one_email
//! 5. account_one_password
//! 6. account_two_label
//! 7. account_two_username
//! 8. account_two_email
//! 9. account_two_password
//!
//! etc...
//!
//! Accounts that do not have an email or a username have blank lines instead

use magic_crypt::{MagicCrypt256, MagicCryptTrait};

use std::fs::File;
use std::io::{
    prelude::{Read, Write},
    Result,
};

use crate::account::Account;

/// Deserialises and decrypts the password file and returns a vector of Accounts
///
/// # Arguments
///
/// * `decrypter`     - The thing to decrypt with
/// * `password_file` - The name of the file containing the encrypted passwords
/// * `password`      - The password that the user has entered to login
///
/// # Returns
///
/// A result either containing the vector of accounts or an IO error
pub fn deserialise(
    decrypter: &MagicCrypt256,
    password_file: &str,
    password: &str,
) -> Result<Vec<Account>> {
    let mut file = File::open(password_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut lines = contents.lines();

    let mut accounts: Vec<Account> = vec![];

    let passkey = decrypter
        .decrypt_base64_to_string(lines.next().expect("Should be safe to unwrap"))
        .unwrap();

    if password != passkey {
        println!("{}", passkey);
        panic!("WRONG PASSWORD!");
    }

    for _ in 0..(lines.clone().count() / 4) {
        let mut account_builder = Account::builder();

        account_builder.label(lines.next().expect("Should be safe to unwrap"));

        let username = lines.next().expect("Should be safe to unwrap");
        if !username.is_empty() {
            account_builder.username(username);
        }

        let email = lines.next().expect("Should be safe to unwrap");
        if !email.is_empty() {
            account_builder.email(email);
        }

        account_builder.password(lines.next().expect("Should be safe to unwrap"));

        accounts.push(account_builder.decrypt(decrypter.clone()).build());
    }
    Ok(accounts)
}

/// Serialises the given vector of Accounts in an encrypted format
///
/// # Arguments
///
/// * `encrypter`     - The thing to decrypt with
/// * `accounts`      - The accounts to serialise
/// * `password_file` - The name of the file to serialise to
/// * `password`      - The user's password to verify against
pub fn serialise(
    encrypter: &MagicCrypt256,
    accounts: Vec<Account>,
    password_file: &str,
    password: &str,
) -> Result<()> {
    let mut file = File::create(password_file)?;

    writeln!(file, "{}", encrypter.encrypt_str_to_base64(password))
        .expect("Failed to write passkey");

    for account in accounts {
        writeln!(file, "{}", account.encrypt(&encrypter))?;
    }

    file.flush()?;

    Ok(())
}
