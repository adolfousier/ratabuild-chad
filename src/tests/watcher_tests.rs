// Watcher tests

use crate::tracking::watcher::BuildWatcher;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_build_watcher() {
    let temp_dir = TempDir::new().unwrap();
    let mut watcher = BuildWatcher::new();
    // Watch the temp dir
    watcher.watch(temp_dir.path()).unwrap();
    // Create a file
    fs::write(temp_dir.path().join("test.txt"), "test").unwrap();
    // In real, it would log, but for test, just check no panic
    // Since events are printed, hard to assert
    // Just ensure watch doesn't fail
}