//! Transforms `ast::Expr` into an equivalent `hir_def::expr::Expr`
//! representation.

use pdfc_syntax::{AstPtr, ast};
use triomphe::Arc;

use crate::{
    DefWithBodyId,
    db::DefDatabase,
    expr_store::{Body, BodySourceMap},
    hir::ExprId,
};

pub(super) fn lower_body(db: &dyn DefDatabase, owner: DefWithBodyId, body: Option<ast::Expr>) -> (Body, BodySourceMap) {
    let mut collector = ExprCollector::new(db, owner);
    let body_expr = collector.collect(body);

    return (
        Body {
            store: collector.store.finish(),
            body_expr,
        },
        BodySourceMap { store: collector.source_map },
    );
}

struct ExprCollector<'a> {
    db: &'a dyn DefDatabase,
    owner: ExprStoreOwnerId,
    ast_id_map: Arc<AstIdMap>,
    store: ExpressionStoreBuilder,
    source_map: ExpressionStoreSourceMap,
}

impl ExprCollector<'_> {
    fn new(db: &dyn DefDatabase, owner: ExprStoreOwnerId) -> ExprCollector<'_> {
        ExprCollector {
            db,
            owner,
            source_map: ExpressionStoreSourceMap::default(),
            ast_id_map: db.ast_id_map(expander.current_file_id()),
            store: ExpressionStoreBuilder::default(),
        }
    }

    fn collect(&mut self, expr: Option<ast::Expr>) -> ExprId {
        self.with_label_rib(RibKind::Closure, |this| this.collect_expr_opt(expr))
    }
}
