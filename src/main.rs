use colored::Colorize;
use password_manager::serialisation::serialise;

const PASSWORD_FILE: &str = "testing.txt";

fn main() {
    let (accounts, password) = match password_manager::app::setup(PASSWORD_FILE) {
        Some(config) => config,
        None => return,
    };

    match password_manager::app::run(accounts, password) {
        Ok((accounts, password)) => {
            serialise(accounts, PASSWORD_FILE, password.trim()).expect("Failed to serialise data")
        }
        Err(e) => {
            eprintln!("{}", " ERROR ".bright_white().on_bright_red());
            eprintln!("{}", e);
        }
    }
}
