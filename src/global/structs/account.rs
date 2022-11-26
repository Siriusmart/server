use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    env,
    error::Error,
    fmt::Display,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use crate::global::functions::{cipher, decipher, sha384};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Account {
    pub password_hash: String,
    pub email: String,
    pub user_id: String,
    pub username: String,
    pub last_seen: u64,
    pub created: u64,
    pub verified: bool,
}

impl Account {
    pub fn password_hash(user_id: &str, mut password: String) -> String {
        password.push_str(user_id);
        password.push_str(env::var("PASSWORD_SALT").unwrap().as_str());
        hex::encode(sha384(&password))
    }

    pub fn userid_hash(id: &str) -> String {
        hex::encode(sha384(&format!(
            "{}{}",
            id,
            env::var("USERID_SALT").unwrap()
        )))
    }

    pub fn username_hash(username: &str) -> String {
        hex::encode(sha384(&format!(
            "{}{}",
            username,
            env::var("USERNAME_SALT").unwrap()
        )))
    }

    pub fn email_hash(email: &str) -> String {
        hex::encode(sha384(&format!(
            "{}{}",
            email,
            env::var("EMAIL_SALT").unwrap()
        )))
    }
}

impl Account {
    pub fn new(username: String, password: String, email: String) -> Self {
        let current_time = Utc::now().timestamp() as u64;
        let user_id = loop {
            let user_id = rand::thread_rng().gen_range(0..u64::MAX);
            if !Account::exists(user_id.to_string().as_str()) {
                break user_id.to_string();
            }
        };

        Self {
            password_hash: Self::password_hash(&user_id.to_string(), password),
            user_id,
            email,
            username,
            last_seen: current_time,
            created: current_time,
            verified: false,
        }
    }

    pub fn load(id: &str) -> Result<Self, Box<dyn Error>> {
        let hashed_id = Account::userid_hash(id);
        let path = PathBuf::from(format!("./storage/accounts/profiles/{}", &hashed_id));

        if !path.exists() {
            return Err(Box::new(AccountError::UserNotExist));
        }

        let decrypted_string = decipher(
            &fs::read_to_string(path)?,
            &env::var("ACCOUNTS_FILES_KEY").unwrap(),
            hashed_id.into_bytes(),
        )?;

        Ok(serde_json::from_str(&decrypted_string)?)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let hashed_id = Account::userid_hash(self.user_id.to_string().as_str());
        let path = PathBuf::from(format!("./storage/accounts/profiles/{}", &hashed_id));

        let serialized_string = serde_json::to_string(self)?;
        let encrypted_string = cipher(
            &serialized_string,
            &env::var("ACCOUNTS_FILES_KEY").unwrap(),
            hashed_id.into_bytes(),
        )?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        file.write_all(encrypted_string.as_bytes())?;
        Ok(())
    }

    pub fn save_username(&self) -> Result<(), Box<dyn Error>> {
        let hashed_username = Account::username_hash(&self.username);
        let path = PathBuf::from(format!("./storage/accounts/usernames/{}", &hashed_username));

        let encrypted_string = cipher(
            &self.user_id.to_string(),
            &env::var("USERNAME_FILES_KEY").unwrap(),
            hashed_username.into_bytes(),
        )?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        file.write_all(encrypted_string.as_bytes())?;
        Ok(())
    }

    pub fn save_email(&self) -> Result<(), Box<dyn Error>> {
        let hashed_email = Account::email_hash(&self.email);
        let path = PathBuf::from(format!("./storage/accounts/emails/{}", &hashed_email));

        let encrypted_string = cipher(
            &self.user_id.to_string(),
            &env::var("EMAIL_FILES_KEY").unwrap(),
            hashed_email.into_bytes(),
        )?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        file.write_all(encrypted_string.as_bytes())?;
        Ok(())
    }

    pub fn load_username(username: &str) -> Result<String, Box<dyn Error>> {
        let hashed_username = Account::username_hash(username);
        let path = PathBuf::from(format!("./storage/accounts/usernames/{}", &hashed_username));

        if !path.exists() {
            return Err(Box::new(AccountError::UserNotExist));
        }

        let decrypted_string = decipher(
            &fs::read_to_string(path)?,
            &env::var("USERNAME_FILES_KEY").unwrap(),
            hashed_username.into_bytes(),
        )?;
        Ok(decrypted_string)
    }

    pub fn load_email(email: &str) -> Result<String, Box<dyn Error>> {
        let hashed_email = Account::email_hash(email);
        let path = PathBuf::from(format!("./storage/accounts/emails/{}", &hashed_email));

        if !path.exists() {
            return Err(Box::new(AccountError::UserNotExist));
        }

        let decrypted_string = decipher(
            &fs::read_to_string(path)?,
            &env::var("EMAIL_FILES_KEY").unwrap(),
            hashed_email.into_bytes(),
        )?;
        Ok(decrypted_string)
    }

    pub fn delete(id: &str) -> Result<(), Box<dyn Error>> {
        let hashed_id = Account::userid_hash(id);
        let path = PathBuf::from(format!("./storage/accounts/profiles/{}", &hashed_id));

        fs::remove_file(path)?;
        Ok(())
    }

    pub fn delete_username(username: &str) -> Result<(), Box<dyn Error>> {
        let hashed_username = Account::username_hash(username);
        let path = PathBuf::from(format!("./storage/accounts/usernames/{}", &hashed_username));

        fs::remove_file(path)?;
        Ok(())
    }

    pub fn delete_email(email: &str) -> Result<(), Box<dyn Error>> {
        let hashed_email = Account::email_hash(email);
        let path = PathBuf::from(format!("./storage/accounts/emails/{}", &hashed_email));

        fs::remove_file(path)?;
        Ok(())
    }

    pub fn exists(id: &str) -> bool {
        let hashed_id = Account::userid_hash(id);
        let path = PathBuf::from(format!("./storage/accounts/profiles/{}", &hashed_id));

        path.exists()
    }

    pub fn exists_username(username: &str) -> bool {
        let hashed_username = hex::encode(sha384(&format!(
            "{}{}",
            username,
            env::var("USERNAME_SALT").unwrap()
        )));
        let path = PathBuf::from(format!("./storage/accounts/usernames/{}", &hashed_username));

        path.exists()
    }

    pub fn exists_email(email: &str) -> bool {
        let hashed_email = hex::encode(sha384(&format!(
            "{}{}",
            email,
            env::var("EMAIL_SALT").unwrap()
        )));
        let path = PathBuf::from(format!("./storage/accounts/emails/{}", &hashed_email));

        path.exists()
    }

    pub fn login(mut identifier: String, password: String) -> Result<Account, String> {
        identifier = identifier.to_lowercase();

        let id = if Account::exists(&identifier) {
            identifier
        } else if Account::exists_username(&identifier) {
            if let Ok(id) = Account::load_username(&identifier) {
                id
            } else {
                return Err(String::from("error getting user id from username"));
            }
        } else if Account::exists_email(&identifier) {
            if let Ok(id) = Account::load_email(&identifier) {
                id
            } else {
                return Err(String::from("error getting user id from email"));
            }
        } else {
            return Err(String::from("no such user with this username or id"));
        };

        let account = match Account::load(&id) {
            Ok(account) => account,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let hashed_password = Account::password_hash(&id, password);

        if hashed_password != account.password_hash {
            return Err(String::from("incorrect password"));
        }

        Ok(account)
    }

    pub fn bump(&mut self) {
        self.last_seen = Utc::now().timestamp() as u64;
    }
}

#[derive(Debug)]
pub enum AccountError {
    UserNotExist,
}

impl Display for AccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::UserNotExist => "user does not exist",
        })
    }
}

impl Error for AccountError {}
