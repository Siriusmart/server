use actix_web::{web, Scope};

mod responses;
pub use responses::*;

mod changepassword;
mod create;
mod delete;
mod login;
mod rename;

pub fn scope() -> Scope {
    web::scope("/accounts")
        .service(create::create)
        .service(login::login)
        .service(delete::delete)
        .service(changepassword::changepassword)
        .service(rename::rename)
}
