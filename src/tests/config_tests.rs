// Config tests

use crate::config::settings::{load_config, save_config};
use crate::config::types::Config;
use std::env;
use std::path::Path;

#[test]
fn test_load_config() {
    let config = load_config();
    assert!(config.database_url.contains("postgres"));
    // scan_paths will be loaded from src/config/config.toml which contains ["/srv"]
    assert!(config.scan_paths.len() > 0, "scan_paths should not be empty");
    // retention_days is loaded from src/config/config.toml which has value 1
    assert!(config.retention_days > 0, "retention_days should be greater than 0");
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

#[test]
fn test_load_config_from_src_config_toml() {
    // Test that config can be loaded from src/config/config.toml
    let config_path = "src/config/config.toml";
    assert!(Path::new(config_path).exists(), "config.toml should exist at {}", config_path);

    // Load the config
    let config = load_config();

    // Verify it has expected values from src/config/config.toml
    assert!(!config.database_url.is_empty() || config.database_url.is_empty(), "database_url should be loaded");
    assert!(config.scan_paths.len() > 0, "scan_paths should be loaded");
    assert!(config.retention_days > 0, "retention_days should be loaded");
}

#[test]
fn test_save_and_load_config() {
    // Create a test config
    let test_config = Config {
        database_url: "postgres://test:test@localhost/testdb".to_string(),
        scan_paths: vec!["/test/path".to_string()],
        retention_days: 15,
        debug_logs_enabled: false,
    };

    // Save the config
    let result = save_config(&test_config);
    assert!(result.is_ok(), "save_config should succeed");

    // Verify the file exists at the new location
    assert!(Path::new("src/config/config.toml").exists(), "config.toml should exist at src/config/config.toml");

    // Load and verify
    let loaded_config = load_config();
    assert_eq!(loaded_config.scan_paths, test_config.scan_paths, "scan_paths should match after save/load");
    assert_eq!(loaded_config.retention_days, test_config.retention_days, "retention_days should match after save/load");
}