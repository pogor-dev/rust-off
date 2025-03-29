//! Defines `Body`: a lowered representation of functions, statics and
//! consts.

use crate::expr_store::ExpressionStore;

/// The body of an item (function, const etc.).
#[derive(Debug, Eq, PartialEq)]
pub struct Body {
    pub store: ExpressionStore,
    /// The patterns for the function's parameters. While the parameter types are
    /// part of the function signature, the patterns are not (they don't change
    /// the external type of the function).
    ///
    /// If this `Body` is for the body of a constant, this will just be
    /// empty.
    pub params: Box<[PatId]>,
    pub self_param: Option<BindingId>,
    /// The `ExprId` of the actual body expression.
    pub body_expr: ExprId,
}

impl ops::Deref for Body {
    type Target = ExpressionStore;

    fn deref(&self) -> &Self::Target {
        &self.store
    }
}
