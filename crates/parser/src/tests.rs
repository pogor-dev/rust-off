use std::{fmt::Write, fs, path::PathBuf};

use expect_test::expect_file;

use crate::{Edition, LexedStr, TopEntryPoint};

#[rustfmt::skip]
#[allow(unused_imports)]
#[path = "../test_data/lexer/generated/runner.rs"]
mod lexer_runner;

#[rustfmt::skip]
#[allow(unused_imports)]
#[path = "../test_data/parser/generated/runner.rs"]
mod parser_runner;

fn lex(text: &[u8]) -> String {
    let lexed = LexedStr::new(Edition::LATEST, text);

    let mut res = String::new();
    for i in 0..lexed.len() {
        let kind = lexed.kind(i);
        let text = lexed.text(i);
        let error = lexed.error(i);

        let escaped_text = escape_bytes(text);

        let error = error.map(|err| format!(" error: {err}")).unwrap_or_default();
        writeln!(res, "{kind:?} {escaped_text:?}{error}").unwrap();
    }
    res
}

fn escape_bytes(text: &[u8]) -> String {
    let escaped_text: String = text
        .iter()
        .map(|&c| if c.is_ascii() { (c as char).to_string() } else { format!("\\x{:02x}", c) })
        .collect();

    escaped_text
}

fn parse(entry: TopEntryPoint, text: &[u8], edition: Edition) -> (String, bool) {
    let lexed = LexedStr::new(edition, text);
    let input = lexed.to_input(edition);
    let output = entry.parse(&input, edition);

    let mut buf = String::new();
    let mut errors = Vec::new();
    let mut indent = String::new();
    let mut depth = 0;
    let mut len = 0;
    lexed.intersperse_trivia(&output, &mut |step| match step {
        crate::StrStep::Token { kind, text } => {
            assert!(depth > 0);
            len += text.len();
            let text = escape_bytes(text);
            writeln!(buf, "{indent}{kind:?} {text:?}").unwrap();
        }
        crate::StrStep::Enter { kind } => {
            assert!(depth > 0 || len == 0);
            depth += 1;
            writeln!(buf, "{indent}{kind:?}").unwrap();
            indent.push_str("  ");
        }
        crate::StrStep::Exit => {
            assert!(depth > 0);
            depth -= 1;
            indent.pop();
            indent.pop();
        }
        crate::StrStep::Error { msg, pos } => {
            assert!(depth > 0);
            errors.push(format!("error {pos}: {msg}\n"))
        }
    });
    assert_eq!(len, text.len(), "didn't parse all text.\nParsed:\n{:?}\n\nAll:\n{:?}\n", &text[..len], text);

    for (token, msg) in lexed.errors() {
        let pos = lexed.text_start(token);
        errors.push(format!("error {pos}: {msg}\n"));
    }

    let has_errors = !errors.is_empty();
    for e in errors {
        buf.push_str(&e);
    }
    (buf, has_errors)
}

#[track_caller]
fn lex_and_expect_no_errors(path: &str) {
    let path = PathBuf::from(path);
    let text = fs::read(&path).unwrap().into_boxed_slice();
    let actual = lex(&text);
    let mut p = PathBuf::from("..");
    p.push(path);
    p.set_extension("rast");
    expect_file![p].assert_eq(&actual)
}

#[track_caller]
fn parse_and_expect_no_errors(path: &str) {
    parse_and_expect_no_errors_with_edition(path, Edition::CURRENT)
}

#[track_caller]
fn parse_and_expect_no_errors_with_edition(path: &str, edition: Edition) {
    let path = PathBuf::from(path);
    let text = fs::read(&path).unwrap().into_boxed_slice();
    let (actual, errors) = parse(TopEntryPoint::PdfDocument, &text, edition);
    assert!(!errors, "errors in an OK file {}:\n{actual}", path.display());
    let mut p = PathBuf::from("..");
    p.push(path);
    p.set_extension("rast");
    expect_file![p].assert_eq(&actual)
}
