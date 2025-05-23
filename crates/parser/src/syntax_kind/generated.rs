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
    L_BRACK,
    R_BRACK,
    L_DICT,
    R_DICT,
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
    INT_NUMBER,
    LITERAL_STRING,
    NAME,
    REAL_NUMBER,
    COMMENT,
    ERROR,
    NEWLINE,
    RAW_STREAM,
    STREAM_DATA,
    WHITESPACE,
    ARRAY_EXPR,
    BODY,
    DICTIONARY_EXPR,
    DICTIONARY_ITEM_EXPR,
    DICTIONARY_ITEM_KEY_EXPR,
    DICTIONARY_ITEM_VALUE_EXPR,
    EXPR,
    INDIRECT_OBJECT_EXPR,
    INDIRECT_OBJECT_ID,
    INDIRECT_REFERENCE_EXPR,
    LITERAL,
    PDF_DOCUMENT,
    STREAM_EXPR,
    TRAILER,
    X_REF_ENTRY,
    X_REF_ENTRY_TYPE,
    X_REF_SECTION,
    X_REF_SUBSECTION,
    X_REF_TABLE,
    #[doc(hidden)]
    __LAST,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    #[allow(unreachable_patterns)]
    pub const fn text(self) -> &'static str {
        match self {
            TOMBSTONE
            | EOF
            | __LAST
            | HEX_STRING
            | INT_NUMBER
            | LITERAL_STRING
            | NAME
            | REAL_NUMBER
            | ARRAY_EXPR
            | BODY
            | DICTIONARY_EXPR
            | DICTIONARY_ITEM_EXPR
            | DICTIONARY_ITEM_KEY_EXPR
            | DICTIONARY_ITEM_VALUE_EXPR
            | EXPR
            | INDIRECT_OBJECT_EXPR
            | INDIRECT_OBJECT_ID
            | INDIRECT_REFERENCE_EXPR
            | LITERAL
            | PDF_DOCUMENT
            | STREAM_EXPR
            | TRAILER
            | X_REF_ENTRY
            | X_REF_ENTRY_TYPE
            | X_REF_SECTION
            | X_REF_SUBSECTION
            | X_REF_TABLE
            | COMMENT
            | ERROR
            | NEWLINE
            | RAW_STREAM
            | STREAM_DATA
            | WHITESPACE => panic!("no text for these `SyntaxKind`s"),
            L_BRACK => "[",
            R_BRACK => "]",
            L_DICT => "<<",
            R_DICT => ">>",
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
    #[allow(unused_variables)]
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
    #[allow(unused_variables)]
    pub fn is_contextual_keyword(self, edition: Edition) -> bool {
        match self {
            _ => false,
        }
    }
    #[doc = r" Checks whether this syntax kind is a strict or weak keyword for the given edition."]
    #[allow(unused_variables)]
    pub fn is_keyword(self, edition: Edition) -> bool {
        matches!(
            self,
            R_KW | ENDOBJ_KW | ENDSTREAM_KW | F_KW | FALSE_KW | N_KW | NULL_KW | OBJ_KW | STARTXREF_KW | STREAM_KW | TRAILER_KW | TRUE_KW | XREF_KW
        ) || match self {
            _ => false,
        }
    }
    pub fn is_punct(self) -> bool { matches!(self, L_BRACK | R_BRACK | L_DICT | R_DICT) }
    pub fn is_literal(self) -> bool { matches!(self, HEX_STRING | INT_NUMBER | LITERAL_STRING | NAME | REAL_NUMBER) }
    #[allow(unused_variables)]
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
    #[allow(unused_variables)]
    pub fn from_contextual_keyword(ident: &str, edition: Edition) -> Option<SyntaxKind> {
        #[allow(unused_variables)]
        let kw = match ident {
            _ => return None,
        };
        #[allow(unreachable_code)]
        Some(kw)
    }
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            '[' => L_BRACK,
            ']' => R_BRACK,
            _ => return None,
        };
        Some(tok)
    }
}
#[macro_export]
macro_rules ! T { ['['] => { $ crate :: SyntaxKind :: L_BRACK } ; [']'] => { $ crate :: SyntaxKind :: R_BRACK } ; [<<] => { $ crate :: SyntaxKind :: L_DICT } ; [>>] => { $ crate :: SyntaxKind :: R_DICT } ; [R] => { $ crate :: SyntaxKind :: R_KW } ; [endobj] => { $ crate :: SyntaxKind :: ENDOBJ_KW } ; [endstream] => { $ crate :: SyntaxKind :: ENDSTREAM_KW } ; [f] => { $ crate :: SyntaxKind :: F_KW } ; [false] => { $ crate :: SyntaxKind :: FALSE_KW } ; [n] => { $ crate :: SyntaxKind :: N_KW } ; [null] => { $ crate :: SyntaxKind :: NULL_KW } ; [obj] => { $ crate :: SyntaxKind :: OBJ_KW } ; [startxref] => { $ crate :: SyntaxKind :: STARTXREF_KW } ; [stream] => { $ crate :: SyntaxKind :: STREAM_KW } ; [trailer] => { $ crate :: SyntaxKind :: TRAILER_KW } ; [true] => { $ crate :: SyntaxKind :: TRUE_KW } ; [xref] => { $ crate :: SyntaxKind :: XREF_KW } ; [stream_data] => { $ crate :: SyntaxKind :: STREAM_DATA } ; }
