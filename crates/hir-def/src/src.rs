//! Utilities for mapping between hir IDs and the surface syntax.

use pdfc_syntax::{AstNode, AstPtr};

use crate::{db::DefDatabase, files::InFile, item_tree::ItemTreeNode, ItemTreeLoc};

pub trait HasSource {
    type Value: AstNode;
    fn source(&self, db: &dyn DefDatabase) -> InFile<Self::Value> {
        let InFile { file_id, value } = self.ast_ptr(db);
        InFile::new(file_id, value.to_node(&db.parse(file_id)))
    }
    fn ast_ptr(&self, db: &dyn DefDatabase) -> InFile<AstPtr<Self::Value>>;
}

impl<T> HasSource for T
where
    T: ItemTreeLoc,
    T::Id: ItemTreeNode,
{
    type Value = <T::Id as ItemTreeNode>::Source;
    fn ast_ptr(&self, db: &dyn DefDatabase) -> InFile<AstPtr<Self::Value>> {
        let id = self.item_tree_id();
        let file_id = id.file_id();
        let tree = id.item_tree(db);
        let ast_id_map = db.ast_id_map(file_id);
        let node = &tree[id.value];

        InFile::new(file_id, ast_id_map.get(node.ast_id()))
    }
}
