//! Database used for testing `hir`.

use std::{fmt, sync::Mutex};

use base_db::{EditionedFileId, FileSourceRootInput, FileText, Files, SourceDatabase, SourceRoot, SourceRootId, SourceRootInput};
use hir_def::{PdfDocumentId, nameres::pdf_document_def_map};
use salsa::Durability;
use span::{Edition, FileId};
use triomphe::Arc;

#[salsa::db]
#[derive(Default, Clone)]
pub(crate) struct TestDB {
    storage: salsa::Storage<Self>,
    files: Arc<Files>,
    events: Arc<Mutex<Option<Vec<salsa::Event>>>>,
}

impl fmt::Debug for TestDB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TestDB").finish()
    }
}

#[salsa::db]
impl SourceDatabase for TestDB {
    fn file_text(&self, file_id: base_db::FileId) -> FileText {
        self.files.file_text(file_id)
    }

    fn set_file_text(&mut self, file_id: base_db::FileId, text: &[u8]) {
        let files = Arc::clone(&self.files);
        files.set_file_text(self, file_id, text);
    }

    fn set_file_text_with_durability(&mut self, file_id: base_db::FileId, text: &[u8], durability: Durability) {
        let files = Arc::clone(&self.files);
        files.set_file_text_with_durability(self, file_id, text, durability);
    }

    /// Source root of the file.
    fn source_root(&self, source_root_id: SourceRootId) -> SourceRootInput {
        self.files.source_root(source_root_id)
    }

    fn set_source_root_with_durability(&mut self, source_root_id: SourceRootId, source_root: Arc<SourceRoot>, durability: Durability) {
        let files = Arc::clone(&self.files);
        files.set_source_root_with_durability(self, source_root_id, source_root, durability);
    }

    fn file_source_root(&self, id: base_db::FileId) -> FileSourceRootInput {
        self.files.file_source_root(id)
    }

    fn set_file_source_root_with_durability(&mut self, id: base_db::FileId, source_root_id: SourceRootId, durability: Durability) {
        let files = Arc::clone(&self.files);
        files.set_file_source_root_with_durability(self, id, source_root_id, durability);
    }
}

#[salsa::db]
impl salsa::Database for TestDB {
    fn salsa_event(&self, event: &dyn std::ops::Fn() -> salsa::Event) {
        let mut events = self.events.lock().unwrap();
        if let Some(events) = &mut *events {
            let event = event();
            events.push(event);
        }
    }
}

impl TestDB {
    pub(crate) fn document_for_file_opt(&self, file_id: impl Into<FileId>) -> Option<PdfDocumentId> {
        let file_id = file_id.into();
        let def_map = pdf_document_def_map(self, EditionedFileId::new(self, file_id, Edition::CURRENT));
        None
    }

    pub(crate) fn document_for_file(&self, file_id: impl Into<FileId>) -> PdfDocumentId {
        self.document_for_file_opt(file_id.into()).unwrap()
    }
}
