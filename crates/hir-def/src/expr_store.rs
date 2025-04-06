//! Defines `ExpressionStore`: a lowered representation of functions, statics and
//! consts.

mod body;
mod lower;

pub use self::body::{Body, BodySourceMap};

#[derive(Debug, Eq, PartialEq)]
pub struct ExpressionStore {}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct ExpressionStoreSourceMap {}
