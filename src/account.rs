use magic_crypt::{MagicCrypt256, MagicCryptTrait};

#[derive(Debug)]
pub struct Account {
    name: String,
    username: Option<String>,
    email: Option<String>,
    password: String,
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
    pub fn builder() -> AccountBuilder {
        AccountBuilder::new()
    }

    /// Encrypts the Account into a string with the stored encrypter
    ///
    /// # Arguments
    /// * `encrypter` - The thing to decrypt with
    ///
    /// # Returns
    /// A string with newline breaks for each element
    pub fn encrypt(&self, encrypter: &MagicCrypt256) -> String {
        let encrypted_name = encrypter.encrypt_to_base64(&self.name);

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
            "{}\n{}\n{}\n{}",
            encrypted_name, encrypted_username, encrypted_email, encrypted_password
        )
    }
}

pub struct AccountBuilder {
    decrypter: Option<MagicCrypt256>,
    name: String,
    username: Option<String>,
    email: Option<String>,
    password: String,
}

impl AccountBuilder {
    pub fn new() -> Self {
        Self {
            decrypter: None,
            name: String::new(),
            username: None,
            email: None,
            password: String::new(),
        }
    }

    pub fn build(&mut self) -> Account {
        if let Some(decrypter) = &self.decrypter {
            if let Some(username) = &self.username {
                self.username = Some(
                    decrypter
                        .decrypt_base64_to_string(username)
                        .expect("Username failed to desrypt"),
                );
                if let Some(email) = &self.email {
                    self.email = Some(
                        decrypter
                            .decrypt_base64_to_string(email)
                            .expect("Username failed to desrypt"),
                    );
                }
                self.password = decrypter
                    .decrypt_base64_to_string(&self.password)
                    .expect("Username failed to desrypt");

                self.name = decrypter
                    .decrypt_base64_to_string(&self.name)
                    .expect("Username failed to desrypt");
            }
        }

        Account {
            name: self.name.clone(),
            username: self.username.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
        }
    }

    pub fn password<T: ToString>(&mut self, password: T) -> &mut Self {
        self.password = password.to_string();
        self
    }

    /// Changes the name of the Account
    ///
    /// # Example
    ///
    /// ```
    /// let account = AccountBuilder::new().name("Name").build();
    /// assert_eq!(account.name, "Name");
    /// ```
    pub fn name<T: ToString>(&mut self, name: T) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn email<T: ToString>(&mut self, email: T) -> &mut Self {
        self.email = Some(email.to_string());
        self
    }

    pub fn username<T: ToString>(&mut self, username: T) -> &mut Self {
        self.username = Some(username.to_string());
        self
    }

    pub fn decrypt(&mut self, decrypter: MagicCrypt256) -> &mut Self {
        self.decrypter = Some(decrypter);
        self
    }
}
