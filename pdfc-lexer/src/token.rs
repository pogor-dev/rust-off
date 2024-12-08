/// Parsed token.
/// It doesn't contain information about data that has been parsed,
/// only the type of the token and its size.
#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, len: u32) -> Token {
        Token { kind, len }
    }
}

/// Enum representing common lexeme types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// A kewyword token, e.g. `obj`, `endobj`, `stream`, `endstream`.
    Keyword,

    /// Unknown token, not expected by the lexer.
    Unknown,

    /// Any whitespace character sequence.
    Whitespace,

    /// A literal token, e.g. `123`, `3.14`.
    ///
    /// The [LiteralKind] enum contains information about the type of the literal.
    Literal { kind: LiteralKind },

    /// `(`
    OpenParen,
    /// `)`
    CloseParen,
    /// `<`
    LessThan,
    /// `>`
    GreaterThan,
    /// `{`
    OpenCurly,
    /// `}`
    CloseCurly,
    /// `[`
    OpenSquare,
    /// `]`
    CloseSquare,
    /// `<<`
    DoubleLessThan,
    /// `>>`
    DoubleGreaterThan,
    /// `-`
    Minus,
    /// `+`
    Plus,

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
}
