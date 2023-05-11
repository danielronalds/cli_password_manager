use colored::Colorize;
use magic_crypt::new_magic_crypt;
use password_manager::serialisation::{deserialise, serialise};

const PASSWORD_FILE: &str = "testing.txt";
const FILE_KEY: &str = "password";

fn main() {
    let magic_crypt = new_magic_crypt!("thisIsMyPasswordKeyItsSuperLong", 256);

    let accounts =
        deserialise(&magic_crypt, PASSWORD_FILE, FILE_KEY).expect("Failed to deserialise data");

    let mut app = password_manager::app::PasswordManagerApp::new(accounts);
    match app.run() {
        Ok(accounts) => serialise(&magic_crypt, accounts, PASSWORD_FILE, FILE_KEY)
            .expect("Failed to serialise data"),
        Err(e) => {
            eprintln!("{}", " ERROR ".bright_white().on_bright_red());
            eprintln!("{}", e);
        }
    }
}
