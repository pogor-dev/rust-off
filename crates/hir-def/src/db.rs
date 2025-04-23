//! Defines database & queries for name resolution.

use base_db::{RootQueryDb, SourceDatabase};
use span::FileId;
use triomphe::Arc;

use crate::nameres::DefMap;

#[salsa::db]
pub trait InternDatabase: RootQueryDb + salsa::Database {}

#[salsa::db]
impl<DB> InternDatabase for DB where DB: RootQueryDb + salsa::Database {}

#[salsa::db]
pub trait DefDatabase: InternDatabase + SourceDatabase {
    fn file_def_map(&self, file_id: FileId) -> Arc<DefMap>;
}

#[salsa::db]
impl<DB> DefDatabase for DB
where
    DB: InternDatabase + SourceDatabase,
{
    fn file_def_map(&self, file_id: FileId) -> Arc<DefMap> {
        Arc::new(DefMap {})
    }
}
