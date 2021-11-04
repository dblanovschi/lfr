use std::path::{Path, PathBuf};

use lfr_base_db::salsa;
use lfr_vfs::FileWatcher;

#[salsa::database(
    lfr_vfs::VfsDatabaseStorage,
    lfr_hir_def::db::HirDefStorage,
    lfr_hir_def::db::InternDatabaseStorage,
)]
struct LfrDatabase {
    storage: salsa::Storage<Self>,
    watcher: lfr_vfs::VfsWatcher,
}

impl<'a> salsa::Database for LfrDatabase {}

impl FileWatcher for LfrDatabase {
    fn watch(&self, path: &Path) {
        self.watcher.watch(path);
    }

    fn did_change_file(&mut self, path: &PathBuf) {
        lfr_vfs::VfsWatcher::did_change_file(self, path);
    }
}
