// Config tests

use crate::config::settings::load_config;
use std::env;

#[test]
fn test_load_config() {
    let config = load_config();
    assert!(config.database_url.contains("postgres"));
    assert!(config.scan_paths.contains(&".".to_string()));
    assert_eq!(config.retention_days, 30);
}

#[test]
fn test_load_config_with_env() {
    unsafe {
        env::set_var("DATABASE_URL", "postgres://test:test@localhost/test");
        env::set_var("POSTGRES_USERNAME", "testuser");
    }
    let config = load_config();
    assert_eq!(config.database_url, "postgres://test:test@localhost/test");
    unsafe {
        env::remove_var("DATABASE_URL");
    }
}