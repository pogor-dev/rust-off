#[cfg(test)]
mod tests {
    use pdfc_lexer::{tokenize, LiteralKind, Token, TokenKind};
    use std::{env, error::Error, fs, path::PathBuf};

    use csv::ReaderBuilder;

    /// ISO `32000-1:2008`, Section 7.2.3 Character Set.
    #[test]
    fn test_safedocs_pdf_syntax() {
        let safedocs_pdf_syntax = fs::read(project_root().join("crates/lexer/tests/safedocs_pdf_syntax_tests.pdf")).unwrap();
        let safedocs_pdf_syntax_expected_table = read_tsv(project_root().join("crates/lexer/tests/safedocs_pdf_syntax_tests.csv"));

        let tokens = test_input(safedocs_pdf_syntax.as_slice());
        let mut expected_table_iter = safedocs_pdf_syntax_expected_table.unwrap().into_iter();

        for token in tokens {
            let expected = match expected_table_iter.next() {
                Some(expected) => expected,
                None => return, //panic!("Expected table is shorter than the number of tokens in the PDF."),
            };

            assert_eq!(token.kind.to_token_kind_string(), expected.token_kind);

            if let TokenKind::Literal { kind } = token.kind {
                assert_eq!(kind.to_literal_kind_string(), expected.literal_kind);
            }

            assert_eq!(token.len, expected.length);
        }
    }

    trait ToTokenKindString {
        fn to_token_kind_string(&self) -> String;
    }

    impl ToTokenKindString for TokenKind {
        fn to_token_kind_string(&self) -> String {
            let debug_str = format!("{:?}", self); // Get the Debug output
            debug_str.split_whitespace().next().unwrap().to_string()
        }
    }

    trait ToLiteralKind {
        fn to_literal_kind_string(&self) -> String;
    }

    impl ToLiteralKind for LiteralKind {
        fn to_literal_kind_string(&self) -> String {
            format!("{:?}", self)
        }
    }

    fn test_input(input: &[u8]) -> Vec<Token> {
        tokenize(input).collect()
    }

    fn project_root() -> PathBuf {
        env::current_dir().unwrap().parent().unwrap().parent().unwrap().to_path_buf()
    }

    fn read_tsv(file_path: PathBuf) -> Result<Vec<Record>, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new().delimiter(b',').from_path(file_path)?;
        let mut records = Vec::new();
        for result in rdr.deserialize() {
            let record: Record = result?;
            records.push(record);
        }
        Ok(records)
    }

    #[derive(Debug, serde::Deserialize)]
    struct Record {
        token_kind: String,
        literal_kind: String,
        length: u32,
    }
}
