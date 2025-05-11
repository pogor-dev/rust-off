pub mod file_item_tree_id;
pub mod item_tree_data;
pub mod item_tree_node;

use item_tree_data::IndirectObject;
use la_arena::{Arena, Idx};
use smallvec::SmallVec;
use span::FileAstId;
use std::ops::Index;
use syntax::ast;

use file_item_tree_id::FileItemTreeId;
use item_tree_node::ItemTreeNode;

/// The item tree of a source file.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct ItemTree {
    top_level: SmallVec<[PdfItem; 1]>,
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

macro_rules! pdf_items {
    ( $( $typ:ident in $fld:ident -> $ast:ty ),+ $(,)? ) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub enum PdfItem {
            $(
                $typ(FileItemTreeId<$typ>),
            )+
        }

        impl PdfItem {
            pub fn ast_id(&self, tree: &ItemTree) -> FileAstId<ast::Item> {
                match self {
                    $(PdfItem::$typ(it) => tree[it.index()].ast_id().upcast()),+
                }
            }
        }

        $(
            impl From<FileItemTreeId<$typ>> for PdfItem {
                fn from(id: FileItemTreeId<$typ>) -> PdfItem {
                    PdfItem::$typ(id)
                }
            }
        )+

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

pdf_items! {
    IndirectObject in indirect_objects -> ast::IndirectObjectExpr,
}
