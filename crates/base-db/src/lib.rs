//! base_db defines basic database traits. The concrete DB is defined by ide.
// TODO: Rename this crate, base db is non descriptive
mod change;
mod input;

pub use crate::{
    change::FileChange,
    input::{SourceRoot, SourceRootId},
};

use pdfc_syntax::{ast, Parse, SyntaxError};
pub use query_group::{self};
use salsa::Durability;
use triomphe::Arc;
use vfs::{AnchoredPath, FileId};

#[salsa::interned(no_lifetime)]
pub struct EditionedFileId {
    pub editioned_file_id: span::EditionedFileId,
}

impl EditionedFileId {
    pub fn file_id(&self, db: &dyn salsa::Database) -> vfs::FileId {
        let id = self.editioned_file_id(db);
        id.file_id()
    }

    fn unpack(&self, db: &dyn salsa::Database) -> (vfs::FileId, span::Edition) {
        let id = self.editioned_file_id(db);
        (id.file_id(), id.edition())
    }
}

#[salsa::input]
pub struct FileText {
    pub text: Arc<Vec<u8>>,
    pub file_id: vfs::FileId,
}

#[salsa::input]
pub struct FileSourceRootInput {
    pub source_root_id: SourceRootId,
}

#[salsa::input]
pub struct SourceRootInput {
    pub source_root: Arc<SourceRoot>,
}

// Database which stores all significant input facts: source code and project
/// model. Everything else in rust-analyzer is derived from these queries.
#[query_group::query_group]
pub trait RootQueryDb: SourceDatabase + salsa::Database {
    /// Parses the file into the syntax tree.
    #[salsa::invoke_actual(parse)]
    #[salsa::lru(128)]
    fn parse(&self, file_id: EditionedFileId) -> Parse<ast::PdfDocument>;

    /// Returns the set of errors obtained from parsing the file including validation errors.
    fn parse_errors(&self, file_id: EditionedFileId) -> Option<Arc<[SyntaxError]>>;
}

#[salsa::db]
pub trait SourceDatabase: salsa::Database {
    /// Text of the file.
    fn file_text(&self, file_id: vfs::FileId) -> FileText;

    fn set_file_text(&mut self, file_id: vfs::FileId, text: &str);

    fn set_file_text_with_durability(&mut self, file_id: vfs::FileId, text: &str, durability: Durability);

    /// Contents of the source root.
    fn source_root(&self, id: SourceRootId) -> SourceRootInput;

    fn file_source_root(&self, id: vfs::FileId) -> FileSourceRootInput;

    fn set_file_source_root_with_durability(&mut self, id: vfs::FileId, source_root_id: SourceRootId, durability: Durability);

    /// Source root of the file.
    fn set_source_root_with_durability(&mut self, source_root_id: SourceRootId, source_root: Arc<SourceRoot>, durability: Durability);

    fn resolve_path(&self, path: AnchoredPath<'_>) -> Option<FileId> {
        // FIXME: this *somehow* should be platform agnostic...
        let source_root = self.file_source_root(path.anchor);
        let source_root = self.source_root(source_root.source_root_id(self));
        source_root.source_root(self).resolve_path(path)
    }
}

fn parse(db: &dyn RootQueryDb, file_id: EditionedFileId) -> Parse<ast::PdfDocument> {
    let _p = tracing::info_span!("parse", ?file_id).entered();
    let (file_id, edition) = file_id.unpack(db.as_dyn_database());
    let text = db.file_text(file_id).text(db);
    ast::PdfDocument::parse(&text, edition)
}

fn parse_errors(db: &dyn RootQueryDb, file_id: EditionedFileId) -> Option<Arc<[SyntaxError]>> {
    let errors = db.parse(file_id).errors();
    match &*errors {
        [] => None,
        [..] => Some(errors.into()),
    }
}
