use magic_crypt::{MagicCrypt256, MagicCryptTrait};

#[derive(Debug)]
pub struct Account {
   pub email: Option<String>,
   pub username: Option<String>,
   pub password: String,
}

impl Account {
    /// Creates a new Account
    ///
    /// # Arguments
    /// * `decrypter` - The thing to decrypt with
    /// * `username`  - The encrypted username. Option as the user doesn't have to attach one
    /// * `email`     - The encrypted email. Option as the user doesn't have to attach one
    /// * `password`  - The email attached to the account, option as the user might not enter it
    pub fn new(
        decrypter: &MagicCrypt256,
        username: Option<String>,
        email: Option<String>,
        password: String,
    ) -> Self {
        let username = match username {
            Some(encrypted_username) => Some(
                decrypter
                    .decrypt_base64_to_string(encrypted_username)
                    .expect("Username failed to desrypt"),
            ),
            None => None,
        };

        let email = match email {
            Some(encrypted_email) => Some(
                decrypter
                    .decrypt_base64_to_string(encrypted_email)
                    .expect("Email failed to desrypt"),
            ),
            None => None,
        };

        let password = decrypter
            .decrypt_base64_to_string(password)
            .expect("Password failed to decrypt");

        Self {
            email,
            username,
            password,
        }
    }

    /// Encrypts the Account into a string with the stored encrypter
    ///
    /// # Arguments
    /// * `encrypter` - The thing to decrypt with
    ///
    /// # Returns
    /// A string with newline breaks for each element
    pub fn encrypt(&self, encrypter: &MagicCrypt256) -> String {
        let encrypted_email = match &self.email {
            Some(email) => encrypter.encrypt_to_base64(&email),
            None => "".to_string(),
        };

        let encrypted_username = match &self.username {
            Some(username) => encrypter.encrypt_to_base64(&username),
            None => "".to_string(),
        };

        let encrypted_password = encrypter.encrypt_to_base64(&self.password);

        format!("{}\n{}\n{}", encrypted_username, encrypted_email, encrypted_password)
    }
}
