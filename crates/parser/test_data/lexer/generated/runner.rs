mod ok {
    #![allow(unused_imports)]
    use crate::tests::*;
    #[test]
    fn pdf_7_2_3_character_set() { lex_and_expect_no_errors("test_data/lexer/ok/pdf_7_2_3_character_set.pdf"); }
    #[test]
    fn pdf_7_2_4_comments() { lex_and_expect_no_errors("test_data/lexer/ok/pdf_7_2_4_comments.pdf"); }
    #[test]
    fn pdf_7_3_10_indirect_objects() { lex_and_expect_no_errors("test_data/lexer/ok/pdf_7_3_10_indirect_objects.pdf"); }
    #[test]
    fn pdf_7_3_10_indirect_references() { lex_and_expect_no_errors("test_data/lexer/ok/pdf_7_3_10_indirect_references.pdf"); }
    #[test]
    fn pdf_7_3_2_booleans() { lex_and_expect_no_errors("test_data/lexer/ok/pdf_7_3_2_booleans.pdf"); }
    #[test]
    fn pdf_7_3_3_numbers() { lex_and_expect_no_errors("test_data/lexer/ok/pdf_7_3_3_numbers.pdf"); }
    #[test]
    fn pdf_7_3_4_strings() { lex_and_expect_no_errors("test_data/lexer/ok/pdf_7_3_4_strings.pdf"); }
    #[test]
    fn pdf_7_3_5_names() { lex_and_expect_no_errors("test_data/lexer/ok/pdf_7_3_5_names.pdf"); }
    #[test]
    fn pdf_7_3_6_arrays() { lex_and_expect_no_errors("test_data/lexer/ok/pdf_7_3_6_arrays.pdf"); }
    #[test]
    fn pdf_7_3_7_dictionaries() { lex_and_expect_no_errors("test_data/lexer/ok/pdf_7_3_7_dictionaries.pdf"); }
    #[test]
    fn pdf_7_3_8_streams() { lex_and_expect_no_errors("test_data/lexer/ok/pdf_7_3_8_streams.pdf"); }
    #[test]
    fn pdf_7_3_9_null() { lex_and_expect_no_errors("test_data/lexer/ok/pdf_7_3_9_null.pdf"); }
    #[test]
    fn pdf_7_5_4_xref_table() { lex_and_expect_no_errors("test_data/lexer/ok/pdf_7_5_4_xref_table.pdf"); }
    #[test]
    fn pdf_7_5_5_file_trailer() { lex_and_expect_no_errors("test_data/lexer/ok/pdf_7_5_5_file_trailer.pdf"); }
    #[test]
    fn safedocs_pdf_syntax_tests() { lex_and_expect_no_errors("test_data/lexer/ok/safedocs_pdf_syntax_tests.pdf"); }
}
mod err {
    #![allow(unused_imports)]
    use crate::tests::*;
}
