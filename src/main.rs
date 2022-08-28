use std::{env, sync::{Arc, Mutex}, error::Error};
use actix_web::{web, App, HttpServer, middleware::Logger, dev::Service};
use reqwest::StatusCode;
use server::{init::init, api::structs::stats::{SessionStats, LifetimeStats}, root::{root, ping}};
use futures_util::future::FutureExt;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    init()?;

    let session_stats = Arc::new(Mutex::new(SessionStats::new()));
    let mut lifetime_stats = LifetimeStats::load();

    let session_stats_for_app = session_stats.clone();
    let lifetime_stats_for_app = lifetime_stats.clone();

    HttpServer::new(move || {
        let session_stats_for_middleware = session_stats_for_app.clone();

        App::new()
            .wrap(Logger::default())
            .wrap_fn(move |req, srv| {

                let session_stats_cloned = session_stats_for_middleware.clone();
                let req_path = req.path().to_string();

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
                    match session_stats.served_paths.get_mut(&req_path) {
                        Some(path) => *path += 1,
                        None => {session_stats.served_paths.insert(req_path, 1);},
                    }

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

    lifetime_stats.merge(&*session_stats.lock().unwrap());
    lifetime_stats.write()?;

    Ok(())
}
