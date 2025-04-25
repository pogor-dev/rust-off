//! base_db defines basic database traits. The concrete DB is defined by ide.
// FIXME: Rename this crate, base db is non descriptive
mod change;
mod input;

use std::hash::BuildHasherDefault;

use dashmap::{DashMap, mapref::entry::Entry};
use pdfc_syntax::{Parse, SyntaxError, SyntaxNode, ast};
use rustc_hash::FxHasher;
use salsa::{Durability, Setter};
use span::{AstIdMap, Edition};
use triomphe::Arc;

pub use vfs::{AnchoredPath, AnchoredPathBuf, FileId, VfsPath, file_set::FileSet};

pub use crate::{
    change::FileChange,
    input::{SourceRoot, SourceRootId},
};

#[salsa::interned(no_lifetime)]
pub struct SalsaFileId {
    pub file_id: vfs::FileId,
}

#[derive(Debug, Default)]
pub struct Files {
    files: Arc<DashMap<vfs::FileId, FileText, BuildHasherDefault<FxHasher>>>,
    source_roots: Arc<DashMap<SourceRootId, SourceRootInput, BuildHasherDefault<FxHasher>>>,
    file_source_roots: Arc<DashMap<vfs::FileId, FileSourceRootInput, BuildHasherDefault<FxHasher>>>,
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

    pub fn set_file_text_with_durability(&self, db: &mut dyn SourceDatabase, file_id: vfs::FileId, text: &[u8], durability: Durability) {
        match self.files.entry(file_id) {
            Entry::Occupied(mut occupied) => {
                occupied.get_mut().set_text(db).with_durability(durability).to(Arc::from(text));
            }
            Entry::Vacant(vacant) => {
                let text = FileText::builder(Arc::from(text), file_id).durability(durability).new(db);
                vacant.insert(text);
            }
        };
    }

    /// Source root of the file.
    pub fn source_root(&self, source_root_id: SourceRootId) -> SourceRootInput {
        let source_root = self.source_roots.get(&source_root_id).expect("Unable to fetch source root id; this is a bug");

        *source_root
    }

    pub fn set_source_root_with_durability(
        &self,
        db: &mut dyn SourceDatabase,
        source_root_id: SourceRootId,
        source_root: Arc<SourceRoot>,
        durability: Durability,
    ) {
        match self.source_roots.entry(source_root_id) {
            Entry::Occupied(mut occupied) => {
                occupied.get_mut().set_source_root(db).with_durability(durability).to(source_root);
            }
            Entry::Vacant(vacant) => {
                let source_root = SourceRootInput::builder(source_root).durability(durability).new(db);
                vacant.insert(source_root);
            }
        };
    }

    pub fn file_source_root(&self, id: vfs::FileId) -> FileSourceRootInput {
        let file_source_root = self.file_source_roots.get(&id).expect("Unable to fetch FileSourceRootInput; this is a bug");
        *file_source_root
    }

    pub fn set_file_source_root_with_durability(&self, db: &mut dyn SourceDatabase, id: vfs::FileId, source_root_id: SourceRootId, durability: Durability) {
        match self.file_source_roots.entry(id) {
            Entry::Occupied(mut occupied) => {
                occupied.get_mut().set_source_root_id(db).with_durability(durability).to(source_root_id);
            }
            Entry::Vacant(vacant) => {
                let file_source_root = FileSourceRootInput::builder(source_root_id).durability(durability).new(db);
                vacant.insert(file_source_root);
            }
        };
    }
}

#[salsa::input]
pub struct FileText {
    pub text: Arc<[u8]>,
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

#[salsa::db]
pub trait SourceDatabase: salsa::Database {
    /// Text of the file.
    fn file_text(&self, file_id: vfs::FileId) -> FileText;

    fn set_file_text(&mut self, file_id: vfs::FileId, text: &[u8]);

    fn set_file_text_with_durability(&mut self, file_id: vfs::FileId, text: &[u8], durability: Durability);

    /// Contents of the source root.
    fn source_root(&self, id: SourceRootId) -> SourceRootInput;

    fn file_source_root(&self, id: vfs::FileId) -> FileSourceRootInput;

    fn set_file_source_root_with_durability(&mut self, id: vfs::FileId, source_root_id: SourceRootId, durability: Durability);

    /// Source root of the file.
    fn set_source_root_with_durability(&mut self, source_root_id: SourceRootId, source_root: Arc<SourceRoot>, durability: Durability);
}

/// Database which stores all significant input facts: source code and project
/// model. Everything else in pdf-analyzer is derived from these queries.
#[salsa::db]
pub trait RootQueryDb: SourceDatabase + salsa::Database {
    /// Parses the file into the syntax tree.
    fn parse(&self, file_id: SalsaFileId) -> Parse<ast::PdfDocument>;

    // TODO: this was from macro expand
    // TODO: FileId vs SalsaFileId
    fn parse_or_expand(&self, file_id: FileId) -> SyntaxNode;

    // TODO: this was from macro expand
    fn ast_id_map(&self, file_id: FileId) -> Arc<AstIdMap>;
}

#[salsa::db]
impl<DB> RootQueryDb for DB
where
    DB: SourceDatabase + salsa::Database,
{
    fn parse(&self, file_id: SalsaFileId) -> Parse<ast::PdfDocument> {
        let _p = tracing::info_span!("parse", ?file_id).entered();
        let file_id = file_id.file_id(self);
        let text = self.file_text(file_id).text(self);
        ast::PdfDocument::parse(&text, Edition::CURRENT) // TODO: we need to remove edition as is readed from pdf document
    }

    fn parse_or_expand(&self, file_id: FileId) -> SyntaxNode {
        // Implement the logic for parse_or_expand here
        unimplemented!("parse_or_expand is not yet implemented")
    }

    fn ast_id_map(&self, file_id: FileId) -> Arc<AstIdMap> {
        // Implement the logic for ast_id_map here
        unimplemented!("ast_id_map is not yet implemented")
    }
}
