pub use crate::cursor::Cursor;
use crate::token::Token;
use crate::token::TokenKind;

mod cursor;
mod token;

impl Cursor<'_> {
    /// Parses a token from the input string.
    pub fn advance_token(&mut self) -> Token {
        let first_byte = match self.next() {
            Some(b) => b,
            None => return Token::new(TokenKind::Eof, 0),
        };
        
        // let token_kind = match first_byte {
        //     b'o' | b'e' | b's' | b't' | b'r' | b'm' => TokenKind::Keyword,
        //     _ => TokenKind::Unknown,
        // };
        Token::new(TokenKind::Eof, 0)
    }
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &[u8]) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != TokenKind::Eof {
            Some(token)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eof() {
        let tokens: Vec<Token> = tokenize(b"").collect();

        let expected_tokens = vec![];

        assert_eq!(tokens, expected_tokens);
    }
}
