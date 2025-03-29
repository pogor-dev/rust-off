//! Defines `ExpressionStore`: a lowered representation of functions, statics and
//! consts.

mod body;

pub use self::body::Body;

#[derive(Debug, Eq, PartialEq)]
pub struct ExpressionStore {}
