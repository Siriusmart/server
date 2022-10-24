use crate::global::structs::Account;
use actix_web::{get, web, Responder};
use serde::{Deserialize, Serialize};

#[get("/login/{username_or_id}/{password}")]
async fn login(path: web::Path<(String, String)>) -> impl Responder {
    let (mut username_or_id, password) = path.into_inner();

    let id = if Account::exists(&username_or_id) {
        username_or_id
    } else {
        username_or_id = username_or_id.to_lowercase();
        if Account::exists_username(&username_or_id) {
            if let Ok(id) = Account::load_username(&username_or_id) {
                id
            } else {
                return web::Json(AccountLoginResponse::Error {
                    error: String::from("error getting user id from username"),
                });
            }
        } else {
            return web::Json(AccountLoginResponse::Error {
                error: String::from("no such user with this username or id"),
            });
        }
    };

    let account = match Account::load(&id) {
        Ok(account) => account,
        Err(e) => {
            return web::Json(AccountLoginResponse::Error {
                error: e.to_string(),
            })
        }
    };

    let hashed_password = Account::password_hash(&id, password);

    if hashed_password != account.password_hash {
        return web::Json(AccountLoginResponse::Error {
            error: String::from("incorrect password"),
        });
    }

    web::Json(AccountLoginResponse::Success {
        username: account.username,
        id: account.user_id,
    })
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum AccountLoginResponse {
    Success { username: String, id: String },
    Error { error: String },
}
