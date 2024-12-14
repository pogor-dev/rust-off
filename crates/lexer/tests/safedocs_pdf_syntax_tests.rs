#[cfg(test)]
mod tests {
    use pdfc_lexer::{tokenize, LiteralKind, Token, TokenKind};
    use std::{env, error::Error, fs, path::PathBuf};

    use csv::ReaderBuilder;

    /// ISO `32000-1:2008`, Section 7.2.3 Character Set.
    #[test]
    fn test_safedocs_pdf_syntax() {
        let safedocs_pdf_syntax = fs::read(project_root().join("./tests/safedocs_pdf_syntax_tests.pdf")).unwrap();
        let safedocs_pdf_syntax_expected_table = read_tsv(project_root().join("./tests/safedocs_pdf_syntax_tests.csv"));

        let tokens = test_input(safedocs_pdf_syntax.as_slice());
        let mut expected_table_iter = safedocs_pdf_syntax_expected_table.unwrap().into_iter();

        let mut offset: u32 = 0;

        for token in tokens {
            let expected = match expected_table_iter.next() {
                Some(expected) => expected,
                None => return, //panic!("Expected table is shorter than the number of tokens in the PDF."),
            };

            let parsed_string = String::from_utf8_lossy(&safedocs_pdf_syntax[offset as usize..(offset + token.len) as usize]);
            let start = if offset > 50 { offset as usize - 50 } else { 0 };
            let partial_string = String::from_utf8_lossy(&safedocs_pdf_syntax[start..(offset + token.len) as usize]);

            assert!(
                token.kind.to_token_kind_string() == expected.token_kind
                    && (if let TokenKind::Literal { kind } = token.kind { kind.to_literal_kind_string() == expected.literal_kind } else { expected.literal_kind.is_empty() })
                    && token.len == expected.length,
                "Token mismatch: \nExpected:\t\t\t{{ kind: {}, literal_kind: {}, length: {} }}\nFound:\t\t\t\t{{ kind: {}, literal_kind: {}, length: {} }}\nParsed string:\t\t{:?}\nPartial string:\t{:?}",
                expected.token_kind,
                if expected.literal_kind.is_empty() { "N/A".to_string() } else { expected.literal_kind.clone() },
                expected.length,
                token.kind.to_token_kind_string(),
                if let TokenKind::Literal { kind } = token.kind { kind.to_literal_kind_string() } else { "N/A".to_string() },
                token.len,
                parsed_string,
                partial_string
            );

            offset += token.len;
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
        env::current_dir().unwrap()
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
