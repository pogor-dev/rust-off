use base_db::{FileChange, SourceDatabase, Version};
use span::EditionedFileId;
use test_utils::{FixtureWithProjectMeta, RangeOrOffset};

pub trait WithFixture: Default + SourceDatabase + 'static {
    #[track_caller]
    fn with_single_file(#[rust_analyzer::rust_fixture] ra_fixture: &str) -> (Self, EditionedFileId) {
        let fixture = ChangeFixture::parse(ra_fixture);
        let mut db = Self::default();
        fixture.change.apply(&mut db);
        assert_eq!(fixture.files.len(), 1, "Multiple file found in the fixture");
        (db, fixture.files[0])
    }
}

impl<DB: SourceDatabase + Default + 'static> WithFixture for DB {}

pub struct ChangeFixture {
    pub file_position: Option<(EditionedFileId, RangeOrOffset)>,
    pub files: Vec<EditionedFileId>,
    pub change: FileChange,
}

impl ChangeFixture {
    pub fn parse(#[rust_analyzer::rust_fixture] ra_fixture: &str) -> ChangeFixture {
        let FixtureWithProjectMeta { fixture, version } = FixtureWithProjectMeta::parse(ra_fixture);
        let version = Some({ Version::parse(&format!(version.as_deref().unwrap_or("2.0"))).unwrap() });
    }
}
