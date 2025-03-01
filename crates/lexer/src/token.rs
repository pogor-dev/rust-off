/// Parsed token.
/// It doesn't contain information about data that has been parsed,
/// only the type of the token and its size.
#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Token {
    pub fn new(kind: TokenKind, len: u32) -> Token {
        Token { kind, len }
    }
}

/// Enum representing common lexeme types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// Unknown token, not expected by the lexer.
    Unknown,

    /// End of line marker.
    Eol,

    /// Any whitespace character sequence.
    Whitespace,

    /// A comment token, e.g. `% This is a comment`.
    Comment,

    /// A identifier or kewyword token, e.g. `obj`, `endobj`, `stream`, `endstream`.
    Ident,

    /// A literal token, e.g. `123`, `3.14`.
    ///
    /// The [LiteralKind] enum contains information about the type of the literal.
    Literal { kind: LiteralKind },

    /// `[`
    OpenBracket,
    /// `]`
    CloseBracket,
    /// `<<`
    OpenDict,
    /// `>>`
    CloseDict,

    /// End of input.
    Eof,
}

/// Enum representing the literal types supported by the lexer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    /// `123`, `+123`, `-123`.
    Int,

    /// `3.14`, `+3.14`, `-3.14`, '3.', '.3'.
    Real,

    /// `/Name1`, `/ASomewhatLongerName`.
    Name, // TODO: add terminated flag

    /// Literal String `(This is a string)`, `(This is a string with \(escaped\) characters)`.
    LiteralString, // TODO: add terminated flag

    /// Hexadecimal String `<0123456789ABCDEF>`, `<0123456789abcdef>`.
    HexString, // TODO: add terminated flag
}
