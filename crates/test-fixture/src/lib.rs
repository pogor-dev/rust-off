use std::{mem, str::FromStr};

use base_db::{FileChange, FileSet, SalsaFileId, SourceDatabase};
use span::{Edition, FileId};
use test_utils::{CURSOR_MARKER, ESCAPED_CURSOR_MARKER, Fixture, FixtureWithProjectMeta, RangeOrOffset, extract_range_or_offset};

pub trait WithFixture: Default + SourceDatabase + 'static {
    #[track_caller]
    fn with_single_file(#[rust_analyzer::rust_fixture] ra_fixture: &str) -> (Self, SalsaFileId) {
        let fixture = ChangeFixture::parse(ra_fixture);
        let mut db = Self::default();
        fixture.change.apply(&mut db);
        assert_eq!(fixture.files.len(), 1, "Multiple file found in the fixture");
        (db, fixture.files[0])
    }
}

// impl<DB: SourceDatabase + Default + 'static> WithFixture for DB {}

pub struct ChangeFixture {
    pub file_position: Option<(FileId, RangeOrOffset)>,
    pub files: Vec<FileId>,
    pub change: FileChange,
}

const SOURCE_ROOT_PREFIX: &str = "/";

impl ChangeFixture {
    pub fn parse(#[rust_analyzer::rust_fixture] ra_fixture: &str) -> ChangeFixture {
        let FixtureWithProjectMeta { fixture } = FixtureWithProjectMeta::parse(ra_fixture);
        let mut change = FileChange::new();
        let files = Vec::new();

        let mut file_set = FileSet::default();
        let file_id = FileId::from_raw(0);
        let mut roots = Vec::new();

        let mut file_position = None;

        for entry in fixture {
            let mut range_or_offset = None;
            let _text = if entry.text.contains(CURSOR_MARKER) {
                if entry.text.contains(ESCAPED_CURSOR_MARKER) {
                    entry.text.replace(ESCAPED_CURSOR_MARKER, CURSOR_MARKER)
                } else {
                    let (roo, text) = extract_range_or_offset(&entry.text);
                    assert!(file_position.is_none());
                    range_or_offset = Some(roo);
                    text
                }
            } else {
                entry.text.as_str().into()
            };

            let meta = FileMeta::from_fixture(entry);
            if let Some(range_or_offset) = range_or_offset {
                file_position = Some((file_id, range_or_offset));
            }

            assert!(meta.path.starts_with(SOURCE_ROOT_PREFIX));
        }

        let root = SourceRoot::new_local(mem::take(&mut file_set));
        roots.push(root);

        change.set_roots(roots);
        ChangeFixture { file_position, files, change }
    }
}

#[derive(Debug)]
struct FileMeta {
    path: String,
    edition: Edition,
}

impl FileMeta {
    fn from_fixture(f: Fixture) -> Self {
        Self {
            path: f.path,
            edition: f.edition.map_or(Edition::CURRENT, |v| Edition::from_str(&v).unwrap()),
        }
    }
}
