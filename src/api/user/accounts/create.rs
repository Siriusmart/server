use crate::global::structs::{Account, VerificationStore, VerificationType};
use actix_web::{get, web, Responder};
use std::error::Error;

#[get("/create/{username}/{password}/{email}")]
async fn create(path: web::Path<(String, String, String)>) -> impl Responder {
    let (mut username, password, mut email) = path.into_inner();

    for c in username.chars() {
        if !c.is_ascii_alphanumeric() && c != '_' {
            return String::from("Username contains illegal characters");
        }
    }

    username = username.to_lowercase();
    email = email.to_lowercase();

    if Account::exists_username(&username) {
        return String::from("Username taken");
    }

    if Account::exists_email(&email) {
        return String::from("Email is in use");
    }

    let account = Account::new(username.clone(), password, email.clone());

    let res = VerificationStore::new_expire_after(
        email.clone(),
        VerificationType::VerifyEmail {
            id: account.user_id.clone(),
            email,
        },
        900,
    )
    .await;

    if let Err(e) = res {
        return format!("Error sending verification email: {e}");
    }

    let res = (|| -> Result<(), Box<dyn Error>> {
        account.save()?;
        account.save_username()?;
        account.save_email()?;
        Ok(())
    })();

    if let Err(e) = res {
        return e.to_string();
    }

    format!("Account has been successfully created:\n\nUsername: {}\nUser ID: {}\nEmail: {}\n\nPlease verify your email address.", account.username, account.user_id, account.email)
}
