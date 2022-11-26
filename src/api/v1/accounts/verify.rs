use crate::{
    api::v1::accounts::AccountResponse,
    global::structs::{verification_code_hash, Account, VerificationStore, VerificationType},
};
use actix_web::{get, web, Responder};
use log::warn;

#[get("/verify/{code}")]
async fn verify(path: web::Path<String>) -> impl Responder {
    let code = path.into_inner();

    let store = match VerificationStore::load(&code) {
        Ok(store) => store,
        Err(e) => {
            return web::Json(AccountResponse::Error {
                error: format!("cannot find verification code {e}"),
            })
        }
    };

    match store.r#type {
        VerificationType::VerifyEmail { id, email } => {
            let mut account = match Account::load(&id) {
                Ok(account) => account,
                Err(e) => {
                    return web::Json(AccountResponse::Error {
                        error: format!("cannot load account {e}"),
                    })
                }
            };

            if account.email != email {
                return web::Json(AccountResponse::Error {
                    error: String::from("email mismatch"),
                });
            }

            account.verified = true;

            if let Err(e) = account.save() {
                return web::Json(AccountResponse::Error {
                    error: e.to_string(),
                });
            }

            if VerificationStore::delete(&verification_code_hash(&code)).is_err() {
                warn!("Error deleting verification store for code `{code}`");
            }

            web::Json(AccountResponse::Success {
                username: account.username,
                id: account.user_id,
                email: account.email,
            })
        }
    }
}
