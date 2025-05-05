use super::file_db::FileText;

#[salsa_macros::db]
pub(super) trait SourceDatabase: salsa::Database {
    /// Text of the file.
    fn file_text(&self, file_id: vfs::FileId) -> FileText;

    fn set_file_text(&mut self, file_id: vfs::FileId, text: &[u8]);
}
