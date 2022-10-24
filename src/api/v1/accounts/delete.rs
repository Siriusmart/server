use crate::global::structs::Account;
use actix_web::{get, web, Responder};
use log::warn;
use serde::{Deserialize, Serialize};

#[get("/delete/{username_or_id}/{password}")]
async fn delete(path: web::Path<(String, String)>) -> impl Responder {
    let (mut username_or_id, password) = path.into_inner();

    let id = if Account::exists(&username_or_id) {
        username_or_id
    } else {
        username_or_id = username_or_id.to_lowercase();
        if Account::exists_username(&username_or_id) {
            if let Ok(id) = Account::load_username(&username_or_id) {
                id
            } else {
                return web::Json(AccountDeleteResponse::Error {
                    error: String::from("error getting user id from username"),
                });
            }
        } else {
            return web::Json(AccountDeleteResponse::Error {
                error: String::from("no such user with this username or id"),
            });
        }
    };

    let account = match Account::load(&id) {
        Ok(account) => account,
        Err(e) => {
            return web::Json(AccountDeleteResponse::Error {
                error: e.to_string(),
            })
        }
    };

    let hashed_password = Account::password_hash(&id, password);

    if hashed_password != account.password_hash {
        return web::Json(AccountDeleteResponse::Error {
            error: String::from("incorrect password"),
        });
    }

    if let Err(e) = Account::delete(account.user_id.as_str()) {
        warn!("Error deleting user profile file: {e}")
    }

    if let Err(e) = Account::delete_username(&account.username) {
        warn!("Error deleting username file: {e}")
    }

    web::Json(AccountDeleteResponse::Success {
        username: account.username,
        user_id: account.user_id,
    })
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum AccountDeleteResponse {
    Success { username: String, user_id: String },
    Error { error: String },
}
