use syntax::{Parse, SyntaxError, ast};

use super::{file_db::FileId, source_db::SourceDatabase};

/// Database which stores all significant input facts: source code and project
/// model. Everything else in pdf-analyzer is derived from these queries.
#[salsa_macros::db]
pub(super) trait RootQueryDb: SourceDatabase + salsa::Database {
    /// Parses the file into the syntax tree.
    fn parse(&self, file_id: FileId) -> Parse<ast::PdfDocument>;

    /// Returns the set of errors obtained from parsing the file including validation errors.
    fn parse_errors(&self, file_id: FileId) -> Option<&[SyntaxError]>;
}

#[salsa_macros::db]
impl<DB> RootQueryDb for DB
where
    DB: SourceDatabase + salsa::Database,
{
    /// Parses the file into the syntax tree.
    fn parse(&self, file_id: FileId) -> Parse<ast::PdfDocument> {
        unimplemented!()
    }

    /// Returns the set of errors obtained from parsing the file including validation errors.
    fn parse_errors(&self, file_id: FileId) -> Option<&[SyntaxError]> {
        unimplemented!()
    }
}
