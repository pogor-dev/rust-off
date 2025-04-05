//! `AstIdMap` allows to create stable IDs for "large" syntax nodes like items
//! and macro calls.
//!
//! Specifically, it enumerates all items in a file and uses position of a an
//! item as an ID. That way, id's don't change unless the set of items itself
//! changes.

use std::{
    any::type_name,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use pdfc_syntax::{ast, AstNode, SyntaxNode};

/// See crates\hir-expand\src\ast_id_map.rs
/// This is a type erased FileAstId.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErasedFileAstId(u32);

impl ErasedFileAstId {
    pub const fn into_raw(self) -> u32 {
        self.0
    }
    pub const fn from_raw(u32: u32) -> Self {
        Self(u32)
    }
}

impl fmt::Display for ErasedFileAstId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl fmt::Debug for ErasedFileAstId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// `AstId` points to an AST node in a specific file.
pub struct FileAstId<N: AstIdNode> {
    raw: ErasedFileAstId,
    covariant: PhantomData<fn() -> N>,
}

impl<N: AstIdNode> Clone for FileAstId<N> {
    fn clone(&self) -> FileAstId<N> {
        *self
    }
}
impl<N: AstIdNode> Copy for FileAstId<N> {}

impl<N: AstIdNode> PartialEq for FileAstId<N> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}
impl<N: AstIdNode> Eq for FileAstId<N> {}
impl<N: AstIdNode> Hash for FileAstId<N> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.raw.hash(hasher);
    }
}

impl<N: AstIdNode> fmt::Debug for FileAstId<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileAstId::<{}>({})", type_name::<N>(), self.raw)
    }
}

impl<N: AstIdNode> FileAstId<N> {
    // Can't make this a From implementation because of coherence
    pub fn upcast<M: AstIdNode>(self) -> FileAstId<M>
    where
        N: Into<M>,
    {
        FileAstId {
            raw: self.raw,
            covariant: PhantomData,
        }
    }

    pub fn erase(self) -> ErasedFileAstId {
        self.raw
    }
}

pub trait AstIdNode: AstNode {}
macro_rules! register_ast_id_node {
    (impl AstIdNode for $($ident:ident),+ ) => {
        $(
            impl AstIdNode for ast::$ident {}
        )+
        fn should_alloc_id(kind: pdfc_syntax::SyntaxKind) -> bool {
            $(
                ast::$ident::can_cast(kind)
            )||+
        }
    };
}
register_ast_id_node! {
    impl AstIdNode for IndirectObjectExpr
    // Item, AnyHasGenericParams,
    //     Adt,
    //         Enum,
    //             Variant,
    //         Struct,
    //         Union,
    //     AssocItem,
    //         Const,
    //         Fn,
    //         MacroCall,
    //         TypeAlias,
    //     ExternBlock,
    //     ExternCrate,
    //     Impl,
    //     Macro,
    //         MacroDef,
    //         MacroRules,
    //     Module,
    //     Static,
    //     Trait,
    //     TraitAlias,
    //     Use,
    // BlockExpr, ConstArg
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum TreeOrder {
    BreadthFirst,
    DepthFirst,
}

/// Walks the subtree in bdfs order, calling `f` for each node. What is bdfs
/// order? It is a mix of breadth-first and depth first orders. Nodes for which
/// `f` returns [`TreeOrder::BreadthFirst`] are visited breadth-first, all the other nodes are explored
/// [`TreeOrder::DepthFirst`].
///
/// In other words, the size of the bfs queue is bound by the number of "true"
/// nodes.
fn bdfs(node: &SyntaxNode, mut f: impl FnMut(SyntaxNode) -> TreeOrder) {
    let mut curr_layer = vec![node.clone()];
    let mut next_layer = vec![];
    while !curr_layer.is_empty() {
        curr_layer.drain(..).for_each(|node| {
            let mut preorder = node.preorder();
            while let Some(event) = preorder.next() {
                match event {
                    pdfc_syntax::WalkEvent::Enter(node) => {
                        if f(node.clone()) == TreeOrder::BreadthFirst {
                            next_layer.extend(node.children());
                            preorder.skip_subtree();
                        }
                    }
                    pdfc_syntax::WalkEvent::Leave(_) => {}
                }
            }
        });
        std::mem::swap(&mut curr_layer, &mut next_layer);
    }
}
