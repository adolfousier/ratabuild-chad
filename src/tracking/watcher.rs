// File system watcher for build artifacts

use notify::{RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher};
use std::sync::{Arc, Mutex};
use std::path::Path;

#[derive(Clone)]
pub struct BuildWatcher {
    watcher: Arc<Mutex<RecommendedWatcher>>,
}

impl BuildWatcher {
    pub fn new() -> Self {
        let watcher = RecommendedWatcher::new(
            |res: Result<notify::Event, notify::Error>| match res {
                Ok(event) => {
                    // Only log create/modify/remove events, not access
                    if matches!(
                        event.kind,
                        notify::EventKind::Create(_)
                            | notify::EventKind::Modify(_)
                            | notify::EventKind::Remove(_)
                    ) {
                        println!("Build change detected: {:?}", event);
                    }
                }
                Err(e) => println!("Watch error: {:?}", e),
            },
            notify::Config::default(),
        )
        .unwrap();
        BuildWatcher { watcher: Arc::new(Mutex::new(watcher)) }
    }

    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> NotifyResult<()> {
        self.watcher.lock().unwrap()
            .watch(path.as_ref(), RecursiveMode::Recursive)?;
        Ok(())
    }
}
