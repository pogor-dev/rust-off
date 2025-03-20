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
    text: &'a [u8],
    kind: Vec<SyntaxKind>,
    start: Vec<u32>,
    error: Vec<LexError>,
}

struct LexError {
    msg: String,
    token: u32,
}

impl<'a> LexedStr<'a> {
    pub fn new(edition: Edition, text: &'a [u8]) -> LexedStr<'a> {
        let _p = tracing::info_span!("LexedStr::new").entered();
        let mut conv = Converter::new(edition, text);
        let mut tokenized = pdfc_lexer::tokenize(text);

        while let Some(token) = tokenized.next() {
            let token_text = &text[conv.offset..][..token.len as usize];
            conv.extend_token(&token.kind, token_text);
        }

        conv.finalize_with_eof()
    }

    pub fn single_token(edition: Edition, text: &'a [u8]) -> Option<(SyntaxKind, Option<String>)> {
        if text.is_empty() {
            return None;
        }

        let token = pdfc_lexer::tokenize(text).next()?;
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

    pub fn as_str(&self) -> &[u8] {
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

    pub fn text(&self, i: usize) -> &[u8] {
        self.range_text(i..i + 1)
    }

    pub fn range_text(&self, r: ops::Range<usize>) -> &[u8] {
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
    fn new(edition: Edition, text: &'a [u8]) -> Self {
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

    fn extend_token(&mut self, kind: &pdfc_lexer::TokenKind, token_text: &[u8]) {
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
                pdfc_lexer::TokenKind::Ident => {
                    let token_text_str: String = token_text
                        .iter()
                        .map(|&c| if c.is_ascii() { (c as char).to_string() } else { format!("\\x{:02x}", c) })
                        .collect();

                    SyntaxKind::from_keyword(token_text_str.as_str(), self.edition).unwrap_or(ERROR)
                }
                pdfc_lexer::TokenKind::Literal { kind, .. } => {
                    self.extend_literal(token_text.len(), kind);
                    return;
                }
                pdfc_lexer::TokenKind::OpenBracket => T!['['],
                pdfc_lexer::TokenKind::CloseBracket => T![']'],
                pdfc_lexer::TokenKind::OpenDict => T![<<],
                pdfc_lexer::TokenKind::CloseDict => T![>>],
                pdfc_lexer::TokenKind::RawStreamData => RAW_STREAM,
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
