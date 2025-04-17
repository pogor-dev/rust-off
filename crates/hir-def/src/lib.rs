//! `hir_def` crate contains everything between macro expansion and type
//! inference.
//!
//! It defines various items (structs, enums, traits) which comprises Rust code,
//! as well as an algorithm for resolving paths to such entities.
//!
//! Note that `hir_def` is a work in progress, so not all of the above is
//! actually true.

use crate::db::DefDatabase;
use base_db::impl_intern_key;
use hir_expand::{AstId, impl_intern_lookup};
use pdfc_syntax::ast;

pub mod builtin_type;
pub mod db;

#[cfg(test)]
mod test_db;

macro_rules! impl_intern {
    ($id:ident, $loc:ident, $intern:ident, $lookup:ident) => {
        impl_intern_key!($id, $loc);
        impl_intern_lookup!(DefDatabase, $id, $loc, $intern, $lookup);
    };
}

macro_rules! impl_loc {
    ($loc:ident, $id:ident: $id_ty:ident, $container:ident: $container_type:ident) => {
        impl ItemTreeLoc for $loc {
            type Container = $container_type;
            type Id = $id_ty;
            fn item_tree_id(&self) -> ItemTreeId<Self::Id> {
                self.$id
            }
            fn container(&self) -> Self::Container {
                self.$container
            }
        }
    };
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct BlockLoc {
    pub ast_id: AstId<ast::IndirectObjectExpr>, // TODO: block expr?
    /// The containing module.
    pub module: ModuleId,
}
impl_intern!(BlockId, BlockLoc, intern_block, lookup_intern_block);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModuleId {
    // block: Option<BlockId>,
}
