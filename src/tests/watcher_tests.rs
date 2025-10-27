// Watcher tests

use crate::tracking::watcher::BuildWatcher;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_build_watcher() {
    let temp_dir = TempDir::new().unwrap();
    let mut watcher = BuildWatcher::new(false);
    // Watch the temp dir
    watcher.watch(temp_dir.path()).unwrap();
    // Create a file
    fs::write(temp_dir.path().join("test.txt"), "test").unwrap();
    // In real, it would log if debug_logs_enabled is true, but for test, just check no panic
    // Just ensure watch doesn't fail
}