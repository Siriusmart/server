use crate::{api::v1::accounts::responses::AccountResponse, global::structs::VerificationStore};
use crate::global::structs::{Account, VerificationType};
use actix_web::{get, web, Responder};

#[get("/changeemail/{identifier}/{passowrd}/{new_email}")]
async fn changeemail(path: web::Path<(String, String, String)>) -> impl Responder {
    let (username_or_id, password, mut new_email) = path.into_inner();
    new_email = new_email.to_lowercase();

    let mut account = match Account::login(username_or_id, password) {
        Ok(account) => account,
        Err(e) => return web::Json(AccountResponse::Error { error: e }),
    };

    if Account::exists_email(&new_email) {
        return web::Json(AccountResponse::Error { error: String::from("email in use") });
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
        return web::Json(AccountResponse::Error { error: e.to_string() });
    }

    account.email = new_email;
    account.verified = false;

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
