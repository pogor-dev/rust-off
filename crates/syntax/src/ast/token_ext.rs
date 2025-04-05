use crate::{
    ast::{self, AstToken},
    TextRange, TextSize,
};
use pdfc_lexer::unescape::{unescape_str, EscapeError, Mode};
use std::{borrow::Cow, num::ParseIntError, str};

impl ast::IntNumber {
    pub fn value(&self) -> Result<u128, ParseIntError> {
        u128::from_str_radix(self.text(), 10)
    }
}

impl ast::RealNumber {
    pub fn value_string(&self) -> String {
        self.text().to_string()
    }
}

impl ast::LiteralString {
    pub fn value(&self) -> Result<Cow<'_, str>, EscapeError> {
        let text = self.text();
        let text_range = self.text_range_between_quotes().ok_or(EscapeError::NonAsciiChar)?; // TODO: check if error is correct
        let text = &text[text_range - self.syntax().text_range().start()];

        let mut buf = String::new();
        let mut prev_end = 0;
        let mut has_error = None;
        unescape_str(
            text,
            Self::MODE,
            &mut |char_range, unescaped_char| match (unescaped_char, buf.capacity() == 0) {
                (Ok(c), false) => buf.push(c),
                (Ok(_), true) if char_range.len() == 1 && char_range.start == prev_end => prev_end = char_range.end,
                (Ok(c), true) => {
                    buf.reserve_exact(text.len());
                    buf.push_str(&text[..prev_end]);
                    buf.push(c);
                }
                (Err(e), _) => has_error = Some(e),
            },
        );

        match (has_error, buf.capacity() == 0) {
            (Some(e), _) => Err(e),
            (None, true) => Ok(Cow::Borrowed(text)),
            (None, false) => Ok(Cow::Owned(buf)),
        }
    }
}

pub trait IsLiteralString: AstToken {
    const MODE: Mode;
    fn quote_offsets(&self) -> Option<QuoteOffsets> {
        let text = self.text();
        let offsets = QuoteOffsets::new(text, "(", ")")?;
        let o = self.syntax().text_range().start();
        let offsets = QuoteOffsets {
            quotes: (offsets.quotes.0 + o, offsets.quotes.1 + o),
            contents: offsets.contents + o,
        };
        Some(offsets)
    }
    fn text_range_between_quotes(&self) -> Option<TextRange> {
        self.quote_offsets().map(|it| it.contents)
    }
}

impl IsLiteralString for ast::LiteralString {
    const MODE: Mode = Mode::LiteralString;
}

impl ast::HexString {
    pub fn value(&self) -> Result<Cow<'_, str>, EscapeError> {
        let text = self.text();
        let text_range = self.text_range_between_quotes().ok_or(EscapeError::NonAsciiChar)?; // TODO: check if error is correct
        let text = &text[text_range - self.syntax().text_range().start()];

        let mut buf = String::new();
        let mut prev_end = 0;
        let mut has_error = None;
        unescape_str(
            text,
            Self::MODE,
            &mut |char_range, unescaped_char| match (unescaped_char, buf.capacity() == 0) {
                (Ok(c), false) => buf.push(c),
                (Ok(_), true) if char_range.len() == 1 && char_range.start == prev_end => prev_end = char_range.end,
                (Ok(c), true) => {
                    buf.reserve_exact(text.len());
                    buf.push_str(&text[..prev_end]);
                    buf.push(c);
                }
                (Err(e), _) => has_error = Some(e),
            },
        );

        match (has_error, buf.capacity() == 0) {
            (Some(e), _) => Err(e),
            (None, true) => Ok(Cow::Borrowed(text)),
            (None, false) => Ok(Cow::Owned(buf)),
        }
    }
}

pub trait IsHexString: AstToken {
    const MODE: Mode;
    fn quote_offsets(&self) -> Option<QuoteOffsets> {
        let text = self.text();
        let offsets = QuoteOffsets::new(text, "<<", ">>")?;
        let o = self.syntax().text_range().start();
        let offsets = QuoteOffsets {
            quotes: (offsets.quotes.0 + o, offsets.quotes.1 + o),
            contents: offsets.contents + o,
        };
        Some(offsets)
    }
    fn text_range_between_quotes(&self) -> Option<TextRange> {
        self.quote_offsets().map(|it| it.contents)
    }
}

impl IsHexString for ast::HexString {
    const MODE: Mode = Mode::HexString;
}

impl ast::Name {
    pub fn value(&self) -> Result<Cow<'_, str>, EscapeError> {
        let text = self.text();
        let text_range = self.text_range_between_quotes().ok_or(EscapeError::NonAsciiChar)?; // TODO: check if error is correct
        let text = &text[text_range - self.syntax().text_range().start()];

        let mut buf = String::new();
        let mut prev_end = 0;
        let mut has_error = None;
        unescape_str(
            text,
            Self::MODE,
            &mut |char_range, unescaped_char| match (unescaped_char, buf.capacity() == 0) {
                (Ok(c), false) => buf.push(c),
                (Ok(_), true) if char_range.len() == 1 && char_range.start == prev_end => prev_end = char_range.end,
                (Ok(c), true) => {
                    buf.reserve_exact(text.len());
                    buf.push_str(&text[..prev_end]);
                    buf.push(c);
                }
                (Err(e), _) => has_error = Some(e),
            },
        );

        match (has_error, buf.capacity() == 0) {
            (Some(e), _) => Err(e),
            (None, true) => Ok(Cow::Borrowed(text)),
            (None, false) => Ok(Cow::Owned(buf)),
        }
    }
}

pub trait IsName: AstToken {
    const MODE: Mode;
    fn quote_offsets(&self) -> Option<QuoteOffsets> {
        let text = self.text();
        let offsets = QuoteOffsets::new(text, "<<", ">>")?;
        let o = self.syntax().text_range().start();
        let offsets = QuoteOffsets {
            quotes: (offsets.quotes.0 + o, offsets.quotes.1 + o),
            contents: offsets.contents + o,
        };
        Some(offsets)
    }
    fn text_range_between_quotes(&self) -> Option<TextRange> {
        self.quote_offsets().map(|it| it.contents)
    }
}

impl IsName for ast::Name {
    const MODE: Mode = Mode::Name;
}

#[derive(Debug)]
pub struct QuoteOffsets {
    pub quotes: (TextRange, TextRange),
    pub contents: TextRange,
}

impl QuoteOffsets {
    fn new(literal: &str, open_quote: &str, close_quote: &str) -> Option<QuoteOffsets> {
        let left_quote = literal.find(open_quote)?;
        let right_quote = literal.rfind(close_quote)?;
        if left_quote == right_quote {
            // `literal` only contains one quote
            return None;
        }

        let start = TextSize::from(0);
        let left_quote = TextSize::try_from(left_quote).unwrap() + TextSize::of(close_quote); // TODO: check if closing quote is correct
        let right_quote = TextSize::try_from(right_quote).unwrap();
        let end = TextSize::of(literal);

        let res = QuoteOffsets {
            quotes: (TextRange::new(start, left_quote), TextRange::new(right_quote, end)),
            contents: TextRange::new(left_quote, right_quote),
        };
        Some(res)
    }
}
