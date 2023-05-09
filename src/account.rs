use magic_crypt::{MagicCrypt256, MagicCryptTrait};

#[derive(Debug)]
/// A struct that stores the details of an account stored in the password manager
pub struct Account {
    label: String,
    username: Option<String>,
    email: Option<String>,
    password: String,
}

impl Account {
    /// Returns an `AccountBuilder` to build an Account
    pub fn builder() -> AccountBuilder {
        AccountBuilder::new()
    }

    /// Returns a clone of the Account's label
    pub fn label(&self) -> String {
        self.label.clone()
    }

    /// Returns a copy of the Account's email
    ///
    /// # Returns
    ///
    /// `None` if there is no email attached to the account, otherwise a clone of the accounts
    /// email
    pub fn email(&self) -> Option<String> {
        self.email.clone()
    }

    /// Returns a copy of the Account's username
    ///
    /// # Returns
    ///
    /// `None` if there is no username attached to the account, otherwise a clone of the accounts
    /// username
    pub fn username(&self) -> Option<String> {
        self.username.clone()
    }

    /// Returns a clone of the password attached to the account
    pub fn password(&self) -> String {
        self.password.clone()
    }

    /// Encrypts the Account into a string with the stored encrypter
    ///
    /// # Arguments
    ///
    /// * `encrypter` - The thing to decrypt with
    ///
    /// # Returns
    ///
    /// A string with newline breaks for each element
    pub fn encrypt(&self, encrypter: &MagicCrypt256) -> String {
        let encrypted_label = encrypter.encrypt_to_base64(&self.label);

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
            encrypted_label, encrypted_username, encrypted_email, encrypted_password
        )
    }
}

pub struct AccountBuilder {
    decrypter: Option<MagicCrypt256>,
    label: String,
    username: Option<String>,
    email: Option<String>,
    password: String,
}

impl AccountBuilder {
    /// Creates a new AccountBuilder
    ///
    /// # Defaults
    ///
    /// * `decrypter` - None
    /// * `label` - Empty String
    /// * `username` - None
    /// * `email` - None
    /// * `password` - Empty String
    pub fn new() -> Self {
        Self {
            decrypter: None,
            label: String::new(),
            username: None,
            email: None,
            password: String::new(),
        }
    }

    /// Builds the Account with the data of the AccountBuilder
    ///
    /// # Example
    ///
    /// ```
    /// # use password_manager::account::AccountBuilder;
    ///
    /// let account = AccountBuilder::new()
    ///     .label("crates.io")
    ///     .username("sir_devs_a_lot")
    ///     .email("sirdevsalot@gmail.com")
    ///     .password("sirDevsALotIsNumber1")
    ///     .build();
    ///
    /// assert_eq!(account.label(), "crates.io".to_string());
    /// assert_eq!(account.username(), Some("sir_devs_a_lot".to_string()));
    /// assert_eq!(account.email(), Some("sirdevsalot@gmail.com".to_string()));
    /// assert_eq!(account.password(), "sirDevsALotIsNumber1".to_string());
    /// ```
    pub fn build(&mut self) -> Account {
        if let Some(decrypter) = &self.decrypter {
            if let Some(username) = &self.username {
                self.username = Some(
                    decrypter
                        .decrypt_base64_to_string(username)
                        .expect("Username failed to desrypt"),
                );
            }
            if let Some(email) = &self.email {
                self.email = Some(
                    decrypter
                        .decrypt_base64_to_string(email)
                        .expect("Email failed to desrypt"),
                );
            }
            if !self.password.is_empty() {
                self.password = decrypter
                    .decrypt_base64_to_string(&self.password)
                    .expect("Password failed to decrypt");
            }

            if !self.label.is_empty() {
                self.label = decrypter
                    .decrypt_base64_to_string(&self.label)
                    .expect("Label failed to desrypt");
            }
        }

        Account {
            label: self.label.clone(),
            username: self.username.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
        }
    }

    /// Changes the password of the Account
    ///
    /// # Arguments
    ///
    /// * `password` - What the password of the Account should be
    ///
    /// # Example
    ///
    /// ```
    /// # use password_manager::account::AccountBuilder;
    /// let account = AccountBuilder::new().password("p@ssw0rd").build();
    /// assert_eq!(account.password(), "p@ssw0rd".to_string());
    /// ```
    pub fn password<T: ToString>(&mut self, password: T) -> &mut Self {
        self.password = password.to_string();
        self
    }

    /// Changes the label of the Account
    ///
    /// # Arguments
    ///
    /// * `label` - What the label of the Account should be
    ///
    /// # Example
    ///
    /// ```
    /// # use password_manager::account::AccountBuilder;
    /// let account = AccountBuilder::new().label("Name").build();
    /// assert_eq!(account.label(), "Name".to_string());
    /// ```
    pub fn label<T: ToString>(&mut self, label: T) -> &mut Self {
        self.label = label.to_string();
        self
    }

    /// Changes the email of the Account
    ///
    /// # Arguments
    ///
    /// * `email` - What the email of the Account should be
    ///
    /// # Example
    ///
    /// ```
    /// # use password_manager::account::AccountBuilder;
    /// let account = AccountBuilder::new().email("example@example.com").build();
    /// assert_eq!(account.email(), Some("example@example.com".to_string()));
    /// ```
    pub fn email<T: ToString>(&mut self, email: T) -> &mut Self {
        self.email = Some(email.to_string());
        self
    }

    /// Changes the username of the Account
    ///
    /// # Arguments
    ///
    /// * `username` - What the username of the Account should be
    ///
    /// # Example
    ///
    /// ```
    /// # use password_manager::account::AccountBuilder;
    /// let account = AccountBuilder::new().username("cool_username").build();
    /// assert_eq!(account.username(), Some("cool_username".to_string()));
    /// ```
    pub fn username<T: ToString>(&mut self, username: T) -> &mut Self {
        self.username = Some(username.to_string());
        self
    }

    /// Whether the account should decrypt the given information.
    ///
    /// # Arguments
    ///
    /// * `decrypter` - The decrypter to use when decrypting the information
    ///
    /// # Example
    ///
    /// ```
    /// # use password_manager::account::AccountBuilder;
    /// # use magic_crypt::{new_magic_crypt, MagicCryptTrait};
    ///
    /// // Creating the decrypter with magic-crypt
    /// let decrypter = new_magic_crypt!("thisIsMyDecryptionKey", 256);
    ///
    /// // Encrypting the username
    /// let username = decrypter.encrypt_to_base64("cool_username");
    /// assert_ne!(username, "cool_username");
    ///
    /// // Building the account with the encrypted username and checking if it was decrypted
    /// // successfully
    /// let account = AccountBuilder::new().username(username).decrypt(decrypter).build();
    /// assert_eq!(account.username(), Some("cool_username".to_string()));
    /// ```
    pub fn decrypt(&mut self, decrypter: MagicCrypt256) -> &mut Self {
        self.decrypter = Some(decrypter);
        self
    }
}
