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
            // End of line marker. Sometimes it is required by the PDF spec (e.g. in PDF object keywords).
            b if is_eol(b) => {
                self.eat_eol(b);
                TokenKind::Eol
            }

            // Whitespace sequence.
            b if is_whitespace(b) => {
                self.eat_while(|b| !is_eol(b) && is_whitespace(b));
                TokenKind::Whitespace
            }

            // Numeric literal.
            // See ISO `32000-1:2008`, Section 7.3.3 Numeric Objects.
            _b @ b'0'..=b'9' => {
                let literal_kind = self.number();
                TokenKind::Literal { kind: literal_kind }
            }

            // Real number starting with a dot.
            // See ISO `32000-1:2008`, Section 7.3.3 Numeric Objects.
            _b @ b'.' if matches!(self.peek_first(), b'0'..=b'9') => {
                self.number();
                TokenKind::Literal { kind: LiteralKind::Real }
            }

            // PDF Name.
            // See ISO `32000-1:2008`, Section 7.3.5 Name Objects.
            b'/' => {
                let next_byte = self.peek_first();
                if is_whitespace(next_byte) || is_delimiter(next_byte) {
                    // PDF name that starts with a delimiter or whitespace is invalid.
                    self.eat_while(|b| is_whitespace(b) || is_delimiter(b));
                    TokenKind::Unknown
                } else {
                    // Consume the name until a delimiter or whitespace is encountered.
                    self.eat_while(|b| !is_whitespace(b) && !is_delimiter(b));
                    TokenKind::Literal { kind: LiteralKind::Name }
                }
            }

            // One-symbol tokens.
            b'-' => TokenKind::Minus, // TODO: check if it is a part of a number
            b'+' => TokenKind::Plus,  // TODO: check if it is a part of a number

            _ => TokenKind::Unknown,
        };

        let res = Token::new(token_kind, self.pos_within_token());
        self.reset_pos_within_token();
        res
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

    fn eat_eol(&mut self, first_byte: u8) {
        if first_byte == b'\r' && self.peek_first() == b'\n' {
            self.next();
        }
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

fn is_eol(b: u8) -> bool {
    matches!(
        b,
        b'\n' // line feed
         | b'\r' // carriage return
    )
}

/// See ISO `32000-1:2008`, Section 7.2.3 Character Set, Table 1 Whitespace characters.
fn is_whitespace(b: u8) -> bool {
    matches!(
        b,
        b'\0' // null byte
        | b'\t' // horizontal tab
        | b'\n' // line feed
        | b'\x0C' // form feed
        | b'\r' // carriage return
        | b' ' // space
    )
}

/// See ISO `32000-1:2008`, Section 7.2.3 Character Set, Table 2 Delimiter characters.
fn is_delimiter(b: u8) -> bool {
    matches!(
        b,
        b'(' // open parenthesis
        | b')' // close parenthesis
        | b'<' // less-than sign
        | b'>' // greater-than sign
        | b'[' // open square bracket
        | b']' // close square bracket
        | b'{' // open curly bracket
        | b'}' // close curly bracket
        | b'/' // solidus
        | b'%' // percent sign
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
    fn test_eol() {
        let tokens: Vec<Token> = tokenize(b"\n\r\n\r").collect();
        let expected_tokens = vec![
            Token::new(TokenKind::Eol, 1), // \n
            Token::new(TokenKind::Eol, 2), // \r\n
            Token::new(TokenKind::Eol, 1), // \r
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_whitespace() {
        let tokens: Vec<Token> = tokenize(b" \t\x0C\0\r\n").collect();
        let expected_tokens = vec![
            Token::new(TokenKind::Whitespace, 4), // \t\x0C\0\r
            Token::new(TokenKind::Eol, 2),        // \r\n
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_integer_numbers() {
        let tokens: Vec<Token> = tokenize(b"123 43445 +17 -98 0").filter(|t| t.kind != TokenKind::Whitespace).collect();

        let expected_tokens = vec![
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 3), // 123
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 5), // 43445
            Token::new(TokenKind::Plus, 1),                               // +
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 2), // 17
            Token::new(TokenKind::Minus, 1),                              // -
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 2), // 98
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1), // 0
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_real_numbers() {
        let tokens: Vec<Token> = tokenize(b"34.5 -3.62 +123.6 4. -.002 0").filter(|t| t.kind != TokenKind::Whitespace).collect();
        let expected_tokens = vec![
            Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 4), // 34.5
            Token::new(TokenKind::Minus, 1),                               // -
            Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 4), // 3.62
            Token::new(TokenKind::Plus, 1),                                // +
            Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 5), // 123.6
            Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 2), // 4.
            Token::new(TokenKind::Minus, 1),                               // -
            Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 4), // .002
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1),  // 0
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_names() {
        let tokens: Vec<Token> = tokenize(
            b"/Name1 /ASomewhatLongerName /A;Name_With-Various***Characters? /1.2 /$$ /@pattern /.notdef /Lime#20Green /paired#28#29parentheses /The_Key_of_F#23_Minor /A#42 /Name1/Name2",
        )
        .filter(|t| t.kind != TokenKind::Whitespace)
        .collect();

        let expected_tokens = vec![
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 6),  // /Name1
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 20), // /ASomewhatLongerName
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 34), // /A;Name_With-Various***Characters?
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 4),  // /1.2
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 3),  // /$$
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 9),  // /@pattern
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 8),  // /.notdef
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 13), // /Lime#20Green
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 24), // /paired#28#29parentheses
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 22), // /The_Key_of_F#23_Minor
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 5),  // /A#42
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 6),  // /Name1
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 6),  // /Name2
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_invalid_names() {
        assert_eq!(tokenize(b"////").collect::<Vec<Token>>(), vec![Token::new(TokenKind::Unknown, 4)]);
        assert_eq!(tokenize(b"/(").collect::<Vec<Token>>(), vec![Token::new(TokenKind::Unknown, 2)]); // TODO: should be unknown + literal?
        assert_eq!(tokenize(b"/)").collect::<Vec<Token>>(), vec![Token::new(TokenKind::Unknown, 2)]);
        assert_eq!(tokenize(b"/<").collect::<Vec<Token>>(), vec![Token::new(TokenKind::Unknown, 2)]); // TODO: should be unknown + hex string or dict?
        assert_eq!(tokenize(b"/>").collect::<Vec<Token>>(), vec![Token::new(TokenKind::Unknown, 2)]);
        assert_eq!(tokenize(b"/[").collect::<Vec<Token>>(), vec![Token::new(TokenKind::Unknown, 2)]); // TODO: should be unknown + array?
        assert_eq!(tokenize(b"/]").collect::<Vec<Token>>(), vec![Token::new(TokenKind::Unknown, 2)]);
        assert_eq!(tokenize(b"/{").collect::<Vec<Token>>(), vec![Token::new(TokenKind::Unknown, 2)]); // TODO: should be unknown + PostScript?
        assert_eq!(tokenize(b"/}").collect::<Vec<Token>>(), vec![Token::new(TokenKind::Unknown, 2)]);
        assert_eq!(tokenize(b"/%").collect::<Vec<Token>>(), vec![Token::new(TokenKind::Unknown, 2)]); // TODO: should be unknown + comment?
        assert_eq!(
            tokenize(b"/Name1(").collect::<Vec<Token>>(),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 6), Token::new(TokenKind::Unknown, 1)]
        );
    }
}
