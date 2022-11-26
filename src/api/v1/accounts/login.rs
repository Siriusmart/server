use crate::api::v1::accounts::responses::AccountResponse;
use crate::global::structs::Account;
use actix_web::{get, web, Responder};

#[get("/login/{username_or_id}/{password}")]
async fn login(path: web::Path<(String, String)>) -> impl Responder {
    let (username_or_id, password) = path.into_inner();

    web::Json(match Account::login(username_or_id, password) {
        Ok(account) => {
            if account.verified {
                AccountResponse::Success {
                    username: account.username,
                    id: account.user_id,
                    email: account.email,
                }
            } else {
                AccountResponse::Error {
                    error: String::from("account not verified"),
                }
            }
        }
        Err(e) => AccountResponse::Error { error: e },
    })
}
