use std::{
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};

use super::*;
use expect_test::expect_file;

/// ISO `32000-1:2008`, Section 7.2.3 Character Set.
#[test]
fn test_character_set() {
    assert_eq!(test_input(b""), vec![]);

    // end of line marker
    assert_eq!(
        tokenize(b"\n\r\n\r").collect::<Vec<Token>>(),
        vec![Token::new(TokenKind::Eol, 1), Token::new(TokenKind::Eol, 2), Token::new(TokenKind::Eol, 1)]
    );

    // whitespaces
    assert_eq!(
        tokenize(b" \t\x0C\0\r\n").collect::<Vec<Token>>(),
        vec![Token::new(TokenKind::Whitespace, 4), Token::new(TokenKind::Eol, 2)]
    );
}

/// ISO `32000-1:2008`, Section 7.3.3 Numeric Objects.
#[test]
fn test_numbers() {
    assert_eq!(test_input(b"123"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 3)]);
    assert_eq!(test_input(b"43445"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 5)]);
    assert_eq!(test_input(b"+17"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 3)]);
    assert_eq!(test_input(b"-98"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 3)]);
    assert_eq!(test_input(b"0"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1)]);
    assert_eq!(test_input(b"00987"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 5)]);
    assert_eq!(test_input(b"34.5"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 4)]);
    assert_eq!(
        test_input(b"-3.62"),
        [Token {
            kind: TokenKind::Literal { kind: LiteralKind::Real },
            len: 5
        }]
    );
    assert_eq!(test_input(b"+123.6"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 6)]);
    assert_eq!(test_input(b"4."), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 2)]);
    assert_eq!(test_input(b"-.002"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 5)]);
    assert_eq!(test_input(b"009.87"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 6)]);
    assert_eq!(test_input(b".0"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 2)]);

    // invalid numbers
    assert_eq!(test_input(b"."), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 1)]);
    assert_eq!(test_input(b"+"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1)]);
    assert_eq!(test_input(b"-"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1)]);
    assert_eq!(
        test_input(b"+-"),
        vec![
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1),
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1)
        ]
    );
    assert_eq!(
        test_input(b"-+"),
        vec![
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1),
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1)
        ]
    );
    assert_eq!(test_input(b"-."), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 2)]);
    assert_eq!(
        test_input(b".."),
        vec![
            Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 1),
            Token::new(TokenKind::Literal { kind: LiteralKind::Real }, 1)
        ]
    );
}

/// ISO `32000-1:2008`, Section 7.3.5 Name Objects.
#[test]
fn test_names() {
    assert_eq!(test_input(b"/Name1"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 6)]);
    assert_eq!(
        test_input(b"/ASomewhatLongerName"),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 20)]
    );
    assert_eq!(
        test_input(b"/A;Name_With-Various***Characters?"),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 34)]
    );
    assert_eq!(test_input(b"/1.2"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 4)]);
    assert_eq!(test_input(b"/$$"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 3)]);
    assert_eq!(test_input(b"/@pattern"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 9)]);
    assert_eq!(test_input(b"/.notdef"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 8)]);
    assert_eq!(
        test_input(b"/Lime#20Green"),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 13)]
    );
    assert_eq!(
        test_input(b"/paired#28#29parentheses"),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 24)]
    );
    assert_eq!(
        test_input(b"/The_Key_of_F#23_Minor"),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 22)]
    );

    // invalid names
    assert_eq!(test_input(b"/"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1)]);
    assert_eq!(
        test_input(b"/("),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1), Token::new(TokenKind::Unknown, 1)]
    );
    assert_eq!(
        test_input(b"/)"),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1), Token::new(TokenKind::Unknown, 1)]
    );
    assert_eq!(
        test_input(b"/<"),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1), Token::new(TokenKind::Unknown, 1)]
    );
    assert_eq!(
        test_input(b"/>"),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1), Token::new(TokenKind::Unknown, 1)]
    );
    assert_eq!(
        test_input(b"/["),
        vec![
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1),
            Token::new(TokenKind::OpenBracket, 1)
        ]
    );
    assert_eq!(
        test_input(b"/]"),
        vec![
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1),
            Token::new(TokenKind::CloseBracket, 1)
        ]
    );
    assert_eq!(
        test_input(b"/{"),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1), Token::new(TokenKind::Unknown, 1)]
    );
    assert_eq!(
        test_input(b"/}"),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1), Token::new(TokenKind::Unknown, 1)]
    );
    assert_eq!(
        test_input(b"/%"),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 1), Token::new(TokenKind::Comment, 1)]
    );
    assert_eq!(
        test_input(b"/Name1("),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 6), Token::new(TokenKind::Unknown, 1)]
    );
}

/// ISO `32000-1:2008`, Section 7.3.2 Boolean Objects.
#[test]
fn test_boolean() {
    assert_eq!(test_input(b"true"), vec![Token::new(TokenKind::Ident, 4)]);
    assert_eq!(test_input(b"false"), vec![Token::new(TokenKind::Ident, 5)]);
}

/// ISO `32000-1:2008`, Section 7.3.9 Null Object.
#[test]
fn test_null() {
    assert_eq!(test_input(b"null"), vec![Token::new(TokenKind::Ident, 4)]);
}

/// ISO `32000-1:2008`, Section 7.3.4.2 Literal Strings.
#[test]
fn test_literal_string() {
    assert_eq!(
        test_input(b"(This is a string)"),
        vec![Token::new(
            TokenKind::Literal {
                kind: LiteralKind::LiteralString
            },
            18
        )]
    );
    assert_eq!(
        test_input(b"(Strings can contain newlines \nand such.)"),
        vec![Token::new(
            TokenKind::Literal {
                kind: LiteralKind::LiteralString
            },
            41
        )]
    );

    assert_eq!(
        test_input(b"(Strings can contain balanced parentheses () \nand special characters ( * ! & } ^ %and so on) .)"),
        vec![Token::new(
            TokenKind::Literal {
                kind: LiteralKind::LiteralString
            },
            95
        )]
    );

    assert_eq!(
        test_input(b"(The following is an empty string .)"),
        vec![Token::new(
            TokenKind::Literal {
                kind: LiteralKind::LiteralString
            },
            36
        )]
    );
    assert_eq!(
        test_input(b"()"),
        vec![Token::new(
            TokenKind::Literal {
                kind: LiteralKind::LiteralString
            },
            2
        )]
    );
    assert_eq!(
        test_input(b"(It has zero (0) length.)"),
        vec![Token::new(
            TokenKind::Literal {
                kind: LiteralKind::LiteralString
            },
            25
        )]
    );

    // invalid literal strings
    assert_eq!(test_input(b"("), vec![Token::new(TokenKind::Unknown, 1)]);
    assert_eq!(test_input(b")"), vec![Token::new(TokenKind::Unknown, 1)]);
    assert_eq!(
        test_input(b"(This string has ( unbalanced parentheses.)"),
        vec![Token::new(TokenKind::Unknown, 43)]
    );
}

// ISO `32000-1:2008`, Section 7.3.4.3 Hexadecimal Strings.
#[test]
fn test_hex_string() {
    assert_eq!(
        test_input(b"<4E6F762073686D6F7A206B6120706F702E>"),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::HexString }, 36)]
    );

    assert_eq!(
        test_input(b"<901FA3>"),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::HexString }, 8)]
    );
    assert_eq!(test_input(b"<901FA>"), vec![Token::new(TokenKind::Literal { kind: LiteralKind::HexString }, 7)]);

    assert_eq!(
        test_input(b"<90\0 1F\r\n A3\t\x0C>"),
        vec![Token::new(TokenKind::Literal { kind: LiteralKind::HexString }, 15)]
    );

    // invalid hex strings
    assert_eq!(test_input(b"<"), vec![Token::new(TokenKind::Unknown, 1)]);
    assert_eq!(test_input(b">"), vec![Token::new(TokenKind::Unknown, 1)]);
    assert_eq!(test_input(b"<0"), vec![Token::new(TokenKind::Unknown, 2)]);
    assert_eq!(test_input(b"<0<"), vec![Token::new(TokenKind::Unknown, 2), Token::new(TokenKind::Unknown, 1)]);
}

// ISO `32000-1:2008`, Section 7.3.6 Array Objects.
#[test]
fn test_array() {
    assert_eq!(
        test_input(b"[ 1 (2) 3 ]"),
        vec![
            Token::new(TokenKind::OpenBracket, 1),
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1),
            Token::new(
                TokenKind::Literal {
                    kind: LiteralKind::LiteralString
                },
                3
            ),
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1),
            Token::new(TokenKind::CloseBracket, 1),
        ]
    );
}

// ISO `32000-1:2008`, Section 7.3.7 Dictionary Objects.
#[test]
fn test_dictionary() {
    assert_eq!(
        test_input(b"<< /Type /Catalog /Pages 3 0 R >>"),
        vec![
            Token::new(TokenKind::OpenDict, 2),
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 5),
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 8),
            Token::new(TokenKind::Literal { kind: LiteralKind::Name }, 6),
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1),
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 1),
            Token::new(TokenKind::Ident, 1),
            Token::new(TokenKind::CloseDict, 2),
        ]
    );
}

// ISO `32000-1:2008`, Section 7.2.4 Comments.
#[test]
fn test_comments() {
    assert_eq!(test_input(b"% This is a comment"), vec![Token::new(TokenKind::Comment, 19)]);
    assert_eq!(
        test_input(b"% This is a comment\n"),
        vec![Token::new(TokenKind::Comment, 19), Token::new(TokenKind::Eol, 1)]
    );
    assert_eq!(
        test_input(b"% This is a comment\r\n"),
        vec![Token::new(TokenKind::Comment, 19), Token::new(TokenKind::Eol, 2)]
    );
    assert_eq!(
        test_input(b"abc%comment (/%) blah blah blah \n123"),
        vec![
            Token::new(TokenKind::Ident, 3),
            Token::new(TokenKind::Comment, 29),
            Token::new(TokenKind::Eol, 1),
            Token::new(TokenKind::Literal { kind: LiteralKind::Int }, 3)
        ]
    );
}

/// ISO `32000-1:2008`, Section 7.3.8 Stream Objects.
#[test]
fn test_stream_objects() {
    // Valid stream with endstream
    assert_eq!(test_input(b"stream\nBT /F1 12 Tf ET\nendstream"), vec![Token::new(TokenKind::Stream, 32)]);

    // Encoded stream
    assert_eq!(
        test_input(b"stream\n\x78\x9c\xcb\x48\xcd\xc9\xc9\x57\x28\xcf\x2f\xca\x49\x01\x00\x18\xab\x04\x1d\nendstream"),
        vec![Token::new(TokenKind::Stream, 36),]
    );

    // Incomplete stream (no endstream)
    assert_eq!(test_input(b"stream\nBT /F1 12 Tf ET\n"), vec![Token::new(TokenKind::Stream, 23),]);

    // Stream with additional data after endstream
    assert_eq!(
        test_input(b"stream\nBT /F1 12 Tf ET\nendstream\nadditional data"),
        vec![
            Token {
                kind: TokenKind::Stream,
                len: 32
            },
            Token { kind: TokenKind::Eol, len: 1 },
            Token {
                kind: TokenKind::Ident,
                len: 10
            },
            Token {
                kind: TokenKind::Ident,
                len: 4
            }
        ]
    );
}

fn test_input(input: &[u8]) -> Vec<Token> {
    tokenize(input).filter(|t| t.kind != TokenKind::Whitespace).collect()
}

#[test]
fn lex_ok() {
    for case in TestCase::list("ok") {
        let _guard = stdx::panic_context::enter(format!("{:?}", case.pdf));
        let actual = lex(&case.text);
        expect_file![case.rast].assert_eq(&actual)
    }
}

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
            .map(|&c| if c.is_ascii() { (c as char).to_string() } else { format!("\\x{:02x}", c) })
            .collect();

        writeln!(res, "{kind:?} {escaped_text:?}").unwrap();

        offset += len; // Update offset
    }
    res
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct TestCase {
    pdf: PathBuf,
    rast: PathBuf,
    text: Box<[u8]>,
}

impl TestCase {
    fn list(path: &'static str) -> Vec<TestCase> {
        let crate_root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let test_data_dir = crate_root_dir.join("test_data");
        let dir = test_data_dir.join(path);

        let mut res = Vec::new();
        let read_dir = fs::read_dir(&dir).unwrap_or_else(|err| panic!("can't `read_dir` {}: {err}", dir.display()));
        for file in read_dir {
            let file = file.unwrap();
            let path = file.path();
            if path.extension().unwrap_or_default() == "pdf" {
                let pdf = path;
                let rast = pdf.with_extension("rast");
                let text = fs::read(&pdf).unwrap().into_boxed_slice();
                res.push(TestCase { pdf, rast, text });
            }
        }
        res.sort();
        res
    }
}
