/// Parsed token.
/// It doesn't contain information about data that has been parsed,
/// only the type of the token and its size.
#[derive(Debug)]
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

    /// End of input.
    Eof,
}