use crate::global::structs::Account;
use actix_web::{get, web, Responder};
use log::warn;
use crate::api::v1::accounts::responses::AccountResponse;

#[get("/delete/{username_or_id}/{password}")]
async fn delete(path: web::Path<(String, String)>) -> impl Responder {
    let (username_or_id, password) = path.into_inner();

    let account = match Account::login(username_or_id, password) {
        Ok(account) => account,
        Err(e) => return web::Json(AccountResponse::Error { error: e }),
    };

    if let Err(e) = Account::delete(account.user_id.as_str()) {
        warn!("Error deleting user profile file: {e}")
    }

    if let Err(e) = Account::delete_username(&account.username) {
        warn!("Error deleting username file: {e}")
    }

    web::Json(AccountResponse::Success {
        username: account.username,
        id: account.user_id,
    })
}
