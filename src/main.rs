use magic_crypt::new_magic_crypt;
use password_manager::serialisation::{deserialise, serialise};

const PASSWORD_FILE: &str = "testing.txt";
const FILE_KEY: &str = "password";

fn main() {
    let magic_crypt = new_magic_crypt!("thisIsMyPasswordKeyItsSuperLong", 256);

    let accounts =
        deserialise(&magic_crypt, PASSWORD_FILE, FILE_KEY).expect("Failed to deserialise data");

    let mut app = password_manager::app::PasswordManagerApp::new(accounts);
    app.run();

    //serialise(&magic_crypt, accounts, PASSWORD_FILE, FILE_KEY).expect("Failed to serialise data");
}
