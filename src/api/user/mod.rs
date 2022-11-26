pub mod accounts;

use actix_web::Scope;

pub fn scope() -> Scope {
    Scope::new("/user").service(accounts::scope())
}
