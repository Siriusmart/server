use chrono::Utc;
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    env,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use crate::global::functions::{cipher, decipher, sha384};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationStore {
    pub r#type: VerificationType,
    pub expire: u64,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationType {
    VerifyEmail { id: String, email: String },
}

impl VerificationStore {
    pub async fn new(
        email: String,
        r#type: VerificationType,
        expire: u64,
    ) -> Result<(), Box<dyn Error>> {
        let code = rand::thread_rng().gen_range(0..u64::MAX).to_string();

        let email_username = env::var("EMAIL_USERNAME").unwrap();
        let email_password = env::var("EMAIL_PASSWORD").unwrap();

        let email = Message::builder()
            .from(email_username.parse()?)
            .to(email.parse()?)
            .subject("Your Verification Link (Ignore if it's not yours)")
            .body(match &r#type {
                VerificationType::VerifyEmail { id, .. } => {
                    format!("{}/api/user/accounts/verify/{code}\n\nReason: Account creation\nDouble check your account ID is {id}", env::var("SELF_ADDRESS").unwrap())
                }
            })?;

        let creds = Credentials::new(email_username, email_password);

        let smtp_relay = env::var("SMTP_RELAY").unwrap();

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_relay)?
            .credentials(creds)
            .build();

        mailer.send(email).await?;

        let store = Self { r#type, expire };

        store.save(&verification_code_hash(&code))?;
        Ok(())
    }

    pub async fn new_expire_after(
        email: String,
        r#type: VerificationType,
        expire_after: u64,
    ) -> Result<(), Box<dyn Error>> {
        Self::new(email, r#type, Utc::now().timestamp() as u64 + expire_after).await
    }

    pub fn save(&self, hashed_code: &str) -> Result<(), Box<dyn Error>> {
        let serialized_string = serde_json::to_string(self)?;
        let path = PathBuf::from(format!("./storage/verifications/{}", hashed_code));

        let encrypted_string = cipher(
            &serialized_string,
            &env::var("VERIFICATION_FILES_KEY").unwrap(),
            format!(
                "{}{}",
                hashed_code,
                &env::var("VERIFICATION_NOUNCE_SALT").unwrap()
            )
            .as_bytes()
            .to_vec(),
        )?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        file.write_all(encrypted_string.as_bytes())?;

        Ok(())
    }

    pub fn load(code: &str) -> Result<Self, Box<dyn Error>> {
        let hashed_code = verification_code_hash(code);

        let ciphered_string =
            fs::read_to_string(format!("./storage/verifications/{}", hashed_code))?;
        let decrypted_string = decipher(
            &ciphered_string,
            &env::var("VERIFICATION_FILES_KEY").unwrap(),
            format!(
                "{}{}",
                hashed_code,
                &env::var("VERIFICATION_NOUNCE_SALT").unwrap()
            )
            .as_bytes()
            .to_vec(),
        )?;
        Ok(serde_json::from_str(&decrypted_string)?)
    }

    pub fn delete(hashed_code: &str) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(format!("./storage/verifications/{}", hashed_code));
        fs::remove_file(path)?;
        Ok(())
    }

    pub fn exists(hashed_code: &str) -> bool {
        let path = PathBuf::from(format!("./storage/verifications/{}", hashed_code));
        path.exists()
    }
}

pub fn verification_code_hash(code: &str) -> String {
    hex::encode(sha384(&format!(
        "{}{}",
        code,
        env::var("VERIFICATION_CODE_SALT").unwrap()
    )))
}
