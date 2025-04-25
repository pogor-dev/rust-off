//! Defines database & queries for name resolution.

use base_db::{RootQueryDb, SourceDatabase};
use span::FileId;
use triomphe::Arc;

use crate::nameres::DefMap;

#[salsa::db]
pub(crate) struct InternDatabaseStorage {
    storage: salsa::Storage<Self>,
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

#[salsa::db]
pub trait InternDatabase: RootQueryDb {}

#[salsa::db]
impl<DB> InternDatabase for DB where DB: RootQueryDb {}

#[salsa::db]
pub trait DefDatabase: InternDatabase + SourceDatabase {
    fn pdf_document_def_map(&self, file_id: FileId) -> Arc<DefMap>;
}

#[salsa::db]
impl<DB> DefDatabase for DB
where
    DB: InternDatabase + SourceDatabase,
{
    fn pdf_document_def_map(&self, file_id: FileId) -> Arc<DefMap> {
        Arc::new(DefMap {})
    }
}
