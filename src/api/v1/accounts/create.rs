use crate::api::v1::accounts::responses::AccountResponse;
use crate::global::structs::{Account, VerificationStore, VerificationType};
use actix_web::{get, web, Responder};
use std::error::Error;

#[get("/create/{username}/{password}/{email}")]
async fn create(path: web::Path<(String, String, String)>) -> impl Responder {
    let (mut username, password, mut email) = path.into_inner();

    for c in username.chars() {
        if !c.is_ascii_alphanumeric() && c != '_' {
            return web::Json(AccountResponse::Error {
                error: String::from("username contains illegal characters"),
            });
        }
    }

    username = username.to_lowercase();
    email = email.to_lowercase();

    if Account::exists_username(&username) {
        return web::Json(AccountResponse::Error {
            error: String::from("username taken"),
        });
    }

    if Account::exists_email(&email) {
        return web::Json(AccountResponse::Error {
            error: String::from("email is in use"),
        });
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
        return web::Json(AccountResponse::Error {
            error: e.to_string(),
        });
    }

    let res = (|| -> Result<(), Box<dyn Error>> {
        account.save()?;
        account.save_username()?;
        account.save_email()?;
        Ok(())
    })();

    if let Err(e) = res {
        return web::Json(AccountResponse::Error {
            error: e.to_string(),
        });
    }

    web::Json(AccountResponse::Success {
        username,
        id: account.user_id,
        email: account.email,
    })
}
