use std::hash::BuildHasherDefault;

use dashmap::{DashMap, Entry};
use rustc_hash::FxHasher;
use salsa::Setter;
use triomphe::Arc;

use super::source_db::SourceDatabase;

#[derive(Debug, Default)]
pub(super) struct Files {
    files: Arc<DashMap<vfs::FileId, FileText, BuildHasherDefault<FxHasher>>>,
}

impl Files {
    pub(super) fn file_text(&self, file_id: vfs::FileId) -> FileText {
        match self.files.get(&file_id) {
            Some(text) => *text,
            None => {
                panic!("Unable to fetch file text for `vfs::FileId`: {file_id:?}; this is a bug")
            }
        }
    }

    pub(super) fn set_file_text(&self, db: &mut dyn SourceDatabase, file_id: vfs::FileId, text: &[u8]) {
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

#[salsa_macros::input(debug)]
pub struct FileText {
    pub text: Arc<[u8]>,
    pub file_id: vfs::FileId,
}

#[salsa_macros::interned(no_lifetime, debug)]
pub struct FileId {
    pub file_id: vfs::FileId,
}
