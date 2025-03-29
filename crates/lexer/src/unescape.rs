//! Utilities for validating string and char literals and turning them into
//! values they represent.

use std::ops::Range;
use Mode::*;

/// What kind of literal do we parse.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    LiteralString,
    HexString,
    Name,
}

/// Errors and warnings that can occur during string unescaping. They mostly
/// relate to malformed escape sequences, but there are a few that are about
/// other problems.
#[derive(Debug, PartialEq, Eq)]
pub enum EscapeError {
    /// Invalid escape character (e.g. '\z').
    InvalidEscape,
    /// Non-ascii character in string literal, hex literal, or name.
    NonAsciiChar,
}

/// Takes the contents of a unicode-only (non-mixed-utf8) literal (without
/// quotes) and produces a sequence of escaped characters or errors.
///
/// Values are returned by invoking `callback`. For `Char` and `Byte` modes,
/// the callback will be called exactly once.
pub fn unescape_str<F>(src: &str, mode: Mode, callback: &mut F)
where
    F: FnMut(Range<usize>, Result<char, EscapeError>),
{
    match mode {
        LiteralString => unescape_non_raw_common(src, mode, callback),
        _ => unreachable!(),
    }
}

/// Takes a contents of a string literal (without quotes) and produces a
/// sequence of escaped characters or errors.
fn unescape_non_raw_common<F, T: From<char> + From<u8>>(src: &str, _mode: Mode, callback: &mut F)
where
    F: FnMut(Range<usize>, Result<T, EscapeError>),
{
    let mut chars = src.chars();

    while let Some(c) = chars.next() {
        let start = src.len() - chars.as_str().len() - c.len_utf8();
        let res = match c {
            _ => ascii_check(c).map(T::from),
        };
        let end = src.len() - chars.as_str().len();
        callback(start..end, res);
    }
}

#[inline]
fn ascii_check(c: char) -> Result<char, EscapeError> {
    if c.is_ascii() {
        Ok(c)
    } else {
        Err(EscapeError::NonAsciiChar)
    }
}
