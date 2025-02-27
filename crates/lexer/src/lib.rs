pub use crate::cursor::Cursor;
pub use crate::token::{LiteralKind, Token, TokenKind};

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

            // Numeric literal that starts with a digit and/or plus/minus sign.
            // See ISO `32000-1:2008`, Section 7.3.3 Numeric Objects.
            b if matches!(b, b'+' | b'-' | b'.') || b.is_ascii_digit() => {
                let literal_kind = self.number(b);
                TokenKind::Literal { kind: literal_kind }
            }

            // PDF keyword.
            b if b.is_ascii_alphabetic() => {
                self.eat_while(|b| b.is_ascii_alphanumeric());
                TokenKind::Keyword
            }

            // PDF Name.
            // See ISO `32000-1:2008`, Section 7.3.5 Name Objects.
            b'/' => {
                // Consume the name until a delimiter or whitespace is encountered.
                self.eat_while(|b| !is_whitespace(b) && !is_delimiter(b));
                TokenKind::Literal { kind: LiteralKind::Name }
            }

            // PDF literal string.
            // See ISO `32000-1:2008`, Section 7.3.4.2 Literal Strings.
            b'(' => {
                if self.eat_literal_string() && self.pos_within_token() > 1 {
                    TokenKind::Literal {
                        kind: LiteralKind::LiteralString,
                    }
                } else {
                    TokenKind::Unknown // If there is no byte following the opening parenthesis, it is an invalid token.
                }
            }

            // PDF hexademical string.
            // See ISO `32000-1:2008`, Section 7.3.4.3 Hexadecimal Strings.
            b'<' if self.peek_first() != b'<' => {
                self.eat_while(|b| b.is_ascii_hexdigit() || is_whitespace(b));

                if self.peek_first() == b'>' {
                    self.next();
                    TokenKind::Literal { kind: LiteralKind::HexString }
                } else {
                    TokenKind::Unknown // If the hex string is not closed, it is an invalid token.
                }
            }

            // PDF Comments
            // See ISO `32000-1:2008`, Section 7.2.4 Comments.
            b'%' => {
                self.eat_while(|b| !is_eol(b));
                TokenKind::Comment
            }

            // One-symbol tokens.
            b'[' => TokenKind::OpenBracket,  // See ISO `32000-1:2008`, Section 7.3.6 Array Objects.
            b']' => TokenKind::CloseBracket, // See ISO `32000-1:2008`, Section 7.3.6 Array Objects.
            b'<' => {
                // We ensured before that this is not a hex string.
                // See ISO `32000-1:2008`, Section 7.3.7 Dictionary Objects.
                self.next();
                TokenKind::OpenDict
            }
            b'>' if self.peek_first() == b'>' => {
                self.next();
                TokenKind::CloseDict // See ISO `32000-1:2008`, Section 7.3.7 Dictionary Objects.
            }

            _ => TokenKind::Unknown,
        };

        let res = Token::new(token_kind, self.pos_within_token());
        self.reset_pos_within_token();
        res
    }

    fn number(&mut self, first_byte: u8) -> LiteralKind {
        let next_byte = self.peek_first();

        // If a plus or minus sign is not followed by a digit, it is not a valid number.
        if matches!(first_byte, b'+' | b'-') && !(next_byte.is_ascii_digit() || next_byte == b'.') {
            return LiteralKind::Int;
        }

        self.eat_decimal_digits(); // integer part with optional sign

        if first_byte == b'.' {
            return LiteralKind::Real; // If the first byte is a dot (.345), it is a real number.
        }

        match self.peek_first() {
            b'.' => {
                self.next();
                self.eat_decimal_digits(); // Reading the fractional part of the real number.
                LiteralKind::Real
            }
            _ => LiteralKind::Int,
        }
    }

    fn eat_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.peek_first() {
                b if b.is_ascii_digit() => {
                    has_digits = true;
                    self.next();
                }
                b'+' | b'-' => {
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

    fn eat_literal_string(&mut self) -> bool {
        let mut depth = 1;

        loop {
            match self.next() {
                Some(b'(') => depth += 1,
                Some(b')') => {
                    depth -= 1;
                    if depth == 0 {
                        return true; // the parentheses are balanced, we have a valid literal string
                    }
                }
                None => return false, // the string is not closed
                _ => {}
            }
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

    /// ISO `32000-1:2008`, Section 7.2.3 Character Set.
    #[test]
    fn test_character_set() {
        assert_eq!(test_input(b""), vec![]);

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
    }

    /// ISO `32000-1:2008`, Section 7.3.3 Numeric Objects.
    #[test]
    fn test_numbers() {
        assert_eq!(test_input(b"123"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 3)]);
        assert_eq!(test_input(b"43445"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 5)]);
        assert_eq!(test_input(b"+17"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 3)]);
        assert_eq!(test_input(b"-98"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 3)]);
        assert_eq!(test_input(b"0"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1)]);
        assert_eq!(test_input(b"00987"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 5)]);
        assert_eq!(test_input(b"34.5"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 4)]);
        assert_eq!(
            test_input(b"-3.62"),
            [Token {
                kind: TokenKind::Literal { kind: LiteralKind::Real },
                len: 5
            }]
        );
        assert_eq!(test_input(b"+123.6"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 6)]);
        assert_eq!(test_input(b"4."), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 2)]);
        assert_eq!(test_input(b"-.002"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 5)]);
        assert_eq!(test_input(b"009.87"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 6)]);
        assert_eq!(test_input(b".0"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 2)]);

        // invalid numbers
        assert_eq!(test_input(b"."), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 1)]);
        assert_eq!(test_input(b"+"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1)]);
        assert_eq!(test_input(b"-"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1)]);
        assert_eq!(
            test_input(b"+-"),
            vec![
                Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1),
                Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1)
            ]
        );
        assert_eq!(
            test_input(b"-+"),
            vec![
                Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1),
                Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1)
            ]
        );
        assert_eq!(test_input(b"-."), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 2)]);
        assert_eq!(
            test_input(b".."),
            vec![
                Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 1),
                Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 1)
            ]
        );
    }

    /// ISO `32000-1:2008`, Section 7.3.5 Name Objects.
    #[test]
    fn test_names() {
        assert_eq!(test_input(b"/Name1"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 6)]);
        assert_eq!(
            test_input(b"/ASomewhatLongerName"),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 20)]
        );
        assert_eq!(
            test_input(b"/A;Name_With-Various***Characters?"),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 34)]
        );
        assert_eq!(test_input(b"/1.2"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 4)]);
        assert_eq!(test_input(b"/$$"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 3)]);
        assert_eq!(test_input(b"/@pattern"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 9)]);
        assert_eq!(test_input(b"/.notdef"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 8)]);
        assert_eq!(
            test_input(b"/Lime#20Green"),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 13)]
        );
        assert_eq!(
            test_input(b"/paired#28#29parentheses"),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 24)]
        );
        assert_eq!(
            test_input(b"/The_Key_of_F#23_Minor"),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 22)]
        );

        // invalid names
        assert_eq!(test_input(b"/"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1)]);
        assert_eq!(
            test_input(b"/("),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1), Token::new(TokenKind::Unknown, 1)]
        );
        assert_eq!(
            test_input(b"/)"),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1), Token::new(TokenKind::Unknown, 1)]
        );
        assert_eq!(
            test_input(b"/<"),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1), Token::new(TokenKind::Unknown, 1)]
        );
        assert_eq!(
            test_input(b"/>"),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1), Token::new(TokenKind::Unknown, 1)]
        );
        assert_eq!(
            test_input(b"/["),
            vec![
                Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1),
                Token::new(TokenKind::OpenBracket, 1)
            ]
        );
        assert_eq!(
            test_input(b"/]"),
            vec![
                Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1),
                Token::new(TokenKind::CloseBracket, 1)
            ]
        );
        assert_eq!(
            test_input(b"/{"),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1), Token::new(TokenKind::Unknown, 1)]
        );
        assert_eq!(
            test_input(b"/}"),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1), Token::new(TokenKind::Unknown, 1)]
        );
        assert_eq!(
            test_input(b"/%"),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1), Token::new(TokenKind::Comment, 1)]
        );
        assert_eq!(
            test_input(b"/Name1("),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 6), Token::new(TokenKind::Unknown, 1)]
        );
    }

    /// ISO `32000-1:2008`, Section 7.3.2 Boolean Objects.
    #[test]
    fn test_boolean() {
        assert_eq!(test_input(b"true"), vec![Token::new(TokenKind::Keyword, 4)]);
        assert_eq!(test_input(b"false"), vec![Token::new(TokenKind::Keyword, 5)]);
    }

    /// ISO `32000-1:2008`, Section 7.3.9 Null Object.
    #[test]
    fn test_null() {
        assert_eq!(test_input(b"null"), vec![Token::new(TokenKind::Keyword, 4)]);
    }

    /// ISO `32000-1:2008`, Section 7.3.4.2 Literal Strings.
    #[test]
    fn test_literal_string() {
        assert_eq!(
            test_input(b"(This is a string)"),
            vec![Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::LiteralString
                },
                18
            )]
        );
        assert_eq!(
            test_input(b"(Strings can contain newlines \nand such.)"),
            vec![Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::LiteralString
                },
                41
            )]
        );

        assert_eq!(
            test_input(b"(Strings can contain balanced parentheses () \nand special characters ( * ! & } ^ %and so on) .)"),
            vec![Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::LiteralString
                },
                95
            )]
        );

        assert_eq!(
            test_input(b"(The following is an empty string .)"),
            vec![Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::LiteralString
                },
                36
            )]
        );
        assert_eq!(
            test_input(b"()"),
            vec![Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::LiteralString
                },
                2
            )]
        );
        assert_eq!(
            test_input(b"(It has zero (0) length.)"),
            vec![Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::LiteralString
                },
                25
            )]
        );

        // invalid literal strings
        assert_eq!(test_input(b"("), vec![Token::new(TokenKind::Unknown, 1)]);
        assert_eq!(test_input(b")"), vec![Token::new(TokenKind::Unknown, 1)]);
        assert_eq!(
            test_input(b"(This string has ( unbalanced parentheses.)"),
            vec![Token::new(TokenKind::Unknown, 43)]
        );
    }

    // ISO `32000-1:2008`, Section 7.3.4.3 Hexadecimal Strings.
    #[test]
    fn test_hex_string() {
        assert_eq!(
            test_input(b"<4E6F762073686D6F7A206B6120706F702E>"),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::HexString }, 36)]
        );

        assert_eq!(
            test_input(b"<901FA3>"),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::HexString }, 8)]
        );
        assert_eq!(test_input(b"<901FA>"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::HexString }, 7)]);

        assert_eq!(
            test_input(b"<90\0 1F\r\n A3\t\x0C>"),
            vec![Token::new(TokenKind::Literal { kind: LiteralKind::HexString }, 15)]
        );

        // invalid hex strings
        assert_eq!(test_input(b"<"), vec![Token::new(TokenKind::Unknown, 1)]);
        assert_eq!(test_input(b">"), vec![Token::new(TokenKind::Unknown, 1)]);
        assert_eq!(test_input(b"<0"), vec![Token::new(TokenKind::Unknown, 2)]);
        assert_eq!(test_input(b"<0<"), vec![Token::new(TokenKind::Unknown, 2), Token::new(TokenKind::Unknown, 1)]);
    }

    // ISO `32000-1:2008`, Section 7.3.6 Array Objects.
    #[test]
    fn test_array() {
        assert_eq!(
            test_input(b"[ 1 (2) 3 ]"),
            vec![
                Token::new(TokenKind::OpenBracket, 1),
                Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1),
                Token::new(
                    TokenKind::Literal {
                        kind: LiteralKind::LiteralString
                    },
                    3
                ),
                Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1),
                Token::new(TokenKind::CloseBracket, 1),
            ]
        );
    }

    // ISO `32000-1:2008`, Section 7.3.7 Dictionary Objects.
    #[test]
    fn test_dictionary() {
        assert_eq!(
            test_input(b"<< /Type /Catalog /Pages 3 0 R >>"),
            vec![
                Token::new(TokenKind::OpenDict, 2),
                Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 5),
                Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 8),
                Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 6),
                Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1),
                Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1),
                Token::new(TokenKind::Keyword, 1),
                Token::new(TokenKind::CloseDict, 2),
            ]
        );
    }

    // ISO `32000-1:2008`, Section 7.2.4 Comments.
    #[test]
    fn test_comments() {
        assert_eq!(test_input(b"% This is a comment"), vec![Token::new(TokenKind::Comment, 19)]);
        assert_eq!(
            test_input(b"% This is a comment\n"),
            vec![Token::new(TokenKind::Comment, 19), Token::new(TokenKind::Eol, 1)]
        );
        assert_eq!(
            test_input(b"% This is a comment\r\n"),
            vec![Token::new(TokenKind::Comment, 19), Token::new(TokenKind::Eol, 2)]
        );
        assert_eq!(
            test_input(b"abc%comment (/%) blah blah blah \n123"),
            vec![
                Token::new(TokenKind::Keyword, 3),
                Token::new(TokenKind::Comment, 29),
                Token::new(TokenKind::Eol, 1),
                Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 3)
            ]
        );
    }

    fn test_input(input: &[u8]) -> Vec<Token> {
        tokenize(input).filter(|t| t.kind != TokenKind::Whitespace).collect()
    }
}
