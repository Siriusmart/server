use actix_web::{web, Scope};

mod request_proxy;

pub fn scope() -> Scope {
    web::scope("/utils").service(request_proxy::scope())
}
