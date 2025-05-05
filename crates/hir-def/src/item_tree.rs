use la_arena::Arena;
use smallvec::SmallVec;
use syntax::ast;

use crate::name::Name;

/// The item tree of a source file.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct ItemTree {
    top_level: SmallVec<[ModItem; 1]>,

    data: Option<Box<ItemTreeData>>,
}

#[derive(Default, Debug, Eq, PartialEq)]
struct ItemTreeData {
    objects: Arena<IndirectObject>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IndirectObject {
    pub name: Name,
    pub ast_id: FileAstId<ast::IndirectObjectExpr>,
}
