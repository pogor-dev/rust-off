//! Defines `ExpressionStore`: a lowered representation of functions, statics and
//! consts.

mod body;

pub use self::body::{Body, BodySourceMap};

#[derive(Debug, Eq, PartialEq)]
pub struct ExpressionStore {}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct ExpressionStoreSourceMap {}
