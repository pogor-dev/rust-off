//! Assorted testing utilities.
//!
//! Most notable things are:
//!
//! * Rich text comparison, which outputs a diff.
//! * Extracting markup (mainly, `$0` markers) out of fixture strings.
//! * marks (see the eponymous module).

mod fixture;
use text_size::{TextRange, TextSize};

pub use crate::fixture::{Fixture, FixtureWithProjectMeta};

#[derive(Clone, Copy, Debug)]
pub enum RangeOrOffset {
    Range(TextRange),
    Offset(TextSize),
}

impl RangeOrOffset {
    pub fn expect_offset(self) -> TextSize {
        match self {
            RangeOrOffset::Offset(it) => it,
            RangeOrOffset::Range(_) => panic!("expected an offset but got a range instead"),
        }
    }
    pub fn expect_range(self) -> TextRange {
        match self {
            RangeOrOffset::Range(it) => it,
            RangeOrOffset::Offset(_) => panic!("expected a range but got an offset"),
        }
    }
    pub fn range_or_empty(self) -> TextRange {
        match self {
            RangeOrOffset::Range(range) => range,
            RangeOrOffset::Offset(offset) => TextRange::empty(offset),
        }
    }
}

impl From<RangeOrOffset> for TextRange {
    fn from(selection: RangeOrOffset) -> Self {
        match selection {
            RangeOrOffset::Range(it) => it,
            RangeOrOffset::Offset(it) => TextRange::empty(it),
        }
    }
}
