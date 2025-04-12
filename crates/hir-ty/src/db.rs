//! The home of `HirDatabase`, which is the Salsa database containing all the
//! type inference-related queries.

use hir_def::db::DefDatabase;

pub trait HirDatabase: DefDatabase + std::fmt::Debug {}
