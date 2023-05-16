use colored::Colorize;
use magic_crypt::new_magic_crypt;
use password_manager::serialisation::{deserialise, serialise, DeserialisationResult, read_password_file};

const PASSWORD_FILE: &str = "testing.txt";

fn main() {
    let password_file = match read_password_file(PASSWORD_FILE) {
        Ok(file) => file,
        Err(_) => {
            eprintln!(
                "{} Password file not found!",
                " ERROR ".bright_white().on_red()
            );
            return;
        }
    };

    let password = password_manager::app::login().unwrap();
    let magic_crypt = new_magic_crypt!(password.trim(), 256);

    let accounts = match deserialise(&magic_crypt, password_file, password.trim()) {
        DeserialisationResult::WrongPassword => {
            eprintln!(
                "{} Thats the wrong password!",
                " WARNING ".black().on_yellow()
            );
            return;
        }
        DeserialisationResult::Ok(accounts) => accounts,
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
