use crate::global::structs::Account;
use actix_web::{get, web, Responder};
use crate::api::v1::accounts::responses::AccountResponse;
use std::error::Error;

#[get("/create/{username}/{password}")]
async fn create(path: web::Path<(String, String)>) -> impl Responder {
    let (mut username, password) = path.into_inner();
    username = username.to_lowercase();

    if Account::exists_username(&username) {
        return web::Json(AccountResponse::Error {
            error: String::from("username taken"),
        });
    }

    let account = Account::new(username.clone(), password);
    let res = (|| -> Result<(), Box<dyn Error>> {
        account.save()?;
        account.save_username()?;
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
    })
}
