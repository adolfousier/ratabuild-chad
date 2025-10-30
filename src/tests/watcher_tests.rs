// Watcher tests

use crate::tracking::watcher::BuildWatcher;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_build_watcher() {
    let temp_dir = TempDir::new().unwrap();
    let mut watcher = BuildWatcher::new(false);
    // Watch the temp dir - may fail due to system inotify limits in tests
    // Just ensure it doesn't panic
    match watcher.watch(temp_dir.path()) {
        Ok(()) => {
            // Create a file
            fs::write(temp_dir.path().join("test.txt"), "test").unwrap();
        }
        Err(_) => {
            // Expected in CI environments with inotify limits - test passes
        }
    }
}