use crate::global::structs::{Account, VerificationType, VerificationStore};
use actix_web::{get, web, Responder};

#[get("/changeemail/{identifier}/{passowrd}/{new_email}")]
async fn changeemail(path: web::Path<(String, String, String)>) -> impl Responder {
    let (username_or_id, password, mut new_email) = path.into_inner();
    new_email = new_email.to_lowercase();

    let mut account = match Account::login(username_or_id, password) {
        Ok(account) => account,
        Err(e) => return format!("Error logging in: {e}"),
    };

    if Account::exists_email(&new_email) {
        return format!("Email is taken, try using another email address");
    }

    let res = VerificationStore::new_expire_after(
        new_email.clone(),
        VerificationType::VerifyEmail {
            id: account.user_id.clone(),
            email: new_email.clone(),
        },
        900,
    )
    .await;

    if let Err(e) = res {
        return format!("Error sending verification email: {e}");
    }

    account.email = new_email;
    account.verified = false;

    if let Err(e) = account.save() {
        return e.to_string();
    }

    format!("Email has been changed:\n\nUsername: {}\nUser ID: {}\nEmail: {}\n\nPlease verify your email address.", account.username, account.user_id, account.email)
}
