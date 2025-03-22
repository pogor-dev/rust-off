mod ok {
    use crate::tests::*;
    #[test]
    fn pdf_7_2_3_character_set() { parse_and_expect_no_errors("test_data/parser/ok/pdf_7_2_3_character_set.pdf"); }
    #[test]
    fn pdf_7_2_4_comments() { parse_and_expect_no_errors("test_data/parser/ok/pdf_7_2_4_comments.pdf"); }
    #[test]
    fn pdf_7_3_10_indirect_objects() { parse_and_expect_no_errors("test_data/parser/ok/pdf_7_3_10_indirect_objects.pdf"); }
    #[test]
    fn pdf_7_3_10_indirect_references() { parse_and_expect_no_errors("test_data/parser/ok/pdf_7_3_10_indirect_references.pdf"); }
    #[test]
    fn pdf_7_3_2_booleans() { parse_and_expect_no_errors("test_data/parser/ok/pdf_7_3_2_booleans.pdf"); }
    #[test]
    fn pdf_7_3_3_numbers() { parse_and_expect_no_errors("test_data/parser/ok/pdf_7_3_3_numbers.pdf"); }
    #[test]
    fn pdf_7_3_4_strings() { parse_and_expect_no_errors("test_data/parser/ok/pdf_7_3_4_strings.pdf"); }
    #[test]
    fn pdf_7_3_5_names() { parse_and_expect_no_errors("test_data/parser/ok/pdf_7_3_5_names.pdf"); }
    #[test]
    fn pdf_7_3_6_arrays() { parse_and_expect_no_errors("test_data/parser/ok/pdf_7_3_6_arrays.pdf"); }
    #[test]
    fn pdf_7_3_7_dictionaries() { parse_and_expect_no_errors("test_data/parser/ok/pdf_7_3_7_dictionaries.pdf"); }
    #[test]
    fn pdf_7_3_8_streams() { parse_and_expect_no_errors("test_data/parser/ok/pdf_7_3_8_streams.pdf"); }
    #[test]
    fn pdf_7_3_9_null() { parse_and_expect_no_errors("test_data/parser/ok/pdf_7_3_9_null.pdf"); }
    #[test]
    fn pdf_7_5_4_xref_table() { parse_and_expect_no_errors("test_data/parser/ok/pdf_7_5_4_xref_table.pdf"); }
    #[test]
    fn safedocs_pdf_syntax_tests() { parse_and_expect_no_errors("test_data/parser/ok/safedocs_pdf_syntax_tests.pdf"); }
}
mod err {
    use crate::tests::*;
}
