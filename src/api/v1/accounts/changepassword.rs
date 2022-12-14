use crate::api::v1::accounts::responses::AccountResponse;
use crate::global::structs::Account;
use actix_web::{get, web, Responder};

#[get("/changepassword/{username_or_id}/{old_password}/{new_password}")]
async fn changepassword(path: web::Path<(String, String, String)>) -> impl Responder {
    let (username_or_id, old_password, new_password) = path.into_inner();

    let mut account = match Account::login(username_or_id, old_password) {
        Ok(account) => account,
        Err(e) => return web::Json(AccountResponse::Error { error: e }),
    };

    let new_hashed_password = Account::password_hash(&account.user_id, new_password);
    account.password_hash = new_hashed_password;
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
