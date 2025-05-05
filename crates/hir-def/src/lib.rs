//! `hir_def` crate contains everything between PDF object expansion and type
//! inference.
//!
//! It defines various items (objects, streams, pages) which comprises a PDF document.
//!
//! Note that `hir_def` is a work in progress, so not all of the above is
//! actually true.

pub mod item_tree;
pub mod name;
