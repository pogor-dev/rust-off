use span::FileAstId;
use syntax::ast::{self};

use crate::name::Name;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IndirectObject {
    pub name: Name,
    pub ast_id: FileAstId<ast::IndirectObjectExpr>,
}
