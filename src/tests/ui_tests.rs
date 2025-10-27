// UI tests

use crate::utils::detect_language_for_path;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_detect_language() {
    // Test Rust
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("Cargo.toml"), "").unwrap();

    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "Rust");

    // Test JS
    fs::remove_file(temp_dir.path().join("Cargo.toml")).unwrap();
    fs::write(temp_dir.path().join("package.json"), "").unwrap();
    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "JavaScript");

    // Test unknown
    fs::remove_file(temp_dir.path().join("package.json")).unwrap();
    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "Unknown");
}

