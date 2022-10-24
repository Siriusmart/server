use actix_web::{web, Scope};

mod create;
mod delete;
mod login;

pub fn scope() -> Scope {
    web::scope("/accounts")
        .service(create::create)
        .service(login::login)
        .service(delete::delete)
}
