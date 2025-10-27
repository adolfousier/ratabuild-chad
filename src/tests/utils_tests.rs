// Utils tests

use crate::utils::detect_language_for_path;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_detect_language_rust() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("Cargo.toml"), "").unwrap();
    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "Rust");
}

#[test]
fn test_detect_language_js() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("package.json"), "").unwrap();

    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "JavaScript");
}

#[test]
fn test_detect_language_python() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("pyproject.toml"), "").unwrap();

    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "Python");
}

#[test]
fn test_detect_language_go() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("go.mod"), "").unwrap();

    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "Go");
}

#[test]
fn test_detect_language_c_cpp() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("Makefile"), "").unwrap();

    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "C/C++");
}

#[test]
fn test_detect_language_java() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("pom.xml"), "").unwrap();

    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "Java");
}

#[test]
fn test_detect_language_php() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("composer.json"), "").unwrap();

    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "PHP");
}

#[test]
fn test_detect_language_ruby() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("Gemfile"), "").unwrap();

    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "Ruby");
}

#[test]
fn test_detect_language_swift() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("Package.swift"), "").unwrap();

    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "Swift");
}

#[test]
fn test_detect_language_kotlin() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("build.gradle.kts"), "").unwrap();

    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "Kotlin");
}

#[test]
fn test_detect_language_scala() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("build.sbt"), "").unwrap();

    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "Scala");
}

#[test]
fn test_detect_language_haskell() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("stack.yaml"), "").unwrap();

    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "Haskell");
}

#[test]
fn test_detect_language_elixir() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("mix.exs"), "").unwrap();

    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "Elixir");
}

#[test]
fn test_detect_language_unknown() {
    let temp_dir = TempDir::new().unwrap();

    assert_eq!(detect_language_for_path(temp_dir.path().to_str().unwrap()), "Unknown");
}