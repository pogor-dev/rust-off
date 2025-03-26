pub use crate::cursor::Cursor;
pub use crate::token::{LiteralKind, Token, TokenKind};

mod cursor;
mod token;

#[cfg(test)]
mod tests;

impl Cursor<'_> {
    /// Parses a token from the input.
    pub fn advance_token(&mut self) -> Token {
        let first_byte = match self.next() {
            Some(b) => b,
            None => return Token::new(TokenKind::Eof, 0),
        };

        let token_kind = match first_byte {
            _ if self.is_prev_token_stream() => {
                // PDF stream object.
                // The stream object is a special object that contains binary data and is marked by the `stream` and `endstream` keywords.
                // The stream content is skipped by the lexer and is decoded/parsed later by the parser.
                // See ISO `32000-2:2008`, Section 7.3.8 Stream Objects.
                self.set_prev_token_stream(false);
                self.eat_while_word(|word| word.starts_with(b"endstream"));
                TokenKind::RawStreamData
            }

            // End of line marker. Sometimes it is required by the PDF spec (e.g. in PDF object keywords).
            // See ISO `32000-2:2008`, Section 7.2.3 Character Set.
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
            // See ISO `32000-2:2008`, Section 7.3.3 Numeric Objects.
            b if matches!(b, b'+' | b'-' | b'.') || b.is_ascii_digit() => {
                let literal_kind = self.number(b);
                TokenKind::Literal { kind: literal_kind }
            }

            // PDF keyword.
            b if b.is_ascii_alphabetic() => {
                if b == b's' && self.try_eat_word(b"tream") {
                    self.set_prev_token_stream(true);
                    TokenKind::Ident
                } else {
                    // PDF keyword or identifier.
                    self.eat_while(|b| b.is_ascii_alphanumeric());
                    TokenKind::Ident
                }
            }

            // PDF Name.
            // See ISO `32000-2:2008`, Section 7.3.5 Name Objects.
            b'/' => {
                // Consume the name until a delimiter or whitespace is encountered.
                self.eat_while(|b| !is_whitespace(b) && !is_delimiter(b));
                TokenKind::Literal { kind: LiteralKind::Name }
            }

            // PDF literal string.
            // See ISO `32000-2:2008`, Section 7.3.4.2 Literal Strings.
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
            // See ISO `32000-2:2008`, Section 7.3.4.3 Hexadecimal Strings.
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
            // See ISO `32000-2:2008`, Section 7.2.4 Comments.
            b'%' => {
                self.eat_while(|b| !is_eol(b));
                TokenKind::Comment
            }

            // One-symbol tokens.
            b'[' => TokenKind::OpenBracket,  // See ISO `32000-2:2008`, Section 7.3.6 Array Objects.
            b']' => TokenKind::CloseBracket, // See ISO `32000-2:2008`, Section 7.3.6 Array Objects.
            b'<' => {
                // We ensured before that this is not a hex string.
                // See ISO `32000-2:2008`, Section 7.3.7 Dictionary Objects.
                self.next();
                TokenKind::OpenDict
            }
            b'>' if self.peek_first() == b'>' => {
                self.next();
                TokenKind::CloseDict // See ISO `32000-2:2008`, Section 7.3.7 Dictionary Objects.
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

/// See ISO `32000-2:2008`, Section 7.2.3 Character Set, Table 1 Whitespace characters.
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

/// See ISO `32000-2:2008`, Section 7.2.3 Character Set, Table 2 Delimiter characters.
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
