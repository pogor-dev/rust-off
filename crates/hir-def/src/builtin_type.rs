//! This module defines built-in types.
//!
//! A peculiarity of built-in types is that they are always available and are
//! not associated with any particular crate.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinType {
    Bool,
    Name,
    LiteralString,
    HexString,
    Int,
    Real,
}
