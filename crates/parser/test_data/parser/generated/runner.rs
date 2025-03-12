mod ok {
    use crate::tests::*;
    #[test]
    fn pdf_7_3_10_indirect_references() { parse_and_expect_no_errors("test_data/parser/ok/pdf_7_3_10_indirect_references.pdf"); }
    #[test]
    fn pdf_7_3_2_booleans() { parse_and_expect_no_errors("test_data/parser/ok/pdf_7_3_2_booleans.pdf"); }
}
mod err {
    use crate::tests::*;
}
