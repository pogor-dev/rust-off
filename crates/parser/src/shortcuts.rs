//! Shortcuts that span lexer/parser abstraction.
//!
//! The way Rust works, parser doesn't necessary parse text, and you might
//! tokenize text without parsing it further. So, it makes sense to keep
//! abstract token parsing, and string tokenization as completely separate
//! layers.
//!
//! However, often you do parse text into syntax trees and the glue code for
//! that needs to live somewhere. Rather than putting it to lexer or parser, we
//! use a separate shortcuts module for that.

use std::mem;

use crate::{
    Edition, LexedStr, Step,
    SyntaxKind::{self},
};

#[derive(Debug)]
pub enum StrStep<'a> {
    Token { kind: SyntaxKind, text: &'a [u8] },
    Enter { kind: SyntaxKind },
    Exit,
    Error { msg: &'a str, pos: usize },
}

impl LexedStr<'_> {
    pub fn to_input(&self, _edition: Edition) -> crate::Input {
        let _p = tracing::info_span!("LexedStr::to_input").entered();
        let mut res = crate::Input::default();
        let mut was_joint = false;
        for i in 0..self.len() {
            let kind = self.kind(i);
            if kind.is_trivia() {
                was_joint = false
            } else {
                if was_joint {
                    res.was_joint();
                }

                res.push(kind);

                // Tag the token as joint if it is float with a fractional part
                // we use this jointness to inform the parser about what token split
                // event to emit when we encounter a float literal in a field access
                if kind == SyntaxKind::REAL_NUMBER {
                    if !self.text(i).ends_with(b".") {
                        res.was_joint();
                    } else {
                        was_joint = false;
                    }
                } else {
                    was_joint = true;
                }
            }
        }
        res
    }

    /// NB: only valid to call with Output from Reparser/TopLevelEntry.
    pub fn intersperse_trivia(&self, output: &crate::Output, sink: &mut dyn FnMut(StrStep<'_>)) -> bool {
        let mut builder = Builder {
            lexed: self,
            pos: 0,
            state: State::PendingEnter,
            sink,
        };

        for event in output.iter() {
            match event {
                Step::Token {
                    kind,
                    n_input_tokens: n_raw_tokens,
                } => builder.token(kind, n_raw_tokens),
                Step::Enter { kind } => builder.enter(kind),
                Step::Exit => builder.exit(),
                Step::Error { msg } => {
                    let text_pos = builder.lexed.text_start(builder.pos);
                    (builder.sink)(StrStep::Error { msg, pos: text_pos });
                }
            }
        }

        match mem::replace(&mut builder.state, State::Normal) {
            State::PendingExit => {
                builder.eat_trivias();
                (builder.sink)(StrStep::Exit);
            }
            State::PendingEnter | State::Normal => unreachable!(),
        }

        // is_eof?
        builder.pos == builder.lexed.len()
    }
}

struct Builder<'a, 'b> {
    lexed: &'a LexedStr<'a>,
    pos: usize,
    state: State,
    sink: &'b mut dyn FnMut(StrStep<'_>),
}

enum State {
    PendingEnter,
    Normal,
    PendingExit,
}

impl Builder<'_, '_> {
    fn token(&mut self, kind: SyntaxKind, n_tokens: u8) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingEnter => unreachable!(),
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }
        self.eat_trivias();
        self.do_token(kind, n_tokens as usize);
    }

    fn enter(&mut self, kind: SyntaxKind) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingEnter => {
                (self.sink)(StrStep::Enter { kind });
                // No need to attach trivias to previous node: there is no
                // previous node.
                return;
            }
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }

        let n_trivias = (self.pos..self.lexed.len()).take_while(|&it| self.lexed.kind(it).is_trivia()).count();
        let leading_trivias = self.pos..self.pos + n_trivias;
        let n_attached_trivias = n_attached_trivias(kind, leading_trivias.rev().map(|it| (self.lexed.kind(it), self.lexed.text(it))));
        self.eat_n_trivias(n_trivias - n_attached_trivias);
        (self.sink)(StrStep::Enter { kind });
        self.eat_n_trivias(n_attached_trivias);
    }

    fn exit(&mut self) {
        match mem::replace(&mut self.state, State::PendingExit) {
            State::PendingEnter => unreachable!(),
            State::PendingExit => (self.sink)(StrStep::Exit),
            State::Normal => (),
        }
    }

    fn eat_trivias(&mut self) {
        while self.pos < self.lexed.len() {
            let kind = self.lexed.kind(self.pos);
            if !kind.is_trivia() {
                break;
            }
            self.do_token(kind, 1);
        }
    }

    fn eat_n_trivias(&mut self, n: usize) {
        for _ in 0..n {
            let kind = self.lexed.kind(self.pos);
            assert!(kind.is_trivia());
            self.do_token(kind, 1);
        }
    }

    fn do_token(&mut self, kind: SyntaxKind, n_tokens: usize) {
        let text = &self.lexed.range_text(self.pos..self.pos + n_tokens);
        self.pos += n_tokens;
        (self.sink)(StrStep::Token { kind, text });
    }
}

fn n_attached_trivias<'a>(kind: SyntaxKind, _trivias: impl Iterator<Item = (SyntaxKind, &'a [u8])>) -> usize {
    match kind {
        _ => 0,
    }
}

fn split_once_u8(slice: &[u8], delimiter: u8) -> Option<(&[u8], &[u8])> {
    let mut split_iter = slice.splitn(2, |&byte| byte == delimiter);
    let first = split_iter.next()?;
    let second = split_iter.next()?;
    Some((first, second))
}
