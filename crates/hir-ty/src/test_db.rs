//! Database used for testing `hir`.

use std::{fmt, panic, sync::Mutex};
use triomphe::Arc;

#[salsa::db]
#[derive(Clone)]
pub(crate) struct TestDB {
    storage: salsa::Storage<Self>,
    files: Arc<base_db::Files>,
    events: Arc<Mutex<Option<Vec<salsa::Event>>>>,
}

impl Default for TestDB {
    fn default() -> Self {
        let mut this = Self {
            storage: Default::default(),
            events: Default::default(),
            files: Default::default(),
        };
        this
    }
}

impl fmt::Debug for TestDB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TestDB").finish()
    }
}
