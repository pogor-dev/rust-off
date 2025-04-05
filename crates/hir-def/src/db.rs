//! Defines database & queries for name resolution.

use crate::{
    expr_store::{Body, BodySourceMap},
    item_tree::{ItemTree, ItemTreeSourceMaps},
    DefWithBodyId, IndirectObjectId, IndirectObjectLoc,
};
use base_db::{query_group, RootQueryDb, SourceDatabase, Upcast};
use salsa::plumbing::AsId;
use span::HirFileId;
use triomphe::Arc;

#[query_group::query_group(InternDatabaseStorage)]
pub trait InternDatabase: RootQueryDb {
    #[salsa::interned]
    fn intern_indirect_object(&self, loc: IndirectObjectLoc) -> IndirectObjectId;
}

#[query_group::query_group]
pub trait DefDatabase: InternDatabase + SourceDatabase + Upcast<dyn RootQueryDb> {
    /// Computes an [`ItemTree`] for the given file or macro expansion.
    #[salsa::invoke(ItemTree::file_item_tree_query)]
    fn file_item_tree(&self, file_id: HirFileId) -> Arc<ItemTree>;

    #[salsa::invoke(ItemTree::file_item_tree_with_source_map_query)]
    fn file_item_tree_with_source_map(&self, file_id: HirFileId) -> (Arc<ItemTree>, Arc<ItemTreeSourceMaps>);

    #[salsa::invoke(Body::body_with_source_map_query)]
    #[salsa::lru(512)]
    fn body_with_source_map(&self, def: DefWithBodyId) -> (Arc<Body>, Arc<BodySourceMap>);

    #[salsa::invoke(Body::body_query)]
    fn body(&self, def: DefWithBodyId) -> Arc<Body>;
}
