//! `hir_expand` deals with macro expansion.
//!
//! Specifically, it implements a concept of `MacroFile` -- a file whose syntax
//! tree originates not from the text of some `FileId`, but from some macro
//! expansion.

pub mod files;

pub use crate::files::{AstId, InRealFile};

#[macro_export]
macro_rules! impl_intern_lookup {
    ($db:ident, $id:ident, $loc:ident, $intern:ident, $lookup:ident) => {
        impl $crate::Intern for $loc {
            type Database = dyn $db;
            type ID = $id;
            fn intern(self, db: &Self::Database) -> Self::ID {
                db.$intern(self)
            }
        }

        impl $crate::Lookup for $id {
            type Database = dyn $db;
            type Data = $loc;
            fn lookup(&self, db: &Self::Database) -> Self::Data {
                db.$lookup(*self)
            }
        }
    };
}

// ideally these would be defined in base-db, but the orphan rule doesn't let us
pub trait Intern {
    type Database: ?Sized;
    type ID;
    fn intern(self, db: &Self::Database) -> Self::ID;
}

pub trait Lookup {
    type Database: ?Sized;
    type Data;
    fn lookup(&self, db: &Self::Database) -> Self::Data;
}
