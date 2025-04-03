//! Defines `Body`: a lowered representation of functions, statics and
//! consts.

use std::ops;

use triomphe::Arc;

use crate::{
    db::DefDatabase,
    expr_store::{ExpressionStore, ExpressionStoreSourceMap},
    hir::ExprId,
    DefWithBodyId,
};

/// The body of an item (function, const etc.).
#[derive(Debug, Eq, PartialEq)]
pub struct Body {
    pub store: ExpressionStore,
    /// The `ExprId` of the actual body expression.
    pub body_expr: ExprId,
}

impl Body {
    pub(crate) fn body_with_source_map_query(db: &dyn DefDatabase, def: DefWithBodyId) -> (Arc<Body>, Arc<BodySourceMap>) {
        let _p = tracing::info_span!("body_with_source_map_query").entered();
    }

    pub(crate) fn body_query(db: &dyn DefDatabase, def: DefWithBodyId) -> Arc<Body> {
        db.body_with_source_map(def).0
    }
}

impl ops::Deref for Body {
    type Target = ExpressionStore;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

/// An item body together with the mapping from syntax nodes to HIR expression
/// IDs. This is needed to go from e.g. a position in a file to the HIR
/// expression containing it; but for type inference etc., we want to operate on
/// a structure that is agnostic to the actual positions of expressions in the
/// file, so that we don't recompute types whenever some whitespace is typed.
///
/// One complication here is that, due to macro expansion, a single `Body` might
/// be spread across several files. So, for each ExprId and PatId, we record
/// both the HirFileId and the position inside the file. However, we only store
/// AST -> ExprId mapping for non-macro files, as it is not clear how to handle
/// this properly for macros.
#[derive(Default, Debug, Eq, PartialEq)]
pub struct BodySourceMap {
    pub store: ExpressionStoreSourceMap,
}

impl ops::Deref for BodySourceMap {
    type Target = ExpressionStoreSourceMap;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}
