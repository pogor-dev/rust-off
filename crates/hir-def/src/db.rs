//! Defines database & queries for name resolution.

use base_db::{RootQueryDb, SourceDatabase};

use crate::{BlockId, BlockLoc};

#[salsa::db]
pub trait InternDatabase: RootQueryDb + salsa::Database {
    fn intern_block(&self, loc: BlockLoc) -> BlockId;
}

impl<Db> InternDatabase for Db
where
    Db: RootQueryDb + salsa::Database,
{
    fn intern_block(&self, loc: BlockLoc) -> BlockId {
        let id = BlockId { loc };
        self.intern_block(id)
    }
}

#[salsa::db]
pub trait DefDatabase: InternDatabase + SourceDatabase {}
