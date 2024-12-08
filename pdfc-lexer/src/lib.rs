pub use crate::cursor::Cursor;
use crate::token::LiteralKind;
use crate::token::Token;
use crate::token::TokenKind;

mod cursor;
mod token;

impl Cursor<'_> {
    /// Parses a token from the input.
    pub fn advance_token(&mut self) -> Token {
        let first_byte = match self.next() {
            Some(b) => b,
            None => return Token::new(TokenKind::Eof, 0),
        };

        let token_kind = match first_byte {
            // Whitespace sequence.
            b if is_whitespace(b) => self.whitespace(),

            // Numeric literal.
            _b @ b'0'..=b'9' => {
                let literal_kind = self.number();
                TokenKind::Literal { kind: literal_kind }
            }

            // Real number starting with a dot.
            _b @ b'.' if matches!(self.peek_first(), b'0'..=b'9') => {
                self.number();
                TokenKind::Literal { kind: LiteralKind::Real }
            }

            // One-symbol tokens.
            b'-' => TokenKind::Minus,
            b'+' => TokenKind::Plus,

            _ => TokenKind::Unknown,
        };

        let res = Token::new(token_kind, self.pos_within_token());
        self.reset_pos_within_token();
        res
    }

    fn whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);
        TokenKind::Whitespace
    }

    fn number(&mut self) -> LiteralKind {
        self.eat_decimal_digits(); // integer part

        match self.peek_first() {
            // Decimal part.
            b'.' => {
                self.next();
                self.eat_decimal_digits();
                LiteralKind::Real
            }
            _ => LiteralKind::Int,
        }
    }

    fn eat_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.peek_first() {
                b'0'..=b'9' => {
                    has_digits = true;
                    self.next();
                }
                _ => break,
            }
        }
        has_digits
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

pub(crate) fn is_whitespace(b: u8) -> bool {
    matches!(
        b,
        b'\0' // null byte
        | b' ' // space
        | b'\n' // newline
        | b'\r' // carriage return
        | b'\t' // horizontal tab
        | b'\x0C' // form feed
    )
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

    #[test]
    fn test_whitespace() {
        let tokens: Vec<Token> = tokenize(b" \t\x0C\0\r\n").collect();
        let expected_tokens = vec![Token::new(TokenKind::Whitespace, 6)];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_integer_numbers() {
        let tokens: Vec<Token> = tokenize(b"123 43445 +17 -98 0").collect();
        let expected_tokens = vec![
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 3), // 123
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 5), // 43445
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::Plus, 1),
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 2), // 17
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::Minus, 1),
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 2), // 98
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1), // 0
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_real_numbers() {
        let tokens: Vec<Token> = tokenize(b"34.5 -3.62 +123.6 4. -.002 0").collect();
        let expected_tokens = vec![
            Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 4), // 34.5
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::Minus, 1),
            Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 4), // 3.62
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::Plus, 1),
            Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 5), // 123.6
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 2), // 4.
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::Minus, 1),
            Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 4), // .002
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1), // 0
        ];
        assert_eq!(tokens, expected_tokens);
    }
}
