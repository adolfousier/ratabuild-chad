// Configuration types

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub scan_paths: Vec<String>,
    pub retention_days: u32,
    #[serde(default)]
    pub debug_logs_enabled: bool,
    #[serde(default)]
    pub excluded_paths: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: String::new(), // Must be set from .env
            scan_paths: vec![".".to_string()],
            retention_days: 30,
            debug_logs_enabled: false,
            excluded_paths: vec![],
        }
    }
}
