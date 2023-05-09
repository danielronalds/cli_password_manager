mod account;
mod serialisation;

use magic_crypt::new_magic_crypt;
use serialisation::{deserialise, serialise};

const PASSWORD_FILE: &str = "testing.txt";

fn main() {
    let magic_crypt = new_magic_crypt!("thisIsMyPasswordKeyItsSuperLong", 256);

    let accounts = deserialise(&magic_crypt, PASSWORD_FILE).expect("Failed to deserialise data");

    println!("{:#?}", accounts);

    serialise(&magic_crypt, accounts, PASSWORD_FILE).expect("Failed to serialise data");
}
