use crate::global::structs::Account;
use actix_web::{get, web, Responder};

#[get("/login/{username_or_id}/{password}")]
async fn login(path: web::Path<(String, String)>) -> impl Responder {
    let (username_or_id, password) = path.into_inner();

    match Account::login(username_or_id, password) {
        Ok(account) => {
            if account.verified {
                format!(
                    "Your login is valid:\n\nUsername: {}\nUser ID: {}\nEmail: {}",
                    account.username, account.user_id, account.email
                )
            } else {
                String::from("Cannot log in: account not verified")
            }
        }
        Err(e) => format!("Error: {e}"),
    }
}
