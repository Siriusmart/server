use crate::global::structs::{
    verification_code_hash, Account, VerificationStore, VerificationType,
};
use actix_web::{get, web, Responder};
use log::warn;

#[get("/verify/{code}")]
async fn verify(path: web::Path<String>) -> impl Responder {
    let code = path.into_inner();

    let r#type = match VerificationStore::load(&code) {
        Ok(store) => store.r#type,
        Err(e) => return format!("Cannot find verification code: {e}"),
    };

    match r#type {
        VerificationType::VerifyEmail { id, email } => {
            let mut account = match Account::load(&id) {
                Ok(account) => account,
                Err(e) => return format!("Cannot load account: {e}"),
            };

            if account.email != email {
                return String::from("Cannot verify: email has been changed to something else");
            }

            account.verified = true;

            if let Err(e) = account.save() {
                return format!("Error: {e}");
            }

            if VerificationStore::delete(&verification_code_hash(&code)).is_err() {
                warn!("Error deleting verification store for code `{code}`");
            }

            format!("Your email has been verified:\n\nUsername: {}\nUser ID: {}\nEmail: {}\n\nYou may now close this window.", account.username, account.user_id, account.email)
        }
    }
}
