//! Things to wrap other things in file ids.

use span::{EditionedFileId, HirFileId};

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
pub type InFile<T> = InFileWrapper<HirFileId, T>;
pub type InRealFile<T> = InFileWrapper<EditionedFileId, T>;

impl<FileKind, T> InFileWrapper<FileKind, T> {
    pub fn new(file_id: FileKind, value: T) -> Self {
        Self { file_id, value }
    }

    pub fn map<F: FnOnce(T) -> U, U>(self, f: F) -> InFileWrapper<FileKind, U> {
        InFileWrapper::new(self.file_id, f(self.value))
    }
}
