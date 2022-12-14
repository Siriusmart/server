use crate::api::v1::accounts::responses::AccountResponse;
use crate::global::structs::Account;
use actix_web::{get, web, Responder};

#[get("/rename/{username_or_id}/{password}/{new_username}")]
async fn rename(path: web::Path<(String, String, String)>) -> impl Responder {
    let (username_or_id, password, mut new_username) = path.into_inner();
    new_username = new_username.to_lowercase();

    let mut account = match Account::login(username_or_id, password) {
        Ok(account) => account,
        Err(e) => return web::Json(AccountResponse::Error { error: e }),
    };

    if Account::exists_username(&new_username) {
        return web::Json(AccountResponse::Error {
            error: String::from("username taken"),
        });
    }

    if let Err(e) = Account::delete_username(&account.username) {
        return web::Json(AccountResponse::Error {
            error: e.to_string(),
        });
    }
    account.username = new_username;

    if let Err(e) = account.save_username() {
        return web::Json(AccountResponse::Error {
            error: e.to_string(),
        });
    };
    if let Err(e) = account.save() {
        return web::Json(AccountResponse::Error {
            error: e.to_string(),
        });
    }

    web::Json(AccountResponse::Success {
        username: account.username,
        id: account.user_id,
        email: account.email,
    })
}
