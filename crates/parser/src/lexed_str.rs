//! Lexing `&str` into a sequence of PDF tokens.
//!
//! Note that strictly speaking the parser in this crate is not required to work
//! on tokens which originated from text. Macros, eg, can synthesize tokens out
//! of thin air. So, ideally, lexer should be an orthogonal crate. It is however
//! convenient to include a text-based lexer here!
//!
//! Note that these tokens, unlike the tokens we feed into the parser, do
//! include info about comments and whitespace.

// TODO: adjust docs

use std::ops;

use crate::{
    Edition,
    SyntaxKind::{self, *},
    T,
};

pub struct LexedStr<'a> {
    text: &'a str,
    kind: Vec<SyntaxKind>,
    start: Vec<u32>,
    error: Vec<LexError>,
}

struct LexError {
    msg: String,
    token: u32,
}

impl<'a> LexedStr<'a> {
    pub fn new(edition: Edition, text: &'a str) -> LexedStr<'a> {
        let _p = tracing::info_span!("LexedStr::new").entered();
        let mut conv = Converter::new(edition, text);

        // Re-create the tokenizer from scratch every token because `GuardedStrPrefix` is one token in the lexer
        // but we want to split it to two in edition <2024.
        while let Some(token) = pdfc_lexer::tokenize(text[conv.offset..].as_bytes()).next() {
            let token_text = &text[conv.offset..][..token.len as usize];

            conv.extend_token(&token.kind, token_text);
        }

        conv.finalize_with_eof()
    }

    pub fn single_token(edition: Edition, text: &'a str) -> Option<(SyntaxKind, Option<String>)> {
        if text.is_empty() {
            return None;
        }

        let token = pdfc_lexer::tokenize(text.as_bytes()).next()?;
        if token.len as usize != text.len() {
            return None;
        }

        let mut conv = Converter::new(edition, text);
        conv.extend_token(&token.kind, text);
        match &*conv.res.kind {
            [kind] => Some((*kind, conv.res.error.pop().map(|it| it.msg))),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        self.text
    }

    pub fn len(&self) -> usize {
        self.kind.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn kind(&self, i: usize) -> SyntaxKind {
        assert!(i < self.len());
        self.kind[i]
    }

    pub fn text(&self, i: usize) -> &str {
        self.range_text(i..i + 1)
    }

    pub fn range_text(&self, r: ops::Range<usize>) -> &str {
        assert!(r.start < r.end && r.end <= self.len());
        let lo = self.start[r.start] as usize;
        let hi = self.start[r.end] as usize;
        &self.text[lo..hi]
    }

    // Naming is hard.
    pub fn text_range(&self, i: usize) -> ops::Range<usize> {
        assert!(i < self.len());
        let lo = self.start[i] as usize;
        let hi = self.start[i + 1] as usize;
        lo..hi
    }
    pub fn text_start(&self, i: usize) -> usize {
        assert!(i <= self.len());
        self.start[i] as usize
    }
    pub fn text_len(&self, i: usize) -> usize {
        assert!(i < self.len());
        let r = self.text_range(i);
        r.end - r.start
    }

    pub fn error(&self, i: usize) -> Option<&str> {
        assert!(i < self.len());
        let err = self.error.binary_search_by_key(&(i as u32), |i| i.token).ok()?;
        Some(self.error[err].msg.as_str())
    }

    pub fn errors(&self) -> impl Iterator<Item = (usize, &str)> + '_ {
        self.error.iter().map(|it| (it.token as usize, it.msg.as_str()))
    }

    fn push(&mut self, kind: SyntaxKind, offset: usize) {
        self.kind.push(kind);
        self.start.push(offset as u32);
    }
}

struct Converter<'a> {
    res: LexedStr<'a>,
    offset: usize,
    edition: Edition,
}

impl<'a> Converter<'a> {
    fn new(edition: Edition, text: &'a str) -> Self {
        Self {
            res: LexedStr {
                text,
                kind: Vec::new(),
                start: Vec::new(),
                error: Vec::new(),
            },
            offset: 0,
            edition,
        }
    }

    fn finalize_with_eof(mut self) -> LexedStr<'a> {
        self.res.push(EOF, self.offset);
        self.res
    }

    fn push(&mut self, kind: SyntaxKind, len: usize, err: Option<&str>) {
        self.res.push(kind, self.offset);
        self.offset += len;

        if let Some(err) = err {
            let token = self.res.len() as u32;
            let msg = err.to_owned();
            self.res.error.push(LexError { msg, token });
        }
    }

    fn extend_token(&mut self, kind: &pdfc_lexer::TokenKind, token_text: &str) {
        // A note on an intended tradeoff:
        // We drop some useful information here, namely the exact text of the token.
        // Storing that info in `SyntaxKind` is not possible due to its layout requirements of
        // being `u16` that come from `rowan::SyntaxKind`.
        let err = "";

        let syntax_kind = {
            match kind {
                pdfc_lexer::TokenKind::Unknown => ERROR,
                pdfc_lexer::TokenKind::Eol => NEWLINE,
                pdfc_lexer::TokenKind::Whitespace => WHITESPACE,
                pdfc_lexer::TokenKind::Comment => COMMENT,

                // Keywords that are not recognized by the parser are treated as errors.
                pdfc_lexer::TokenKind::Ident => SyntaxKind::from_keyword(token_text, self.edition).unwrap_or(ERROR),

                pdfc_lexer::TokenKind::Literal { kind, .. } => {
                    self.extend_literal(token_text.len(), kind);
                    return;
                }

                pdfc_lexer::TokenKind::OpenBracket => T!['['],
                pdfc_lexer::TokenKind::CloseBracket => T![']'],
                pdfc_lexer::TokenKind::OpenDict => T![<<],
                pdfc_lexer::TokenKind::CloseDict => T![>>],
                pdfc_lexer::TokenKind::Eof => EOF,
            }
        };

        let err = if err.is_empty() { None } else { Some(err) };
        self.push(syntax_kind, token_text.len(), err);
    }

    fn extend_literal(&mut self, len: usize, kind: &pdfc_lexer::LiteralKind) {
        let err = "";

        let syntax_kind = match *kind {
            pdfc_lexer::LiteralKind::Int => INT_NUMBER,
            pdfc_lexer::LiteralKind::Real => REAL_NUMBER,
            pdfc_lexer::LiteralKind::Name => NAME,
            pdfc_lexer::LiteralKind::LiteralString => LITERAL_STRING,
            pdfc_lexer::LiteralKind::HexString => HEX_STRING,
        };

        let err = if err.is_empty() { None } else { Some(err) };
        self.push(syntax_kind, len, err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Edition;

    fn lex(edition: Edition, text: &str) -> LexedStr {
        LexedStr::new(edition, text)
    }

    #[test]
    fn test_empty_input() {
        let lexed = lex(Edition::Pdf20, "");
        assert_eq!(lexed.len(), 0);
        assert!(lexed.is_empty());
    }

    #[test]
    fn test_single_token() {
        let lexed = lex(Edition::Pdf20, "123");
        assert_eq!(lexed.len(), 1);
        assert_eq!(lexed.kind(0), SyntaxKind::INT_NUMBER);
        assert_eq!(lexed.text(0), "123");
    }

    #[test]
    fn test_multiple_tokens() {
        let lexed = lex(Edition::Pdf20, "123 456");
        assert_eq!(lexed.len(), 3);
        assert_eq!(lexed.kind(0), SyntaxKind::INT_NUMBER);
        assert_eq!(lexed.kind(1), SyntaxKind::WHITESPACE);
        assert_eq!(lexed.kind(2), SyntaxKind::INT_NUMBER);
        assert_eq!(lexed.text(0), "123");
        assert_eq!(lexed.text(1), " ");
        assert_eq!(lexed.text(2), "456");
    }

    #[test]
    fn test_error_token() {
        let lexed = lex(Edition::Pdf20, "123 abc");
        assert_eq!(lexed.len(), 3);
        assert_eq!(lexed.kind(0), SyntaxKind::INT_NUMBER);
        assert_eq!(lexed.kind(1), SyntaxKind::WHITESPACE);
        assert_eq!(lexed.kind(2), SyntaxKind::ERROR);
        assert_eq!(lexed.text(0), "123");
        assert_eq!(lexed.text(1), " ");
        assert_eq!(lexed.text(2), "abc");
        // assert!(lexed.error(2).is_some()); // TODO: error handling for unrecognized keywords
    }

    #[test]
    fn test_comments_and_whitespace() {
        let lexed = lex(Edition::Pdf20, "123 % comment\n456");
        assert_eq!(lexed.len(), 5);
        assert_eq!(lexed.kind(0), SyntaxKind::INT_NUMBER);
        assert_eq!(lexed.kind(1), SyntaxKind::WHITESPACE);
        assert_eq!(lexed.kind(2), SyntaxKind::COMMENT);
        assert_eq!(lexed.kind(3), SyntaxKind::NEWLINE);
        assert_eq!(lexed.kind(4), SyntaxKind::INT_NUMBER);
        assert_eq!(lexed.text(0), "123");
        assert_eq!(lexed.text(1), " ");
        assert_eq!(lexed.text(2), "% comment");
        assert_eq!(lexed.text(3), "\n");
        assert_eq!(lexed.text(4), "456");
    }

    #[test]
    fn test_literal_strings() {
        let lexed = lex(Edition::Pdf20, "(This is a string)");
        assert_eq!(lexed.len(), 1);
        assert_eq!(lexed.kind(0), SyntaxKind::LITERAL_STRING);
        assert_eq!(lexed.text(0), "(This is a string)");
    }

    #[test]
    fn test_hex_strings() {
        let lexed = lex(Edition::Pdf20, "<4E6F762073686D6F7A>");
        assert_eq!(lexed.len(), 1);
        assert_eq!(lexed.kind(0), SyntaxKind::HEX_STRING);
        assert_eq!(lexed.text(0), "<4E6F762073686D6F7A>");
    }

    #[test]
    fn test_arrays() {
        let lexed = lex(Edition::Pdf20, "[ 1 (2) 3 ]");
        assert_eq!(lexed.len(), 9);
        assert_eq!(lexed.kind(0), SyntaxKind::L_BRACK);
        assert_eq!(lexed.kind(1), SyntaxKind::WHITESPACE);
        assert_eq!(lexed.kind(2), SyntaxKind::INT_NUMBER);
        assert_eq!(lexed.kind(3), SyntaxKind::WHITESPACE);
        assert_eq!(lexed.kind(4), SyntaxKind::LITERAL_STRING);
        assert_eq!(lexed.kind(5), SyntaxKind::WHITESPACE);
        assert_eq!(lexed.kind(6), SyntaxKind::INT_NUMBER);
        assert_eq!(lexed.kind(7), SyntaxKind::WHITESPACE);
        assert_eq!(lexed.kind(8), SyntaxKind::R_BRACK);
        assert_eq!(lexed.text(0), "[");
        assert_eq!(lexed.text(1), " ");
        assert_eq!(lexed.text(2), "1");
        assert_eq!(lexed.text(3), " ");
        assert_eq!(lexed.text(4), "(2)");
        assert_eq!(lexed.text(5), " ");
        assert_eq!(lexed.text(6), "3");
        assert_eq!(lexed.text(7), " ");
        assert_eq!(lexed.text(8), "]");
    }

    #[test]
    fn test_dictionaries() {
        let lexed = lex(Edition::Pdf20, "<< /Type /Catalog /Pages 3 0 R >>");
        assert_eq!(lexed.len(), 15);
        assert_eq!(lexed.kind(0), SyntaxKind::L_DICT);
        assert_eq!(lexed.kind(1), SyntaxKind::WHITESPACE);
        assert_eq!(lexed.kind(2), SyntaxKind::NAME);
        assert_eq!(lexed.kind(3), SyntaxKind::WHITESPACE);
        assert_eq!(lexed.kind(4), SyntaxKind::NAME);
        assert_eq!(lexed.kind(5), SyntaxKind::WHITESPACE);
        assert_eq!(lexed.kind(6), SyntaxKind::NAME);
        assert_eq!(lexed.kind(7), SyntaxKind::WHITESPACE);
        assert_eq!(lexed.kind(8), SyntaxKind::INT_NUMBER);
        assert_eq!(lexed.kind(9), SyntaxKind::WHITESPACE);
        assert_eq!(lexed.kind(10), SyntaxKind::INT_NUMBER);
        assert_eq!(lexed.kind(11), SyntaxKind::WHITESPACE);
        assert_eq!(lexed.kind(12), SyntaxKind::R_KW);
        assert_eq!(lexed.kind(13), SyntaxKind::WHITESPACE);
        assert_eq!(lexed.kind(14), SyntaxKind::R_DICT);
        assert_eq!(lexed.text(0), "<<");
        assert_eq!(lexed.text(1), " ");
        assert_eq!(lexed.text(2), "/Type");
        assert_eq!(lexed.text(3), " ");
        assert_eq!(lexed.text(4), "/Catalog");
        assert_eq!(lexed.text(5), " ");
        assert_eq!(lexed.text(6), "/Pages");
        assert_eq!(lexed.text(7), " ");
        assert_eq!(lexed.text(8), "3");
        assert_eq!(lexed.text(9), " ");
        assert_eq!(lexed.text(10), "0");
        assert_eq!(lexed.text(11), " ");
        assert_eq!(lexed.text(12), "R");
        assert_eq!(lexed.text(13), " ");
        assert_eq!(lexed.text(14), ">>");
    }

    #[test]
    fn test_names() {
        let lexed = lex(Edition::Pdf20, "/Name1");
        assert_eq!(lexed.len(), 1);
        assert_eq!(lexed.kind(0), SyntaxKind::NAME);
        assert_eq!(lexed.text(0), "/Name1");

        let lexed = lex(Edition::Pdf20, "/ASomewhatLongerName");
        assert_eq!(lexed.len(), 1);
        assert_eq!(lexed.kind(0), SyntaxKind::NAME);
        assert_eq!(lexed.text(0), "/ASomewhatLongerName");
    }
}
