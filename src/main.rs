mod account;
mod serialisation;

use serialisation::{deserialise, serialise};
use magic_crypt::new_magic_crypt;

const PASSWORD_FILE: &str = "testing.txt";

fn main() {
    let magic_crypt = new_magic_crypt!("thisIsMyPasswordKeyItsSuperLong", 256);

    let accounts = deserialise(&magic_crypt, PASSWORD_FILE).expect("Failed to deserialise data");

    accounts.iter().for_each(|x| println!("{:#?}", x));

    serialise(&magic_crypt, accounts, PASSWORD_FILE).expect("Failed to serialise data");
}
