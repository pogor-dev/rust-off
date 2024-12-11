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
            // See ISO `32000-1:2008`, Section 7.2.3 Character Set.
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
            b if b.is_ascii_digit() => {
                let literal_kind = self.number();
                TokenKind::Literal { kind: literal_kind }
            }

            // Real number starting with a dot.
            // See ISO `32000-1:2008`, Section 7.3.3 Numeric Objects.
            _b @ b'.' if matches!(self.peek_first(), b'0'..=b'9') => {
                self.number();
                TokenKind::Literal { kind: LiteralKind::Real }
            }

            // PDF keyword.
            b if b.is_ascii_alphabetic() => {
                self.eat_while(|b| b.is_ascii_alphanumeric());
                TokenKind::Keyword
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
    fn api_walkthrough() {
        assert_eq!(test_input(b""), vec![]);

        // __________________________________
        // ISO `32000-1:2008`, Section 7.2.3 Character Set.
        // __________________________________

        // end of line marker
        assert_eq!(
            tokenize(b"\n\r\n\r").collect::<Vec<Token>>(),
            vec![Token::new(TokenKind::Eol, 1), Token::new(TokenKind::Eol, 2), Token::new(TokenKind::Eol, 1)]
        );

        // whitespaces
        assert_eq!(
            tokenize(b" \t\x0C\0\r\n").collect::<Vec<Token>>(),
            vec![Token::new(TokenKind::Whitespace, 4), Token::new(TokenKind::Eol, 2)]
        );

        // __________________________________
        // ISO `32000-1:2008`, Section 7.3.3 Numeric Objects.
        // __________________________________

        assert_eq!(test_input(b"123"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 3)]);
        assert_eq!(test_input(b"43445"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 5)]);
        assert_eq!(
            test_input(b"+17"),
            vec![Token::new(TokenKind::Plus, 1), Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 2)]
        );
        assert_eq!(
            test_input(b"-98"),
            vec![Token::new(TokenKind::Minus, 1), Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 2)]
        );
        assert_eq!(test_input(b"0"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1)]);
        assert_eq!(test_input(b"00987"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 5)]);
        assert_eq!(test_input(b"34.5"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 4)]);
        assert_eq!(
            test_input(b"-3.62"),
            [
                Token { kind: TokenKind::Minus, len: 1 },
                Token {
                    kind: TokenKind::Literal { kind: LiteralKind::Real },
                    len: 4
                }
            ]
        );
        assert_eq!(
            test_input(b"+123.6"),
            vec![Token::new(TokenKind::Plus, 1), Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 5)]
        );
        assert_eq!(test_input(b"4."), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 2)]);
        assert_eq!(
            test_input(b"-.002"),
            vec![Token { kind: TokenKind::Minus, len: 1 }, Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 4)]
        );
        assert_eq!(test_input(b"009.87"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 6)]);

        // __________________________________
        // ISO `32000-1:2008`, Section 7.3.5 Name Objects.
        // __________________________________

        assert_eq!(test_input(b"/Name1"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 6)]);
        assert_eq!(test_input(b"/ASomewhatLongerName"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 20)]);
        assert_eq!(
            test_input(b"/A;Name_With-Various***Characters?"),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 34)]
        );
        assert_eq!(test_input(b"/1.2"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 4)]);
        assert_eq!(test_input(b"/$$"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 3)]);
        assert_eq!(test_input(b"/@pattern"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 9)]);
        assert_eq!(test_input(b"/.notdef"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 8)]);
        assert_eq!(test_input(b"/Lime#20Green"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 13)]);
        assert_eq!(test_input(b"/paired#28#29parentheses"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 24)]);
        assert_eq!(test_input(b"/The_Key_of_F#23_Minor"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 22)]);

        // invalid names
        assert_eq!(test_input(b"/"), vec![Token::new(TokenKind::Unknown, 1)]);
        assert_eq!(test_input(b"/("), vec![Token::new(TokenKind::Unknown, 2)]);
        assert_eq!(test_input(b"/)"), vec![Token::new(TokenKind::Unknown, 2)]);
        assert_eq!(test_input(b"/<"), vec![Token::new(TokenKind::Unknown, 2)]);
        assert_eq!(test_input(b"/>"), vec![Token::new(TokenKind::Unknown, 2)]);
        assert_eq!(test_input(b"/["), vec![Token::new(TokenKind::Unknown, 2)]);
        assert_eq!(test_input(b"/]"), vec![Token::new(TokenKind::Unknown, 2)]);
        assert_eq!(test_input(b"/{"), vec![Token::new(TokenKind::Unknown, 2)]);
        assert_eq!(test_input(b"/}"), vec![Token::new(TokenKind::Unknown, 2)]);
        assert_eq!(test_input(b"/%"), vec![Token::new(TokenKind::Unknown, 2)]);
        assert_eq!(
            test_input(b"/Name1("),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 6), Token::new(TokenKind::Unknown, 1)]
        );

        // __________________________________
        // ISO `32000-1:2008`, Section 7.3.2 Boolean Objects.
        // __________________________________
        assert_eq!(test_input(b"true"), vec![Token::new(TokenKind::Keyword, 4)]);
        assert_eq!(test_input(b"false"), vec![Token::new(TokenKind::Keyword, 5)]);

        // __________________________________
        // ISO `32000-1:2008`, Section 7.3.9 Null Object.
        // __________________________________
        assert_eq!(test_input(b"null"), vec![Token::new(TokenKind::Keyword, 4)]);
    }

    fn test_input(input: &[u8]) -> Vec<Token> {
        tokenize(input).filter(|t| t.kind != TokenKind::Whitespace).collect()
    }
}
