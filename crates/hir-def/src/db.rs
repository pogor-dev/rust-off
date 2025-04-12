//! Defines database & queries for name resolution.

use base_db::{RootQueryDb, SourceDatabase};

pub trait InternDatabase: RootQueryDb {}
pub trait DefDatabase: InternDatabase + SourceDatabase {}
