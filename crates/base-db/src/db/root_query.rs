use syntax::{Parse, SyntaxError, ast};

use super::{file_db::FileId, source_db::SourceDatabase};

/// Database which stores all significant input facts: source code and project
/// model. Everything else in pdf-analyzer is derived from these queries.
#[salsa_macros::db]
pub(super) trait RootQueryDb: SourceDatabase + salsa::Database {
    /// Parses the file into the syntax tree.
    fn parse(&self, file_id: FileId) -> Parse<ast::SourceFile>;

    /// Returns the set of errors obtained from parsing the file including validation errors.
    fn parse_errors(&self, file_id: FileId) -> Option<&[SyntaxError]>;
}

#[salsa_macros::db]
impl<DB> RootQueryDb for DB
where
    DB: SourceDatabase + salsa::Database,
{
    /// Parses the file into the syntax tree.
    fn parse(&self, file_id: FileId) -> Parse<ast::SourceFile> {
        let _p = tracing::info_span!("parse", ?file_id).entered();
        let text = self.file_text(file_id.file_id(self)).text(self);
        ast::SourceFile::parse(&text)
    }

    /// Returns the set of errors obtained from parsing the file including validation errors.
    fn parse_errors(&self, file_id: FileId) -> Option<&[SyntaxError]> {
        parse_errors(self, file_id).as_ref().map(|it| &**it)
    }
}

#[salsa_macros::tracked(return_ref)]
fn parse_errors(db: &dyn RootQueryDb, file_id: FileId) -> Option<Box<[SyntaxError]>> {
    let errors = db.parse(file_id).errors();
    match &*errors {
        [] => None,
        [..] => Some(errors.into()),
    }
}
