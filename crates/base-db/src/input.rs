//! This module specifies the input to pdf-analyzer. In some sense, this is
//! **the** most important module, because all other fancy stuff is strictly
//! derived from this input.
//!
//! Note that neither this module, nor any other part of the analyzer's core do
//! actual IO. See `vfs` and `project_model` in the `pdf-analyzer` crate for how
//! actual IO is done and lowered to input.

use vfs::{file_set::FileSet, AnchoredPath, FileId, VfsPath};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceRootId(pub u32);

/// Files are grouped into source roots. A source root is a directory on the
/// file systems which is watched for changes.
/// Source roots *might* be nested: in this case, a file belongs to
/// the nearest enclosing source root. Paths to files are always relative to a
/// source root, and the analyzer does not know the root path of the source root at
/// all. So, a file from one source root can't refer to a file in another source
/// root by path.
///
/// ISO `32000-2:2008`, Section 7.3.8 Stream Objects, describes that
/// beginning with PDF 1.2, the bytes may be contained in an external file.
/// So, there is a possibility that the PDF file is not alone
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceRoot {
    file_set: FileSet,
}

impl SourceRoot {
    pub fn path_for_file(&self, file: &FileId) -> Option<&VfsPath> {
        self.file_set.path_for_file(file)
    }

    pub fn file_for_path(&self, path: &VfsPath) -> Option<&FileId> {
        self.file_set.file_for_path(path)
    }

    pub fn resolve_path(&self, path: AnchoredPath<'_>) -> Option<FileId> {
        self.file_set.resolve_path(path)
    }

    pub fn iter(&self) -> impl Iterator<Item = FileId> + '_ {
        self.file_set.iter()
    }
}
