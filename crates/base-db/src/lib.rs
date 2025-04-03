//! base_db defines basic database traits. The concrete DB is defined by ide.
// TODO: Rename this crate, base db is non descriptive
mod change;
mod input;

use std::hash::BuildHasherDefault;

pub use crate::{
    change::FileChange,
    input::{SourceRoot, SourceRootId},
};
use dashmap::{mapref::entry::Entry, DashMap};
use rustc_hash::FxHasher;
pub use semver::Version;

use pdfc_syntax::{ast, Parse, SyntaxError};
pub use query_group::{self};
use salsa::{Durability, Setter};
use triomphe::Arc;
pub use vfs::{file_set::FileSet, AnchoredPath, AnchoredPathBuf, FileId, VfsPath};

#[macro_export]
macro_rules! impl_intern_key {
    ($id:ident, $loc:ident) => {
        #[salsa::interned(no_debug, no_lifetime)]
        pub struct $id {
            pub loc: $loc,
        }

        // If we derive this salsa prints the values recursively, and this causes us to blow.
        impl ::std::fmt::Debug for $id {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_tuple(stringify!($id)).field(&format_args!("{:04x}", self.0.as_u32())).finish()
            }
        }
    };
}

pub trait Upcast<T: ?Sized> {
    fn upcast(&self) -> &T;
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
        let files = Arc::clone(&self.files);
        match files.entry(file_id) {
            Entry::Occupied(mut occupied) => {
                occupied.get_mut().set_text(db).to(Arc::from(text.to_vec()));
            }
            Entry::Vacant(vacant) => {
                let text = FileText::new(db, Arc::from(text.to_vec()), file_id);
                vacant.insert(text);
            }
        };
    }

    pub fn set_file_text_with_durability(&self, db: &mut dyn SourceDatabase, file_id: vfs::FileId, text: &[u8], durability: Durability) {
        let files = Arc::clone(&self.files);
        match files.entry(file_id) {
            Entry::Occupied(mut occupied) => {
                occupied.get_mut().set_text(db).with_durability(durability).to(Arc::from(text.to_vec()));
            }
            Entry::Vacant(vacant) => {
                let text = FileText::builder(Arc::from(text.to_vec()), file_id).durability(durability).new(db);
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
        let source_roots = Arc::clone(&self.source_roots);
        match source_roots.entry(source_root_id) {
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
        let file_source_roots = Arc::clone(&self.file_source_roots);
        // let db = self;
        match file_source_roots.entry(id) {
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
/// model. Everything else in pdf-analyzer is derived from these queries.
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

    fn set_file_text(&mut self, file_id: vfs::FileId, text: &[u8]);

    fn set_file_text_with_durability(&mut self, file_id: vfs::FileId, text: &[u8], durability: Durability);

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
