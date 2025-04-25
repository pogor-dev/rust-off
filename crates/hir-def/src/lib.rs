//! `hir_def` crate contains everything between macro expansion and type
//! inference.
//!
//! It defines various items (structs, enums, traits) which comprises Rust code,
//! as well as an algorithm for resolving paths to such entities.
//!
//! Note that `hir_def` is a work in progress, so not all of the above is
//! actually true.

use hir_expand::AstId;
use pdfc_syntax::ast;
use span::FileId;

pub mod builtin_type;
pub mod db;
pub mod item_tree;
pub mod nameres;

#[cfg(test)]
mod test_db;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PdfDocumentId {
    pub file_id: FileId,
}
