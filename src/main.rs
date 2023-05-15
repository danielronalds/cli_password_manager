use colored::Colorize;
use magic_crypt::new_magic_crypt;
use password_manager::serialisation::{deserialise, serialise, DeserialisationResult};
use std::io::Write;

const PASSWORD_FILE: &str = "testing.txt";

fn main() {
    // Get the password from the user
    let mut password = String::new();

    print!("{} ", " Enter Password ".black().on_bright_white());
    std::io::stdout().flush().expect("Failed to flush");

    if let Err(_) = std::io::stdin().read_line(&mut password) {
        eprintln!("{}", " ERROR ".bright_white().on_bright_red());
        eprintln!("Unable to read password!");
        return;
    }

    let magic_crypt = new_magic_crypt!(password.trim(), 256);

    let accounts = match deserialise(&magic_crypt, PASSWORD_FILE, password.trim()) {
        DeserialisationResult::NoFileFound => {
            eprintln!(
                "{} Password file not found!",
                " ERROR ".bright_white().on_red()
            );
            return;
        }
        DeserialisationResult::FailedToRead => {
            eprintln!(
                "{} Failed to read the file!",
                " ERROR ".bright_white().on_red()
            );
            return;
        }
        DeserialisationResult::WrongPassword => {
            eprintln!(
                "{} Thats the wrong password!",
                " ERROR ".bright_white().on_red()
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
