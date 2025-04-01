//! Defines database & queries for name resolution.

use crate::{ObjectId, ObjectLoc};
use base_db::{query_group, ExpandDatabase, RootQueryDb, SourceDatabase, Upcast};

#[query_group::query_group(InternDatabaseStorage)]
pub trait InternDatabase: RootQueryDb {
    #[salsa::interned]
    fn intern_function(&self, loc: ObjectLoc) -> ObjectId;
}

#[query_group::query_group]
pub trait DefDatabase: InternDatabase + ExpandDatabase + SourceDatabase + Upcast<dyn ExpandDatabase> + Upcast<dyn RootQueryDb> {}
