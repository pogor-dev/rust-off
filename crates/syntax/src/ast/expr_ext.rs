//! Various extension methods to ast Expr Nodes, which are hard to code-generate.
//!
//! These methods should only do simple, shallow tasks related to the syntax of the node itself.

use crate::{
    SyntaxToken, T,
    ast::{self},
    ast::{AstNode, AstToken},
};

use super::{Expr, IndirectObjectExpr, support};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LiteralKind {
    LiteralString(ast::LiteralString),
    HexString(ast::HexString),
    Name(ast::Name),
    IntNumber(ast::IntNumber),
    RealNumber(ast::RealNumber),
    Bool(bool),
    Null,
}

impl ast::Literal {
    pub fn token(&self) -> SyntaxToken {
        self.syntax()
            .children_with_tokens()
            .find(|e| !e.kind().is_trivia())
            .and_then(|e| e.into_token())
            .unwrap()
    }

    pub fn kind(&self) -> LiteralKind {
        let token = self.token();

        if let Some(t) = ast::IntNumber::cast(token.clone()) {
            return LiteralKind::IntNumber(t);
        }
        if let Some(t) = ast::RealNumber::cast(token.clone()) {
            return LiteralKind::RealNumber(t);
        }
        if let Some(t) = ast::LiteralString::cast(token.clone()) {
            return LiteralKind::LiteralString(t);
        }
        if let Some(t) = ast::HexString::cast(token.clone()) {
            return LiteralKind::HexString(t);
        }
        if let Some(t) = ast::Name::cast(token.clone()) {
            return LiteralKind::Name(t);
        }

        match token.kind() {
            T![true] => LiteralKind::Bool(true),
            T![false] => LiteralKind::Bool(false),
            T![null] => LiteralKind::Null,
            _ => unreachable!(),
        }
    }
}

impl IndirectObjectExpr {
    pub fn body(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}
