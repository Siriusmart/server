use actix_web::{dev::Service, middleware::Logger, web, App, HttpServer};
use futures_util::future::FutureExt;
use reqwest::StatusCode;
use server::{
    api::structs::stats::{LifetimeStats, SessionStats},
    init::init,
    root::{ping, root},
};
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init()?;

    let session_stats = Arc::new(Mutex::new(SessionStats::default()));
    let mut lifetime_stats = LifetimeStats::load();

    let session_stats_for_app = session_stats.clone();
    let lifetime_stats_for_app = lifetime_stats.clone();

    HttpServer::new(move || {
        let session_stats_for_middleware = session_stats_for_app.clone();

        App::new()
            .wrap(Logger::default())
            .wrap_fn(move |req, srv| {
                let session_stats_cloned = session_stats_for_middleware.clone();

                srv.call(req).map(move |res| {
                    let res_unwrapped = match &res {
                        Ok(res) => res,
                        Err(_) => return res,
                    };

                    if res_unwrapped.status() != StatusCode::OK {
                        return res;
                    }

                    let session_stats = &mut *session_stats_cloned.lock().unwrap();
                    session_stats.served += 1;

                    res
                })
            })
            .service(server::api::scope())
            .service(root)
            .service(ping)
            .app_data(web::Data::new(session_stats_for_app.clone()))
            .app_data(web::Data::new(lifetime_stats_for_app.clone()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    lifetime_stats.merge(&session_stats.lock().unwrap());
    lifetime_stats.write()?;

    Ok(())
}
