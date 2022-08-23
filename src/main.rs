use std::io;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(server::utils::scope()))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;

    Ok(())
}
