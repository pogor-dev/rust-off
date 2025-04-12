//! base_db defines basic database traits. The concrete DB is defined by ide.
// FIXME: Rename this crate, base db is non descriptive

use std::hash::BuildHasherDefault;

use dashmap::{DashMap, mapref::entry::Entry};
use pdfc_syntax::{Parse, SyntaxError, ast};
use rustc_hash::FxHasher;
use salsa::Setter;
use span::Edition;
use triomphe::Arc;

pub use vfs::{AnchoredPath, AnchoredPathBuf, FileId, VfsPath, file_set::FileSet};

#[salsa::interned(no_lifetime)]
pub struct SalsaFileId {
    pub file_id: vfs::FileId,
}

#[derive(Debug, Default)]
pub struct Files {
    files: Arc<DashMap<vfs::FileId, FileText, BuildHasherDefault<FxHasher>>>,
}

impl Files {
    pub fn file_text(&self, file_id: vfs::FileId) -> FileText {
        *self.files.get(&file_id).expect("Unable to fetch file; this is a bug")
    }

    pub fn set_file_text(&self, db: &mut dyn SourceDatabase, file_id: vfs::FileId, text: &[u8]) {
        match self.files.entry(file_id) {
            Entry::Occupied(mut occupied) => {
                occupied.get_mut().set_text(db).to(Arc::from(text));
            }
            Entry::Vacant(vacant) => {
                let text = FileText::new(db, Arc::from(text), file_id);
                vacant.insert(text);
            }
        };
    }
}

#[salsa::input]
pub struct FileText {
    pub text: Arc<[u8]>,
    pub file_id: vfs::FileId,
}

#[salsa::db]
pub trait SourceDatabase: salsa::Database {
    /// Text of the file.
    fn file_text(&self, file_id: vfs::FileId) -> FileText;

    fn set_file_text(&mut self, file_id: vfs::FileId, text: &[u8]);
}

/// Database which stores all significant input facts: source code and project
/// model. Everything else in pdf-analyzer is derived from these queries.
#[salsa::db]
pub trait RootQueryDb: SourceDatabase + salsa::Database {
    /// Parses the file into the syntax tree.
    fn parse(&self, file_id: SalsaFileId) -> Parse<ast::PdfDocument>;

    /// Returns the set of errors obtained from parsing the file including validation errors.
    fn parse_errors(&self, file_id: SalsaFileId) -> Option<&[SyntaxError]>;
}

#[salsa::tracked(lru = 128)]
fn parse(db: &dyn RootQueryDb, file_id: SalsaFileId) -> Parse<ast::PdfDocument> {
    let _p = tracing::info_span!("parse", ?file_id).entered();
    let file_id = file_id.file_id(db);
    let text = db.file_text(file_id).text(db);
    ast::PdfDocument::parse(&text, Edition::CURRENT) // TODO: we need to remove edition as is readed from pdf document
}

#[salsa::tracked(return_ref)]
fn parse_errors(db: &dyn RootQueryDb, file_id: SalsaFileId) -> Option<Box<[SyntaxError]>> {
    let errors = db.parse(file_id).errors();
    match &*errors {
        [] => None,
        [..] => Some(errors.into()),
    }
}
