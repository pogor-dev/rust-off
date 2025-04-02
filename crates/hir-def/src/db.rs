//! Defines database & queries for name resolution.

use crate::{IndirectObjectId, IndirectObjectLoc};
use base_db::{query_group, ExpandDatabase, RootQueryDb, SourceDatabase, Upcast};

#[query_group::query_group(InternDatabaseStorage)]
pub trait InternDatabase: RootQueryDb {
    #[salsa::interned]
    fn intern_indirect_object(&self, loc: IndirectObjectLoc) -> IndirectObjectId;
}

#[query_group::query_group]
pub trait DefDatabase: InternDatabase + ExpandDatabase + SourceDatabase + Upcast<dyn ExpandDatabase> + Upcast<dyn RootQueryDb> {}
