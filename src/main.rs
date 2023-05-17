use colored::Colorize;
use magic_crypt::new_magic_crypt;
use password_manager::serialisation::{
    deserialise, read_password_file, serialise, DeserialisationResult,
};

const PASSWORD_FILE: &str = "testing.txt";

fn main() {
    let (accounts, password, magic_crypt) = match password_manager::app::setup(PASSWORD_FILE) {
        Some(config) => config,
        None => return,
    };

    match password_manager::app::run(accounts) {
        Ok(accounts) => serialise(&magic_crypt, accounts, PASSWORD_FILE, password.trim())
            .expect("Failed to serialise data"),
        Err(e) => {
            eprintln!("{}", " ERROR ".bright_white().on_bright_red());
            eprintln!("{}", e);
        }
    }
}
