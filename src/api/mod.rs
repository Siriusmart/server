use actix_web::Scope;

pub mod v1;
pub mod structs;

pub fn scope() -> Scope {
    Scope::new("/api")
        .service(v1::scope())
}
