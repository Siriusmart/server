pub mod accounts;
pub mod stats;
pub mod utils;

use actix_web::Scope;

pub fn scope() -> Scope {
    Scope::new("/v1")
        .service(stats::stats)
        .service(utils::scope())
        .service(accounts::scope())
}
