// UI tests

use crate::utils::detect_language;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_detect_language() {
    // Test Rust
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("Cargo.toml"), "").unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    assert_eq!(detect_language(), "Rust");

    // Test JS
    fs::remove_file(temp_dir.path().join("Cargo.toml")).unwrap();
    fs::write(temp_dir.path().join("package.json"), "").unwrap();
    assert_eq!(detect_language(), "JavaScript");

    // Test unknown
    fs::remove_file(temp_dir.path().join("package.json")).unwrap();
    assert_eq!(detect_language(), "Unknown");
}

