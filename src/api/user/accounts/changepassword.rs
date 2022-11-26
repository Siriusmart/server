use crate::global::structs::Account;
use actix_web::{get, web, Responder};

#[get("/changepassword/{username_or_id}/{old_password}/{new_password}")]
async fn changepassword(path: web::Path<(String, String, String)>) -> impl Responder {
    let (username_or_id, old_password, new_password) = path.into_inner();

    let mut account = match Account::login(username_or_id, old_password) {
        Ok(account) => account,
        Err(e) => return format!("Error: {e}"),
    };

    let new_hashed_password = Account::password_hash(&account.user_id, new_password);
    account.password_hash = new_hashed_password;
    if let Err(e) = account.save() {
        return format!("Error: {e}");
    }

    format!(
        "Password has been successfully changed:\n\nUsername: {}\nUser ID: {}\nEmail: {}",
        account.username, account.user_id, account.email
    )
}
