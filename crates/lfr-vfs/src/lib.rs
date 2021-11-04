pub extern crate notify;

use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use notify::{DebouncedEvent, INotifyWatcher, RecursiveMode, Watcher, watcher};

#[salsa::query_group(VfsDatabaseStorage)]
pub trait VfsDatabase: salsa::Database + FileWatcher {
    fn read(&self, path: PathBuf) -> String;
}

fn read(db: &dyn VfsDatabase, path: PathBuf) -> String {
    db.salsa_runtime()
        .report_synthetic_read(salsa::Durability::LOW);

    db.watch(&path);
    std::fs::read_to_string(&path).unwrap_or_default()
}

pub struct VfsWatcher(Arc<Mutex<INotifyWatcher>>, Receiver<DebouncedEvent>);

impl VfsWatcher {
    pub fn watch(&self, path: &Path) {
        let mut watcher = self.0.lock().unwrap();
        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(path, RecursiveMode::Recursive).unwrap();
    }

    pub fn did_change_file(db: &mut dyn VfsDatabase, path: &PathBuf) {
        ReadQuery.in_db_mut(db).invalidate(path);
    }
}

pub trait FileWatcher {
    fn watch(&self, path: &Path);
    fn did_change_file(&mut self, path: &PathBuf);
}

pub fn setup_watcher() -> VfsWatcher {
    let (tx, rx) = channel();
    let watcher = Arc::from(Mutex::new(watcher(tx, Duration::from_secs(1)).unwrap()));

    VfsWatcher(watcher, rx)
}
