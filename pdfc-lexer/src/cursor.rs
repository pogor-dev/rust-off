use std::slice;

pub(crate) const EOF_BYTE: u8 = b'\0';

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

    pub(crate) fn peek_second(&self) -> u8 {
        let mut iter = self.iter.clone();
        iter.next();
        iter.next().copied().unwrap_or(EOF_BYTE)
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_returns_none_when_input_is_empty() {
        let mut cursor = Cursor::new(b"");
        assert_eq!(cursor.next(), None);
    }

    #[test]
    fn next_returns_first_byte() {
        let mut cursor = Cursor::new(b"%PDF-1.7");
        assert_eq!(cursor.next(), Some(b'%'));
    }

    #[test]
    fn next_called_twice_returns_second_byte() {
        let mut cursor = Cursor::new(b"%PDF-1.7");
        cursor.next();
        assert_eq!(cursor.next(), Some(b'P'));
    }

    #[test]
    fn peek_first_returns_first_byte() {
        let cursor = Cursor::new(b"%PDF-1.7");
        assert_eq!(cursor.peek_first(), b'%');
    }

    #[test]
    fn peek_second_returns_second_byte() {
        let cursor = Cursor::new(b"%PDF-1.7");
        assert_eq!(cursor.peek_second(), b'P');
    }

    #[test]
    fn peek_third_returns_third_byte() {
        let cursor = Cursor::new(b"%PDF-1.7");
        assert_eq!(cursor.peek_third(), b'D');
    }

    #[test]
    fn is_eof_returns_true_when_input_is_empty() {
        let cursor = Cursor::new(b"");
        assert_eq!(cursor.is_eof(), true);
    }
    #[test]
    fn is_eof_returns_false_when_input_is_not_empty() {
        let cursor = Cursor::new(b"%PDF-1.7");
        assert_eq!(cursor.is_eof(), false);
    }

    #[test]
    fn is_eof_returns_true_when_iterator_is_at_end() {
        let mut cursor = Cursor::new(b"%");
        cursor.next();
        assert_eq!(cursor.is_eof(), true);
    }

    #[test]
    fn len_remaining_is_initially_set_to_input_length() {
        let cursor = Cursor::new(b"%PDF-1.7");
        assert_eq!(cursor.len_remaining, 8);
    }

    #[test]
    fn pos_within_token_returns_0_when_no_bytes_have_been_read() {
        let cursor = Cursor::new(b"%PDF-1.7");
        assert_eq!(cursor.pos_within_token(), 0);
    }

    #[test]
    fn pos_within_token_returns_number_of_bytes_read() {
        let mut cursor = Cursor::new(b"%PDF-1.7");
        cursor.next();
        cursor.next();
        assert_eq!(cursor.pos_within_token(), 2);
    }

    #[test]
    fn reset_pos_sets_len_remaining_to_iterator_length() {
        let mut cursor = Cursor::new(b"%PDF-1.7");
        cursor.next();
        cursor.next();
        cursor.reset_pos_within_token();
        assert_eq!(cursor.len_remaining, 6);
    }

    #[test]
    fn eat_while_consumes_bytes_while_predicate_is_true() {
        let mut cursor = Cursor::new(b"PDF-1.7");
        cursor.eat_while(|b| b.is_ascii_alphabetic());
        assert_eq!(cursor.pos_within_token(), 3);
    }
}
