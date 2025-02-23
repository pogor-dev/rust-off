//! Generated by `cargo xtask codegen grammar`, do not edit by hand.

#![allow(bad_style, missing_docs, unreachable_pub)]
use crate::Edition;
#[doc = r" The kind of syntax node, e.g. `IDENT`, `USE_KW`, or `STRUCT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    SLASH,
    L_PAREN,
    R_PAREN,
    L_BRACK,
    R_BRACK,
    L_ANGLE,
    R_ANGLE,
    L_DICT,
    R_DICT,
    DOT,
    R_KW,
    ENDOBJ_KW,
    ENDSTREAM_KW,
    F_KW,
    FALSE_KW,
    N_KW,
    NULL_KW,
    OBJ_KW,
    STARTXREF_KW,
    STREAM_KW,
    TRAILER_KW,
    TRUE_KW,
    XREF_KW,
    HEX_STRING,
    IDENT,
    INT_NUMBER,
    LITERAL_STRING,
    RAW_BYTES,
    REAL_NUMBER,
    COMMENT,
    EOF_MARKER,
    ERROR,
    NEWLINE,
    PDF_MARKER,
    WHITESPACE,
    ARRAY,
    BODY,
    BOOLEAN,
    CROSS_REF_ENTRY,
    CROSS_REF_SECTION,
    CROSS_REF_TABLE,
    DICTIONARY,
    DICT_ENTRY,
    DIRECT_OBJECT,
    HEADER,
    HEX_STRING,
    INDIRECT_REFERENCE,
    INTEGER,
    KEY,
    LITERAL_STRING,
    NAME,
    NULL,
    OBJECT,
    OBJECT_DATA,
    OBJECT_I_D,
    PDF_DOCUMENT,
    PDF_VERSION,
    REAL,
    STREAM,
    TRAILER,
    VALUE,
    #[doc(hidden)]
    __LAST,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    #[allow(unreachable_patterns)]
    pub const fn text(self) -> &'static str {
        match self {
            TOMBSTONE | EOF | __LAST | HEX_STRING | IDENT | INT_NUMBER | LITERAL_STRING | RAW_BYTES | REAL_NUMBER | ARRAY | BODY | BOOLEAN
            | CROSS_REF_ENTRY | CROSS_REF_SECTION | CROSS_REF_TABLE | DICTIONARY | DICT_ENTRY | DIRECT_OBJECT | HEADER | HEX_STRING | INDIRECT_REFERENCE
            | INTEGER | KEY | LITERAL_STRING | NAME | NULL | OBJECT | OBJECT_DATA | OBJECT_I_D | PDF_DOCUMENT | PDF_VERSION | REAL | STREAM | TRAILER
            | VALUE | COMMENT | EOF_MARKER | ERROR | NEWLINE | PDF_MARKER | WHITESPACE => panic!("no text for these `SyntaxKind`s"),
            SLASH => "/",
            L_PAREN => "(",
            R_PAREN => ")",
            L_BRACK => "[",
            R_BRACK => "]",
            L_ANGLE => "<",
            R_ANGLE => ">",
            L_DICT => "<<",
            R_DICT => ">>",
            DOT => ".",
            R_KW => "R",
            ENDOBJ_KW => "endobj",
            ENDSTREAM_KW => "endstream",
            F_KW => "f",
            FALSE_KW => "false",
            N_KW => "n",
            NULL_KW => "null",
            OBJ_KW => "obj",
            STARTXREF_KW => "startxref",
            STREAM_KW => "stream",
            TRAILER_KW => "trailer",
            TRUE_KW => "true",
            XREF_KW => "xref",
        }
    }
    #[doc = r" Checks whether this syntax kind is a strict keyword for the given edition."]
    #[doc = r" Strict keywords are identifiers that are always considered keywords."]
    pub fn is_strict_keyword(self, edition: Edition) -> bool {
        matches!(
            self,
            R_KW | ENDOBJ_KW | ENDSTREAM_KW | F_KW | FALSE_KW | N_KW | NULL_KW | OBJ_KW | STARTXREF_KW | STREAM_KW | TRAILER_KW | TRUE_KW | XREF_KW
        ) || match self {
            _ => false,
        }
    }
    #[doc = r" Checks whether this syntax kind is a weak keyword for the given edition."]
    #[doc = r" Weak keywords are identifiers that are considered keywords only in certain contexts."]
    pub fn is_contextual_keyword(self, edition: Edition) -> bool {
        match self {
            _ => false,
        }
    }
    #[doc = r" Checks whether this syntax kind is a strict or weak keyword for the given edition."]
    pub fn is_keyword(self, edition: Edition) -> bool {
        matches!(
            self,
            R_KW | ENDOBJ_KW | ENDSTREAM_KW | F_KW | FALSE_KW | N_KW | NULL_KW | OBJ_KW | STARTXREF_KW | STREAM_KW | TRAILER_KW | TRUE_KW | XREF_KW
        ) || match self {
            _ => false,
        }
    }
    pub fn is_punct(self) -> bool { matches!(self, SLASH | L_PAREN | R_PAREN | L_BRACK | R_BRACK | L_ANGLE | R_ANGLE | L_DICT | R_DICT | DOT) }
    pub fn is_literal(self) -> bool { matches!(self, HEX_STRING | IDENT | INT_NUMBER | LITERAL_STRING | RAW_BYTES | REAL_NUMBER) }
    pub fn from_keyword(ident: &str, edition: Edition) -> Option<SyntaxKind> {
        let kw = match ident {
            "R" => R_KW,
            "endobj" => ENDOBJ_KW,
            "endstream" => ENDSTREAM_KW,
            "f" => F_KW,
            "false" => FALSE_KW,
            "n" => N_KW,
            "null" => NULL_KW,
            "obj" => OBJ_KW,
            "startxref" => STARTXREF_KW,
            "stream" => STREAM_KW,
            "trailer" => TRAILER_KW,
            "true" => TRUE_KW,
            "xref" => XREF_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_contextual_keyword(ident: &str, edition: Edition) -> Option<SyntaxKind> {
        let kw = match ident {
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            '/' => SLASH,
            '(' => L_PAREN,
            ')' => R_PAREN,
            '[' => L_BRACK,
            ']' => R_BRACK,
            '<' => L_ANGLE,
            '>' => R_ANGLE,
            '.' => DOT,
            _ => return None,
        };
        Some(tok)
    }
}
#[macro_export]
macro_rules ! T { [/] => { $ crate :: SyntaxKind :: SLASH } ; ['('] => { $ crate :: SyntaxKind :: L_PAREN } ; [')'] => { $ crate :: SyntaxKind :: R_PAREN } ; ['['] => { $ crate :: SyntaxKind :: L_BRACK } ; [']'] => { $ crate :: SyntaxKind :: R_BRACK } ; [<] => { $ crate :: SyntaxKind :: L_ANGLE } ; [>] => { $ crate :: SyntaxKind :: R_ANGLE } ; [<<] => { $ crate :: SyntaxKind :: L_DICT } ; [>>] => { $ crate :: SyntaxKind :: R_DICT } ; [.] => { $ crate :: SyntaxKind :: DOT } ; [R] => { $ crate :: SyntaxKind :: R_KW } ; [endobj] => { $ crate :: SyntaxKind :: ENDOBJ_KW } ; [endstream] => { $ crate :: SyntaxKind :: ENDSTREAM_KW } ; [f] => { $ crate :: SyntaxKind :: F_KW } ; [false] => { $ crate :: SyntaxKind :: FALSE_KW } ; [n] => { $ crate :: SyntaxKind :: N_KW } ; [null] => { $ crate :: SyntaxKind :: NULL_KW } ; [obj] => { $ crate :: SyntaxKind :: OBJ_KW } ; [startxref] => { $ crate :: SyntaxKind :: STARTXREF_KW } ; [stream] => { $ crate :: SyntaxKind :: STREAM_KW } ; [trailer] => { $ crate :: SyntaxKind :: TRAILER_KW } ; [true] => { $ crate :: SyntaxKind :: TRUE_KW } ; [xref] => { $ crate :: SyntaxKind :: XREF_KW } ; [lifetime_ident] => { $ crate :: SyntaxKind :: LIFETIME_IDENT } ; [int_number] => { $ crate :: SyntaxKind :: INT_NUMBER } ; [ident] => { $ crate :: SyntaxKind :: IDENT } ; [string] => { $ crate :: SyntaxKind :: STRING } ; [shebang] => { $ crate :: SyntaxKind :: SHEBANG } ; }
