use crate::global::structs::Account;
use actix_web::{get, web, Responder};

#[get("/rename/{username_or_id}/{password}/{new_username}")]
async fn rename(path: web::Path<(String, String, String)>) -> impl Responder {
    let (username_or_id, password, mut new_username) = path.into_inner();
    new_username = new_username.to_lowercase();

    let mut account = match Account::login(username_or_id, password) {
        Ok(account) => account,
        Err(e) => return format!("Error: {e}"),
    };

    if Account::exists_username(&new_username) {
        return String::from("Username has been taken, try using another username");
    }

    if let Err(e) = Account::delete_username(&account.username) {
        return format!("Error: {e}");
    }
    account.username = new_username;

    if let Err(e) = account.save_username() {
        return format!("Error: {e}");
    };
    if let Err(e) = account.save() {
        return format!("Error: {e}");
    }

    format!(
        "Account has been renamed:\n\nUsername: {}\nUser ID: {}\nEmail: {}",
        account.username, account.user_id, account.email
    )
}
