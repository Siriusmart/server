use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use actix_web::{get, http::header::ContentType, web, HttpResponse};
use chrono::Utc;
use serde::Serialize;

use crate::api::structs::stats::{LifetimeStats, SessionStats};

#[derive(Serialize)]
pub struct StatsRes {
    pub software: SoftwareStatsRes,
    pub session: SessionStatsRes,
    pub lifetime: LifetimeStatsRes,
}

#[derive(Serialize)]
pub struct SessionStatsRes {
    pub start: i64,
    pub uptime: i64,
    pub served: u32,
}

#[derive(Serialize)]
pub struct LifetimeStatsRes {
    pub total_uptime: u64,
    pub served: u32,
}

impl From<(&SessionStats, &LifetimeStats)> for StatsRes {
    fn from((session_stats, lifetime_stats): (&SessionStats, &LifetimeStats)) -> Self {
        Self {
            software: SoftwareStatsRes::default(),
            lifetime: LifetimeStatsRes {
                total_uptime: lifetime_stats.total_uptime,
                served: lifetime_stats.served,
            },
            session: SessionStatsRes {
                start: session_stats.start,
                uptime: Utc::now().timestamp_millis() - session_stats.start,
                served: session_stats.served,
            },
        }
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
pub struct SoftwareStatsRes {
    pub version: &'static str,
}

impl Default for SoftwareStatsRes {
    fn default() -> Self {
        Self { version: VERSION }
    }
}

#[get("/stats")]
pub async fn stats(
    session_stats: web::Data<Arc<Mutex<SessionStats>>>,
    lifetime_stats: web::Data<LifetimeStats>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let session_stats_mut_ref = &*session_stats.lock().unwrap();
    let res_struct = StatsRes::from((session_stats_mut_ref, &**lifetime_stats));
    let res_content = serde_json::to_string(&res_struct)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(res_content))
}
