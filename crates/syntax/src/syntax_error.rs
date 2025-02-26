//! See docs for `SyntaxError`.

use std::fmt;

use crate::{TextRange, TextSize};

/// Represents the result of unsuccessful tokenization, parsing
/// or tree validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxError(String, TextRange);

impl SyntaxError {
    pub fn new(message: impl Into<String>, range: TextRange) -> Self {
        Self(message.into(), range)
    }
    pub fn new_at_offset(message: impl Into<String>, offset: TextSize) -> Self {
        Self(message.into(), TextRange::empty(offset))
    }

    pub fn range(&self) -> TextRange {
        self.1
    }

    pub fn with_range(mut self, range: TextRange) -> Self {
        self.1 = range;
        self
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TextRange, TextSize};

    #[test]
    fn test_syntax_error_new() {
        let range = TextRange::new(TextSize::from(0), TextSize::from(5));
        let error = SyntaxError::new("error message", range);
        assert_eq!(error.0, "error message");
        assert_eq!(error.1, range);
    }

    #[test]
    fn test_syntax_error_new_at_offset() {
        let offset = TextSize::from(10);
        let error = SyntaxError::new_at_offset("error message", offset);
        assert_eq!(error.0, "error message");
        assert_eq!(error.1, TextRange::empty(offset));
    }

    #[test]
    fn test_syntax_error_range() {
        let range = TextRange::new(TextSize::from(0), TextSize::from(5));
        let error = SyntaxError::new("error message", range);
        assert_eq!(error.range(), range);
    }

    #[test]
    fn test_syntax_error_with_range() {
        let range1 = TextRange::new(TextSize::from(0), TextSize::from(5));
        let range2 = TextRange::new(TextSize::from(5), TextSize::from(10));
        let error = SyntaxError::new("error message", range1).with_range(range2);
        assert_eq!(error.range(), range2);
    }

    #[test]
    fn test_syntax_error_display() {
        let range = TextRange::new(TextSize::from(0), TextSize::from(5));
        let error = SyntaxError::new("error message", range);
        assert_eq!(format!("{}", error), "error message");
    }
}
