use crate::token::Token;
use crate::token::TokenKind::*;
pub use crate::cursor::Cursor;

mod cursor;
mod token;

impl Cursor<'_> {
    /// Parses a token from the input string.
    pub fn advance_token(&mut self) -> Token {
        Token::new(Eof, 0)
    }
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &[u8]) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != Eof { Some(token) } else { None }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_returns_first_byte() {
        let mut cursor = Cursor::new(b"%PDF-1.7");
        assert_eq!(cursor.next(), b'%');
    }
}