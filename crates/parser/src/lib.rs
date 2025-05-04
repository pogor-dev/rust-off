//! The Pdf parser.
//!
//! NOTE: The crate is undergoing refactors, don't believe everything the docs
//! say :-)
//!
//! The parser doesn't know about concrete representation of tokens and syntax
//! trees. Abstract [`TokenSource`] and [`TreeSink`] traits are used instead. As
//! a consequence, this crate does not contain a lexer.
//!
//! The [`Parser`] struct from the [`parser`] module is a cursor into the
//! sequence of tokens.  Parsing routines use [`Parser`] to inspect current
//! state and advance the parsing.
//!
//! The actual parsing happens in the [`grammar`] module.
//!
//! Tests for this crate live in the `syntax` crate.
//!
//! [`Parser`]: crate::parser::Parser

#![cfg_attr(feature = "in-rust-tree", feature(private))]

#[cfg(not(feature = "in-rust-tree"))]
extern crate lexer;
#[cfg(feature = "in-rust-tree")]
extern crate lexer;

mod event;
mod grammar;
mod input;
mod lexed_str;
mod output;
mod parser;
mod shortcuts;
mod syntax_kind;
#[cfg(test)]
mod tests;
mod token_set;

pub(crate) use token_set::TokenSet;

pub use edition::Edition;

pub use crate::{
    input::Input,
    lexed_str::LexedStr,
    output::{Output, Step},
    shortcuts::StrStep,
    syntax_kind::SyntaxKind,
};

/// Parse the whole of the input as a given syntactic construct.
///
/// This covers two main use-cases:
///
///   * Parsing a PDF file.
///   * Parsing a result of stream decode expansion.
///
/// [`TopEntryPoint::parse`] makes a guarantee that
///   * all input is consumed
///   * the result is a valid tree (there's one root node)
#[derive(Debug)]
pub enum TopEntryPoint {
    SourceFile,
    // StreamStmts,
    // Pattern,
    // Type,
    Expr,
}

impl TopEntryPoint {
    pub fn parse(&self, input: &Input, edition: Edition) -> Output {
        let _p = tracing::info_span!("TopEntryPoint::parse", ?self).entered();
        let entry_point: fn(&'_ mut parser::Parser<'_>) = match self {
            TopEntryPoint::SourceFile => grammar::entry::top::SOURCE_FILE,
            // TopEntryPoint::StreamStmts => grammar::entry::top::macro_stmts,
            // TopEntryPoint::Pattern => grammar::entry::top::pattern,
            // TopEntryPoint::Type => grammar::entry::top::type_,
            TopEntryPoint::Expr => grammar::entry::top::expr,
        };
        let mut p = parser::Parser::new(input, edition);
        entry_point(&mut p);
        let events = p.finish();
        let res = event::process(events);

        if cfg!(debug_assertions) {
            let mut depth = 0;
            let mut first = true;
            for step in res.iter() {
                assert!(depth > 0 || first);
                first = false;
                match step {
                    Step::Enter { .. } => depth += 1,
                    Step::Exit { .. } => depth -= 1,
                    Step::Token { .. } | Step::Error { .. } => (),
                }
            }
            assert!(!first, "no tree at all");
            assert_eq!(depth, 0, "unbalanced tree");
        }

        res
    }
}
