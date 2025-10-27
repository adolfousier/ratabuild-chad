// Utils tests

use crate::utils::detect_language;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_detect_language_rust() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("Cargo.toml"), "").unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    assert_eq!(detect_language(), "Rust");
}

#[test]
fn test_detect_language_js() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("package.json"), "").unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    assert_eq!(detect_language(), "JavaScript");
}

#[test]
fn test_detect_language_python() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("pyproject.toml"), "").unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    assert_eq!(detect_language(), "Python");
}

#[test]
fn test_detect_language_go() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("go.mod"), "").unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    assert_eq!(detect_language(), "Go");
}

#[test]
fn test_detect_language_c_cpp() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("Makefile"), "").unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    assert_eq!(detect_language(), "C/C++");
}

#[test]
fn test_detect_language_java() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("pom.xml"), "").unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    assert_eq!(detect_language(), "Java");
}

#[test]
fn test_detect_language_php() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("composer.json"), "").unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    assert_eq!(detect_language(), "PHP");
}

#[test]
fn test_detect_language_ruby() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("Gemfile"), "").unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    assert_eq!(detect_language(), "Ruby");
}

#[test]
fn test_detect_language_swift() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("Package.swift"), "").unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    assert_eq!(detect_language(), "Swift");
}

#[test]
fn test_detect_language_kotlin() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("build.gradle.kts"), "").unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    assert_eq!(detect_language(), "Kotlin");
}

#[test]
fn test_detect_language_scala() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("build.sbt"), "").unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    assert_eq!(detect_language(), "Scala");
}

#[test]
fn test_detect_language_haskell() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("stack.yaml"), "").unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    assert_eq!(detect_language(), "Haskell");
}

#[test]
fn test_detect_language_elixir() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("mix.exs"), "").unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    assert_eq!(detect_language(), "Elixir");
}

#[test]
fn test_detect_language_unknown() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();
    assert_eq!(detect_language(), "Unknown");
}