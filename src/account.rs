use magic_crypt::{MagicCrypt256, MagicCryptTrait};

#[derive(Debug)]
pub struct Account {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

impl Account {
    /// Creates a new Account
    ///
    /// # Arguments
    /// * `decrypter` - The thing to decrypt with. If it is not provided, then the account
    ///                 information is presumed to not be encrypted
    /// * `username`  - The encrypted username. Option as the user doesn't have to attach one
    /// * `email`     - The encrypted email. Option as the user doesn't have to attach one
    /// * `password`  - The email attached to the account, option as the user might not enter it
    pub fn new<T: ToString>(
        decrypter: Option<&MagicCrypt256>,
        username: Option<T>,
        email: Option<T>,
        password: T,
    ) -> Self {
        let username = match username {
            Some(encrypted_username) => Some(match decrypter {
                Some(decrypter) => decrypter
                    .decrypt_base64_to_string(encrypted_username.to_string())
                    .expect("Username failed to desrypt"),
                None => encrypted_username.to_string(),
            }),
            None => None,
        };

        let email = match email {
            Some(encrypted_email) => Some(match decrypter {
                Some(decrypter) => decrypter
                    .decrypt_base64_to_string(encrypted_email.to_string())
                    .expect("Email failed to desrypt"),
                None => encrypted_email.to_string(),
            }),
            None => None,
        };

        let password = match decrypter {
            Some(decrypter) => decrypter
                .decrypt_base64_to_string(password.to_string())
                .expect("Password failed to decrypt"),
            None => password.to_string(),
        };

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

        format!(
            "{}\n{}\n{}",
            encrypted_username, encrypted_email, encrypted_password
        )
    }
}
