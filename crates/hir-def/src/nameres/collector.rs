//! The core of the module-level name resolution algorithm.
//!
//! `DefCollector::collect` contains the fixed-point iteration loop which
//! resolves imports and expands macros.

use crate::{db::DefDatabase, item_tree::TreeId};

use super::DefMap;

pub(super) fn collect_defs(db: &dyn DefDatabase, def_map: DefMap, tree_id: TreeId) -> DefMap {
    let _p = tracing::info_span!("collect_defs", name = ?tree_id).entered();
    def_map
}
