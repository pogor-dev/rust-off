//! `hir_def` crate contains everything between macro expansion and type
//! inference.
//!
//! It defines various items (structs, enums, traits) which comprises Rust code,
//! as well as an algorithm for resolving paths to such entities.
//!
//! Note that `hir_def` is a work in progress, so not all of the above is
//! actually true.

pub mod builtin_type;
pub mod db;
pub mod expr_store;
pub mod hir;
pub mod item_tree;

mod hir_expand;

use base_db::impl_intern_key;
use la_arena::{Arena, Idx, RawIdx};
use pdfc_syntax::{ast, match_ast, SyntaxKind};
use smallvec::SmallVec;
use span::{AstIdNode, Edition, FileAstId, SyntaxContext};
use stdx::impl_from;

use std::{
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    ops::{Index, Range},
    sync::OnceLock,
};

use crate::{
    db::DefDatabase,
    hir_expand::{Intern, Lookup},
    item_tree::{ItemTreeId, ItemTreeNode, Object},
};

#[derive(Debug)]
pub struct ItemLoc<N: ItemTreeNode> {
    pub id: ItemTreeId<N>,
}

impl<N: ItemTreeNode> Clone for ItemLoc<N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<N: ItemTreeNode> Copy for ItemLoc<N> {}

impl<N: ItemTreeNode> PartialEq for ItemLoc<N> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<N: ItemTreeNode> Eq for ItemLoc<N> {}

impl<N: ItemTreeNode> Hash for ItemLoc<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug)]
pub struct AssocItemLoc<N: ItemTreeNode> {
    pub id: ItemTreeId<N>,
}

impl<N: ItemTreeNode> Clone for AssocItemLoc<N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<N: ItemTreeNode> Copy for AssocItemLoc<N> {}

impl<N: ItemTreeNode> PartialEq for AssocItemLoc<N> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<N: ItemTreeNode> Eq for AssocItemLoc<N> {}

impl<N: ItemTreeNode> Hash for AssocItemLoc<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub trait ItemTreeLoc {
    type Container;
    type Id;
    fn item_tree_id(&self) -> ItemTreeId<Self::Id>;
    fn container(&self) -> Self::Container;
}

macro_rules! impl_intern {
    ($id:ident, $loc:ident, $intern:ident, $lookup:ident) => {
        impl_intern_key!($id, $loc);
        impl_intern_lookup!(DefDatabase, $id, $loc, $intern, $lookup);
    };
}

macro_rules! impl_loc {
    ($loc:ident, $id:ident: $id_ty:ident) => {
        impl ItemTreeLoc for $loc {
            type Id = $id_ty;
            fn item_tree_id(&self) -> ItemTreeId<Self::Id> {
                self.$id
            }
        }
    };
}

type ObjectLoc = AssocItemLoc<Object>;
impl_intern!(ObjectId, ObjectLoc, intern_object, lookup_intern_object);
impl_loc!(ObjectLoc, id: Object);

/// The defs which have a body.
#[derive(Debug, PartialOrd, Ord, Clone, Copy, PartialEq, Eq, Hash, salsa::Supertype)]
pub enum DefWithBodyId {
    ObjectId(ObjectId),
    // StreamId(StreamId),
}
impl_from!(ObjectId for DefWithBodyId);

/// The item tree of a source file.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct ItemTree {
    data: Option<Box<ItemTreeData>>,
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
    Object in objects -> ast::IndirectObjectExpr,
}
