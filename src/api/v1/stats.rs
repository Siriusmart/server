use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
};

use actix_web::{get, http::header::ContentType, web, HttpResponse};
use chrono::Utc;
use serde::Serialize;

use crate::api::structs::stats::{LifetimeStats, SessionStats};

#[derive(Serialize)]
pub struct StatsRes<'a> {
    pub software: SoftwareStatsRes,
    pub session: SessionStatsRes<'a>,
    pub lifetime: LifetimeStatsRes<'a>,
}

#[derive(Serialize)]
pub struct SessionStatsRes<'a> {
    pub start: i64,
    pub uptime: i64,
    pub served: u32,
    pub served_paths: &'a HashMap<String, u32>,
}

#[derive(Serialize)]
pub struct LifetimeStatsRes<'a> {
    pub total_uptime: u64,
    pub served: u32,
    pub served_paths: &'a HashMap<String, u32>,
}

impl<'a> From<(&'a SessionStats, &'a LifetimeStats)> for StatsRes<'a> {
    fn from((session_stats, lifetime_stats): (&'a SessionStats, &'a LifetimeStats)) -> Self {
        Self {
            software: SoftwareStatsRes::default(),
            lifetime: LifetimeStatsRes {
                total_uptime: lifetime_stats.total_uptime,
                served: lifetime_stats.served,
                served_paths: &lifetime_stats.served_paths,
            },
            session: SessionStatsRes {
                start: session_stats.start,
                uptime: Utc::now().timestamp_millis() - session_stats.start,
                served: session_stats.served,
                served_paths: &session_stats.served_paths,
            },
        }
    }
}

const VERSION: &str = "0.1.0";

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
