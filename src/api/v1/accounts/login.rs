use crate::global::structs::Account;
use actix_web::{get, web, Responder};
use crate::api::v1::accounts::responses::AccountResponse;

#[get("/login/{username_or_id}/{password}")]
async fn login(path: web::Path<(String, String)>) -> impl Responder {
    let (username_or_id, password) = path.into_inner();

    web::Json(match Account::login(username_or_id, password) {
        Ok(account) => AccountResponse::Success {
            username: account.username,
            id: account.user_id,
        },
        Err(e) => AccountResponse::Error { error: e },
    })
}
