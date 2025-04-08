//! AST -> `ItemTree` lowering code.

use std::cell::OnceCell;

use span::{AstIdMap, HirFileId, SpanMap};
use triomphe::Arc;

use crate::{db::DefDatabase, item_tree::ItemTree};

pub(super) struct Ctx<'a> {
    db: &'a dyn DefDatabase,
    tree: ItemTree,
    source_ast_id_map: Arc<AstIdMap>,
    span_map: OnceCell<SpanMap>,
    file: HirFileId,
    source_maps: ItemTreeSourceMapsBuilder,
}

impl<'a> Ctx<'a> {
    pub(super) fn new(db: &'a dyn DefDatabase, file: HirFileId) -> Self {
        Self {
            db,
            tree: ItemTree::default(),
            source_ast_id_map: db.ast_id_map(file),
            file,
            span_map: OnceCell::new(),
            source_maps: ItemTreeSourceMapsBuilder::default(),
        }
    }
}
