//! A simplified AST that only contains items.
//!
//! This is the primary IR used throughout `hir_def`. It is the input to the name resolution
//! algorithm, as well as to the queries defined in `adt.rs`, `data.rs`, and most things in
//! `attr.rs`.
//!
//! `ItemTree`s are built per `HirFileId`, from the syntax tree of the parsed file. This means that
//! they are crate-independent: they don't know which `#[cfg]`s are active or which module they
//! belong to, since those concepts don't exist at this level (a single `ItemTree` might be part of
//! multiple crates, or might be included into the same crate twice via `#[path]`).
//!
//! One important purpose of this layer is to provide an "invalidation barrier" for incremental
//! computations: when typing inside an item body, the `ItemTree` of the modified file is typically
//! unaffected, so we don't have to recompute name resolution results or item data (see `data.rs`).
//!
//! The `ItemTree` for the currently open file can be displayed by using the VS Code command
//! "rust-analyzer: Debug ItemTree".
//!
//! Compared to rustc's architecture, `ItemTree` has properties from both rustc's AST and HIR: many
//! syntax-level Rust features are already desugared to simpler forms in the `ItemTree`, but name
//! resolution has not yet been performed. `ItemTree`s are per-file, while rustc's AST and HIR are
//! per-crate, because we are interested in incrementally computing it.
//!
//! The representation of items in the `ItemTree` should generally mirror the surface syntax: it is
//! usually a bad idea to desugar a syntax-level construct to something that is structurally
//! different here. Name resolution needs to be able to process attributes and expand macros
//! (including attribute macros), and having a 1-to-1 mapping between syntax and the `ItemTree`
//! avoids introducing subtle bugs.
//!
//! In general, any item in the `ItemTree` stores its `AstId`, which allows mapping it back to its
//! surface syntax.

use std::{
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    ops::{Index, Range},
    sync::OnceLock,
};

use la_arena::{Arena, Idx, RawIdx};
use pdfc_syntax::{ast, match_ast, SyntaxKind};
use span::{AstIdNode, FileAstId, HirFileId};
use triomphe::Arc;

/// The item tree of a source file.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct ItemTree {
    data: Option<Box<ItemTreeData>>,
}

impl ItemTree {
    fn data(&self) -> &ItemTreeData {
        self.data.as_ref().expect("attempted to access data of empty ItemTree")
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
struct ItemTreeData {
    indirect_objects: Arena<IndirectObject>,
}

/// Trait implemented by all nodes in the item tree.
pub trait ItemTreeNode: Clone {
    type Source: AstIdNode;

    fn ast_id(&self) -> FileAstId<Self::Source>;

    /// Looks up an instance of `Self` in an item tree.
    fn lookup(tree: &ItemTree, index: Idx<Self>) -> &Self;
}

pub struct FileItemTreeId<N>(Idx<N>);

impl<N> FileItemTreeId<N> {
    pub fn range_iter(range: Range<Self>) -> impl Iterator<Item = Self> + Clone {
        (range.start.index().into_raw().into_u32()..range.end.index().into_raw().into_u32())
            .map(RawIdx::from_u32)
            .map(Idx::from_raw)
            .map(Self)
    }
}

impl<N> FileItemTreeId<N> {
    pub fn index(&self) -> Idx<N> {
        self.0
    }
}

impl<N> Clone for FileItemTreeId<N> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<N> Copy for FileItemTreeId<N> {}

impl<N> PartialEq for FileItemTreeId<N> {
    fn eq(&self, other: &FileItemTreeId<N>) -> bool {
        self.0 == other.0
    }
}
impl<N> Eq for FileItemTreeId<N> {}

impl<N> Hash for FileItemTreeId<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<N> fmt::Debug for FileItemTreeId<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Identifies a particular [`ItemTree`].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TreeId {
    file: HirFileId,
}

impl TreeId {
    pub(crate) fn new(file: HirFileId) -> Self {
        Self { file }
    }

    pub fn file_id(self) -> HirFileId {
        self.file
    }
}

#[derive(Debug)]
pub struct ItemTreeId<N> {
    tree: TreeId,
    pub value: FileItemTreeId<N>,
}

impl<N> ItemTreeId<N> {
    pub fn new(tree: TreeId, idx: FileItemTreeId<N>) -> Self {
        Self { tree, value: idx }
    }
}

impl<N> Copy for ItemTreeId<N> {}
impl<N> Clone for ItemTreeId<N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<N> PartialEq for ItemTreeId<N> {
    fn eq(&self, other: &Self) -> bool {
        self.tree == other.tree && self.value == other.value
    }
}

impl<N> Eq for ItemTreeId<N> {}

impl<N> Hash for ItemTreeId<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tree.hash(state);
        self.value.hash(state);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IndirectObject {
    pub ast_id: FileAstId<ast::IndirectObjectExpr>,
}

macro_rules! mod_items {
    ( $( $typ:ident in $fld:ident -> $ast:ty ),+ $(,)? ) => {
        $(
            impl ItemTreeNode for $typ {
                type Source = $ast;

                fn ast_id(&self) -> FileAstId<Self::Source> {
                    self.ast_id
                }

                fn lookup(tree: &ItemTree, index: Idx<Self>) -> &Self {
                    &tree.data().$fld[index]
                }
            }

            impl Index<Idx<$typ>> for ItemTree {
                type Output = $typ;

                fn index(&self, index: Idx<$typ>) -> &Self::Output {
                    &self.data().$fld[index]
                }
            }
        )+
    };
}

mod_items! {
    IndirectObject in indirect_objects -> ast::IndirectObjectExpr,
}
