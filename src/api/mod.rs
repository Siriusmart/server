use actix_web::Scope;

pub mod structs;
pub mod user;
pub mod v1;

pub fn scope() -> Scope {
    Scope::new("/api")
        .service(v1::scope())
        .service(user::scope())
}
