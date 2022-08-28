use std::{fs::{OpenOptions, self}, path::Path, error::Error, collections::HashMap, io::Write};

use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SessionStats {
    pub start: i64,
    pub served: u32,
    pub served_paths: HashMap<String, u32>,
}

impl SessionStats {
    pub fn new() -> Self {
        Self {
            start: Utc::now().timestamp_millis(),
            served: 0,
            served_paths: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LifetimeStats {
    pub served: u32,
    pub served_paths: HashMap<String, u32>,
    pub total_uptime: u64,
}

impl Default for LifetimeStats {
    fn default() -> Self {
        Self {
            served: 0,
            total_uptime: 0,
            served_paths: HashMap::new(),
        }
    }
}

const LIFETIME_STATS_PATH: &'static str = "./storage/stats/lifetime.json";

impl LifetimeStats {
    pub fn load() -> Self {
        let path = Path::new(LIFETIME_STATS_PATH);

        match fs::read_to_string(path) {
            Ok(content) => {
                match serde_json::from_str(&content) {
                    Ok(content) => content,
                    Err(_) => Self::default(),
                }
            },
            Err(_) => Self::default(),
        }
    }

    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        let path = Path::new(LIFETIME_STATS_PATH);

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;

        let serialized = serde_json::to_string(&self)?;

        file.write(serialized.as_bytes())?;
        
        Ok(())
    }

    pub fn merge(&mut self, session: &SessionStats) {
        self.served += session.served;

        for (key, value) in session.served_paths.iter() {
            match self.served_paths.get_mut(key) {
                Some(path) => *path += value,
                None => {
                    self.served_paths.insert(key.to_string(), *value);
                },
            }
        }

        self.total_uptime += (Utc::now().timestamp_millis() - session.start) as u64;
    }
}