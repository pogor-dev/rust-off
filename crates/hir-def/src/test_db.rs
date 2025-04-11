//! Database used for testing `hir_def`.

use std::{fmt, sync::Mutex};

use base_db::{FileText, Files, SourceDatabase};
use triomphe::Arc;

#[salsa::db]
#[derive(Default, Clone)]
pub struct TestDB {
    storage: salsa::Storage<Self>,
    files: Arc<Files>,
    events: Arc<Mutex<Option<Vec<salsa::Event>>>>,
}

#[salsa::db]
impl salsa::Database for TestDB {
    fn salsa_event(&self, event: &dyn std::ops::Fn() -> salsa::Event) {
        let mut events = self.events.lock().unwrap();
        if let Some(events) = &mut *events {
            let event = event();
            events.push(event);
        }
    }
}

impl fmt::Debug for TestDB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TestDB").finish()
    }
}

#[salsa::db]
impl SourceDatabase for TestDB {
    fn file_text(&self, file_id: base_db::FileId) -> FileText {
        self.files.file_text(file_id)
    }

    fn set_file_text(&mut self, file_id: base_db::FileId, text: &[u8]) {
        let files = Arc::clone(&self.files);
        files.set_file_text(self, file_id, text);
    }
}
