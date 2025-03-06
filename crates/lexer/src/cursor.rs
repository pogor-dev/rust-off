use std::slice;

pub(crate) const EOF_BYTE: u8 = b'\0';

#[cfg(test)]
#[path = "./cursor.tests.rs"]
mod cursor_tests;

pub struct Cursor<'a> {
    len_remaining: usize,
    iter: slice::Iter<'a, u8>,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a [u8]) -> Cursor<'a> {
        Cursor {
            len_remaining: input.len(),
            iter: input.iter(),
        }
    }

    pub(crate) fn next(&mut self) -> Option<u8> {
        self.iter.next().copied()
    }

    pub(crate) fn peek_first(&self) -> u8 {
        self.iter.clone().next().copied().unwrap_or(EOF_BYTE)
    }

    #[allow(dead_code)]
    pub(crate) fn peek_second(&self) -> u8 {
        let mut iter = self.iter.clone();
        iter.next();
        iter.next().copied().unwrap_or(EOF_BYTE)
    }

    #[allow(dead_code)]
    pub(crate) fn peek_third(&self) -> u8 {
        let mut iter = self.iter.clone();
        iter.next();
        iter.next();
        iter.next().copied().unwrap_or(EOF_BYTE)
    }

    pub(crate) fn eat_while(&mut self, predicate: impl Fn(u8) -> bool) {
        while predicate(self.peek_first()) && !self.is_eof() {
            self.next();
        }
    }

    pub(crate) fn eat_while_word(&mut self, word: &[u8]) {
        while !self.is_eof() {
            if self.try_eat_word(word) {
                break;
            }
            self.next();
        }
    }

    pub(crate) fn try_eat_word(&mut self, word: &[u8]) -> bool {
        if word.len() > self.iter.as_slice().len() {
            return false;
        }

        let mut iter = self.iter.clone();
        for &b in word {
            let n = iter.next();
            if n != Some(&b) {
                return false;
            }
        }

        self.iter = iter;
        true
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.iter.clone().next().is_none()
    }

    pub(crate) fn pos_within_token(&self) -> u32 {
        (self.len_remaining - self.iter.as_slice().len()) as u32
    }

    pub(crate) fn reset_pos_within_token(&mut self) {
        self.len_remaining = self.iter.as_slice().len();
    }
}
