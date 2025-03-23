//! This module implements syntax validation that the parser doesn't handle.
//!
//! A failed validation emits a diagnostic.

use crate::{match_ast, SyntaxError, SyntaxNode};

#[allow(unused_variables)]
pub(crate) fn validate(root: &SyntaxNode, errors: &mut Vec<SyntaxError>) {
    let _p = tracing::info_span!("parser::validate").entered();

    for node in root.descendants() {
        match_ast! {
            match node {
                _ => (),
            }
        }
    }
}
