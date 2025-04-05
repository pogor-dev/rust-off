//! The home of `HirDatabase`, which is the Salsa database containing all the
//! type inference-related queries.

use base_db::Upcast;
use hir_def::{db::DefDatabase, hir::ExprId, DefWithBodyId, IndirectObjectId};
use triomphe::Arc;

use crate::InferenceResult;

#[query_group::query_group]
pub trait HirDatabase: DefDatabase + Upcast<dyn DefDatabase> + std::fmt::Debug {
    #[salsa::invoke_actual(crate::infer::infer_query)]
    fn infer(&self, def: DefWithBodyId) -> Arc<InferenceResult>;
}
