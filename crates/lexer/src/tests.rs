use std::{fmt::Write, fs, path::PathBuf};

use super::*;
use expect_test::expect_file;

#[rustfmt::skip]
#[path = "../test_data/generated/runner.rs"]
mod runner;

fn lex(text: &[u8]) -> String {
    let lexed: Vec<Token> = tokenize(text).collect();

    let mut res = String::new();
    let mut offset = 0; // Initialize offset
    for i in 0..lexed.len() {
        let kind = lexed[i].kind;
        let len = lexed[i].len as usize;
        let token_text = &text[offset..offset + len]; // Compute text based on offset and len

        let escaped_text: String = token_text
            .iter()
            .map(|&c| {
                let ch = c as char;
                match ch {
                    ' ' | '\n' | '\r' | '\t' => ch.to_string(),
                    _ if ch.is_ascii_alphanumeric() || ch.is_ascii_punctuation() => ch.to_string(),
                    _ => format!("\\u{{{:02x}}}", c),
                }
            })
            .collect();

        writeln!(res, "{kind:?} {escaped_text:?}").unwrap();

        offset += len; // Update offset
    }
    res
}

#[track_caller]
fn run_and_expect_no_errors(path: &str) {
    let path = PathBuf::from(path);
    let text = fs::read(&path).unwrap().into_boxed_slice();
    let actual = lex(&text);
    let mut p = PathBuf::from("..");
    p.push(path);
    p.set_extension("rast");
    expect_file![p].assert_eq(&actual)
}
