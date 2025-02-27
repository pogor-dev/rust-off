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

#![cfg_attr(feature = "in-rust-tree", feature(pdfc_private))]

#[cfg(not(feature = "in-rust-tree"))]
extern crate pdfc_lexer;
#[cfg(feature = "in-rust-tree")]
extern crate pdfc_lexer;

mod lexed_str;
mod syntax_kind;

pub use edition::Edition;

pub use crate::{lexed_str::LexedStr, syntax_kind::SyntaxKind};
