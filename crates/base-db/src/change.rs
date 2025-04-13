//! Defines a unit of change that can applied to the database to get the next
//! state. Changes are transactional.

use std::fmt;

use salsa::Durability;
use triomphe::Arc;
use vfs::FileId;

use crate::{SourceDatabase, SourceRoot, SourceRootId};

/// Encapsulate a bunch of raw `.set` calls on the database.
#[derive(Default)]
pub struct FileChange {
    pub roots: Option<Vec<SourceRoot>>,
    pub files_changed: Vec<(FileId, Option<Vec<u8>>)>,
}

impl fmt::Debug for FileChange {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = fmt.debug_struct("Change");
        if let Some(roots) = &self.roots {
            d.field("roots", roots);
        }
        if !self.files_changed.is_empty() {
            d.field("files_changed", &self.files_changed.len());
        }
        d.finish()
    }
}

impl FileChange {
    pub fn set_roots(&mut self, roots: Vec<SourceRoot>) {
        self.roots = Some(roots);
    }

    pub fn change_file(&mut self, file_id: FileId, new_text: Option<Vec<u8>>) {
        self.files_changed.push((file_id, new_text))
    }

    pub fn apply(self, db: &mut dyn SourceDatabase) {
        let _p = tracing::info_span!("FileChange::apply").entered();
        if let Some(roots) = self.roots {
            for (idx, root) in roots.into_iter().enumerate() {
                let root_id = SourceRootId(idx as u32);
                let durability = Durability::LOW;
                for file_id in root.iter() {
                    db.set_file_source_root_with_durability(file_id, root_id, durability);
                }

                db.set_source_root_with_durability(root_id, Arc::new(root), durability);
            }
        }

        for (file_id, text) in self.files_changed {
            let durability = Durability::LOW;
            // XXX: can't actually remove the file, just reset the text
            let text = text.unwrap_or_default();
            db.set_file_text_with_durability(file_id, &text, durability)
        }
    }
}
