//! Things to wrap other things in file ids.

use base_db::RootQueryDb;
use pdfc_syntax::AstPtr;
use span::{AstIdNode, FileAstId, FileId, TextRange};

/// `InFile<T>` stores a value of `T` inside a particular file/syntax tree.
///
/// Typical usages are:
///
/// * `InFile<SyntaxNode>` -- syntax node in a file
/// * `InFile<ast::FnDef>` -- ast node in a file
/// * `InFile<TextSize>` -- offset in a file
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct InFileWrapper<FileKind, T> {
    pub file_id: FileKind,
    pub value: T,
}
pub type InRealFile<T> = InFileWrapper<FileId, T>;

/// `AstId` points to an AST node in any file.
///
/// It is stable across reparses, and can be used as salsa key/value.
pub type AstId<N> = crate::InRealFile<FileAstId<N>>;

impl<N: AstIdNode> AstId<N> {
    pub fn to_node(&self, db: &dyn RootQueryDb) -> N {
        self.to_ptr(db).to_node(&db.parse_or_expand(self.file_id))
    }
    pub fn to_range(&self, db: &dyn RootQueryDb) -> TextRange {
        self.to_ptr(db).text_range()
    }
    pub fn to_in_file_node(&self, db: &dyn RootQueryDb) -> crate::InRealFile<N> {
        crate::InRealFile::new(self.file_id, self.to_ptr(db).to_node(&db.parse_or_expand(self.file_id)))
    }
    pub fn to_ptr(&self, db: &dyn RootQueryDb) -> AstPtr<N> {
        db.ast_id_map(self.file_id).get(self.value)
    }
}

impl<FileKind, T> InFileWrapper<FileKind, T> {
    pub fn new(file_id: FileKind, value: T) -> Self {
        Self { file_id, value }
    }

    pub fn map<F: FnOnce(T) -> U, U>(self, f: F) -> InFileWrapper<FileKind, U> {
        InFileWrapper::new(self.file_id, f(self.value))
    }
}
