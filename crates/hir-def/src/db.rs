//! Defines database & queries for name resolution.

use crate::{
    expr_store::{Body, BodySourceMap},
    DefWithBodyId, IndirectObjectId, IndirectObjectLoc,
};
use base_db::{query_group, RootQueryDb, SourceDatabase, Upcast};
use salsa::plumbing::AsId;
use triomphe::Arc;

#[query_group::query_group(InternDatabaseStorage)]
pub trait InternDatabase: RootQueryDb {
    #[salsa::interned]
    fn intern_indirect_object(&self, loc: IndirectObjectLoc) -> IndirectObjectId;
}

#[query_group::query_group]
pub trait DefDatabase: InternDatabase + SourceDatabase + Upcast<dyn RootQueryDb> {
    #[salsa::invoke(Body::body_with_source_map_query)]
    #[salsa::lru(512)]
    fn body_with_source_map(&self, def: DefWithBodyId) -> (Arc<Body>, Arc<BodySourceMap>);

    #[salsa::invoke(Body::body_query)]
    fn body(&self, def: DefWithBodyId) -> Arc<Body>;
}
