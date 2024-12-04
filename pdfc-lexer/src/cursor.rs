use std::slice;

pub(crate) const EOF_BYTE: u8 = b'\0';

pub struct Cursor<'a> {
    iter: slice::Iter<'a, u8>,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a [u8]) -> Cursor<'a> {
        Cursor { iter: input.iter() }
    }

    pub(crate) fn next(&mut self) -> u8 {
        self.iter.next().copied().unwrap_or(EOF_BYTE)
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

    pub(crate) fn is_eof(&self) -> bool {
        self.iter.clone().next().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_returns_first_byte() {
        let mut cursor = Cursor::new(b"%PDF-1.7");
        assert_eq!(cursor.next(), b'%');
    }

    #[test]
    fn next_called_twice_returns_second_byte() {
        let mut cursor = Cursor::new(b"%PDF-1.7");
        cursor.next();
        assert_eq!(cursor.next(), b'P');
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
}