//! This module describes hir-level representation of expressions.
//!
//! This representation is:
//!
//! 1. Identity-based. Each expression has an `id`, so we can distinguish
//!    between different `1` in `1 + 1`.
//! 2. Independent of syntax. Though syntactic provenance information can be
//!    attached separately via id-based side map.
//! 3. Unresolved. Paths are stored as sequences of names, and not as defs the
//!    names refer to.
//! 4. Desugared. There's no `if let`.
//!
//! See also a neighboring `body` module.

use intern::Symbol;
use la_arena::Idx;
use pdfc_syntax::ast;
use rustc_apfloat::ieee::{Half as f16, Quad as f128};

pub type ExprId = Idx<Expr>;

// We leave float values as a string to avoid double rounding.
// For PartialEq, string comparison should work, as ordering is not important
// https://github.com/rust-lang/rust-analyzer/issues/12380#issuecomment-1137284360
// TODO: check if this is still the case
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FloatTypeWrapper(Symbol);

// TODO: Use builtin types once stabilised.
impl FloatTypeWrapper {
    pub fn new(sym: Symbol) -> Self {
        Self(sym)
    }

    pub fn to_f128(&self) -> f128 {
        self.0.as_str().parse().unwrap_or_default()
    }

    pub fn to_f64(&self) -> f64 {
        self.0.as_str().parse().unwrap_or_default()
    }

    pub fn to_f32(&self) -> f32 {
        self.0.as_str().parse().unwrap_or_default()
    }

    pub fn to_f16(&self) -> f16 {
        self.0.as_str().parse().unwrap_or_default()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal {
    LiteralString(Symbol),
    HexString(Symbol),
    Name(Symbol),
    Bool(bool),
    Int(i128), // TODO: check if 128 is ok
    // Here we are using a wrapper around float because float primitives do not implement Eq, so they
    // could not be used directly here, to understand how the wrapper works go to definition of
    // FloatTypeWrapper
    Real(FloatTypeWrapper),
    Null(Symbol),
}

impl From<ast::LiteralKind> for Literal {
    fn from(ast_lit_kind: ast::LiteralKind) -> Self {
        use ast::LiteralKind;
        match ast_lit_kind {
            LiteralKind::IntNumber(lit) => Literal::Int(lit.value().unwrap_or(0) as i128),
            LiteralKind::RealNumber(lit) => Literal::Real(FloatTypeWrapper::new(Symbol::intern(&lit.value_string()))),
            LiteralKind::LiteralString(s) => {
                let text = s.value().map_or_else(|_| Symbol::empty(), |it| Symbol::intern(&it));
                Literal::LiteralString(text)
            }
            LiteralKind::HexString(s) => {
                let text = s.value().map_or_else(|_| Symbol::empty(), |it| Symbol::intern(&it));
                Literal::HexString(text)
            }
            LiteralKind::Name(s) => {
                let text = s.value().map_or_else(|_| Symbol::empty(), |it| Symbol::intern(&it));
                Literal::Name(text)
            }
            LiteralKind::Bool(val) => Literal::Bool(val),
            LiteralKind::Null => Literal::Null(Symbol::intern("null")),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {}
