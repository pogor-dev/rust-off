//! Type inference, i.e. the process of walking through the code and determining
//! the type of each expression and pattern.
//!
//! For type inference, compare the implementations in rustc (the various
//! check_* methods in rustc_hir_analysis/check/mod.rs are a good entry point) and
//! IntelliJ-Rust (org.rust.lang.core.types.infer). Our entry point for
//! inference here is the `infer` function, which infers the types of all
//! expressions in a given function.
//!
//! During inference, types (i.e. the `Ty` struct) can contain type 'variables'
//! which represent currently unknown types; as we walk through the expressions,
//! we might determine that certain variables need to be equal to each other, or
//! to certain types. To record this, we use the union-find implementation from
//! the `ena` crate, which is extracted from rustc.

use crate::db::HirDatabase;
use hir_def::{expr_store::Body, DefWithBodyId};
use triomphe::Arc;

/// The entry point of type inference.
pub(crate) fn infer_query(db: &dyn HirDatabase, def: DefWithBodyId) -> Arc<InferenceResult> {
    let _p = tracing::info_span!("infer_query").entered();
    let body = db.body(def);
    let mut ctx = InferenceContext::new(db, def, &body);

    match def {
        DefWithBodyId::IndirectObjectId(f) => {
            ctx.collect_indirect_object(f);
        }
    }

    ctx.infer_body();
    Arc::new(ctx.resolve_all())
}

/// The inference context contains all information needed during type inference.
#[derive(Clone, Debug)]
pub(crate) struct InferenceContext<'a> {
    pub(crate) db: &'a dyn HirDatabase,
    pub(crate) owner: DefWithBodyId,
    pub(crate) body: &'a Body,
}

impl<'a> InferenceContext<'a> {
    fn new(db: &'a dyn HirDatabase, owner: DefWithBodyId, body: &'a Body) -> Self {
        InferenceContext { db, owner, body }
    }
}

/// The result of type inference: A mapping from expressions and patterns to types.
///
/// When you add a field that stores types (including `Substitution` and the like), don't forget
/// `resolve_completely()`'ing  them in `InferenceContext::resolve_all()`. Inference variables must
/// not appear in the final inference result.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct InferenceResult {}
