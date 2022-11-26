use actix_web::{web, Scope};

mod changepassword;
mod changeemail;
mod create;
mod delete;
mod login;
mod rename;
mod verify;

pub fn scope() -> Scope {
    web::scope("/accounts")
        .service(create::create)
        .service(login::login)
        .service(delete::delete)
        .service(changepassword::changepassword)
        .service(changeemail::changeemail)
        .service(rename::rename)
        .service(verify::verify)
}
