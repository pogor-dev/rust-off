#[cfg(test)]
mod tests {
    use crate::Cursor;

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

    #[test]
    fn try_eat_word_returns_true_when_word_matches() {
        let mut cursor = Cursor::new(b"PDF-1.7");
        assert!(cursor.try_eat_word(b"PDF"));
        assert_eq!(cursor.pos_within_token(), 3);
    }

    #[test]
    fn try_eat_word_returns_false_when_word_does_not_match() {
        let mut cursor = Cursor::new(b"PDF-1.7");
        assert!(!cursor.try_eat_word(b"PDX"));
        assert_eq!(cursor.pos_within_token(), 0);
    }

    #[test]
    fn try_eat_word_returns_false_when_word_is_longer_than_remaining_input() {
        let mut cursor = Cursor::new(b"PDF");
        assert!(!cursor.try_eat_word(b"PDF-1.7"));
        assert_eq!(cursor.pos_within_token(), 0);
    }

    #[test]
    fn try_eat_word_advances_iterator_when_word_matches() {
        let mut cursor = Cursor::new(b"PDF-1.7");
        assert!(cursor.try_eat_word(b"PDF"));
        assert_eq!(cursor.next(), Some(b'-'));
    }

    #[test]
    fn try_eat_word_does_not_advance_iterator_when_word_does_not_match() {
        let mut cursor = Cursor::new(b"PDF-1.7");
        assert!(!cursor.try_eat_word(b"PDX"));
        assert_eq!(cursor.next(), Some(b'P'));
    }

    #[test]
    fn eat_while_word_consumes_until_word_is_found() {
        let mut cursor = Cursor::new(b"123PDF456");
        cursor.eat_while_word(|s| s.starts_with(b"PDF"));
        assert_eq!(cursor.pos_within_token(), 3);
    }

    #[test]
    fn eat_while_word_does_consume_if_word_is_at_start() {
        let mut cursor = Cursor::new(b"PDF123456");
        cursor.eat_while_word(|s| s.starts_with(b"PDF"));
        assert_eq!(cursor.pos_within_token(), 0);
    }

    #[test]
    fn eat_while_word_consumes_entire_input_if_word_not_found() {
        let mut cursor = Cursor::new(b"123456");
        cursor.eat_while_word(|s| s.starts_with(b"PDF"));
        assert_eq!(cursor.pos_within_token(), 6);
    }

    #[test]
    fn eat_while_word_stops_at_first_occurrence_of_word() {
        let mut cursor = Cursor::new(b"123PDFPDF456");
        cursor.eat_while_word(|s| s.starts_with(b"PDF"));
        assert_eq!(cursor.pos_within_token(), 3);
    }
}
