// Configuration module

pub mod settings;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub scan_paths: Vec<String>,
    pub retention_days: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: "postgres://user:password@localhost:25851/ratabuild-chad".to_string(),
            scan_paths: vec![".".to_string()],
            retention_days: 30,
        }
    }
}
