mod ok {
    use crate::tests::*;
    #[test]
    fn pdf_7_2_3_character_set() { run_and_expect_no_errors("test_data/ok/pdf_7_2_3_character_set.pdf"); }
    #[test]
    fn pdf_7_2_4_comments() { run_and_expect_no_errors("test_data/ok/pdf_7_2_4_comments.pdf"); }
    #[test]
    fn pdf_7_3_10_indirect_objects() { run_and_expect_no_errors("test_data/ok/pdf_7_3_10_indirect_objects.pdf"); }
    #[test]
    fn pdf_7_3_2_booleans() { run_and_expect_no_errors("test_data/ok/pdf_7_3_2_booleans.pdf"); }
    #[test]
    fn pdf_7_3_3_numbers() { run_and_expect_no_errors("test_data/ok/pdf_7_3_3_numbers.pdf"); }
    #[test]
    fn pdf_7_3_4_strings() { run_and_expect_no_errors("test_data/ok/pdf_7_3_4_strings.pdf"); }
    #[test]
    fn pdf_7_3_5_names() { run_and_expect_no_errors("test_data/ok/pdf_7_3_5_names.pdf"); }
    #[test]
    fn pdf_7_3_6_arrays() { run_and_expect_no_errors("test_data/ok/pdf_7_3_6_arrays.pdf"); }
    #[test]
    fn pdf_7_3_7_dictionaries() { run_and_expect_no_errors("test_data/ok/pdf_7_3_7_dictionaries.pdf"); }
    #[test]
    fn pdf_7_3_8_streams() { run_and_expect_no_errors("test_data/ok/pdf_7_3_8_streams.pdf"); }
    #[test]
    fn pdf_7_3_9_null() { run_and_expect_no_errors("test_data/ok/pdf_7_3_9_null.pdf"); }
    #[test]
    fn pdf_7_5_4_xref_table() { run_and_expect_no_errors("test_data/ok/pdf_7_5_4_xref_table.pdf"); }
    #[test]
    fn pdf_7_5_5_file_trailer() { run_and_expect_no_errors("test_data/ok/pdf_7_5_5_file_trailer.pdf"); }
    #[test]
    fn safedocs_pdf_syntax_tests() { run_and_expect_no_errors("test_data/ok/safedocs_pdf_syntax_tests.pdf"); }
}
mod err {
    use crate::tests::*;
}
