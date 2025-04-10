//! Generated by `cargo xtask codegen grammar`, do not edit by hand.

use crate::{
    ast::AstToken,
    SyntaxKind::{self, *},
    SyntaxToken,
};
use std::{fmt, hash};
pub struct Comment {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(&self.syntax, f) }
}
impl AstToken for Comment {
    fn can_cast(kind: SyntaxKind) -> bool { kind == COMMENT }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("Comment").field("syntax", &self.syntax).finish() }
}
impl Clone for Comment {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for Comment {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for Comment {}
impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
pub struct HexString {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for HexString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(&self.syntax, f) }
}
impl AstToken for HexString {
    fn can_cast(kind: SyntaxKind) -> bool { kind == HEX_STRING }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for HexString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("HexString").field("syntax", &self.syntax).finish() }
}
impl Clone for HexString {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for HexString {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for HexString {}
impl PartialEq for HexString {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
pub struct IntNumber {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for IntNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(&self.syntax, f) }
}
impl AstToken for IntNumber {
    fn can_cast(kind: SyntaxKind) -> bool { kind == INT_NUMBER }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for IntNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("IntNumber").field("syntax", &self.syntax).finish() }
}
impl Clone for IntNumber {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for IntNumber {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for IntNumber {}
impl PartialEq for IntNumber {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
pub struct LiteralString {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for LiteralString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(&self.syntax, f) }
}
impl AstToken for LiteralString {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LITERAL_STRING }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for LiteralString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("LiteralString").field("syntax", &self.syntax).finish() }
}
impl Clone for LiteralString {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for LiteralString {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for LiteralString {}
impl PartialEq for LiteralString {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
pub struct Name {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(&self.syntax, f) }
}
impl AstToken for Name {
    fn can_cast(kind: SyntaxKind) -> bool { kind == NAME }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("Name").field("syntax", &self.syntax).finish() }
}
impl Clone for Name {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for Name {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for Name {}
impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
pub struct RealNumber {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for RealNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(&self.syntax, f) }
}
impl AstToken for RealNumber {
    fn can_cast(kind: SyntaxKind) -> bool { kind == REAL_NUMBER }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for RealNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("RealNumber").field("syntax", &self.syntax).finish() }
}
impl Clone for RealNumber {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for RealNumber {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for RealNumber {}
impl PartialEq for RealNumber {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
pub struct Whitespace {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Whitespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(&self.syntax, f) }
}
impl AstToken for Whitespace {
    fn can_cast(kind: SyntaxKind) -> bool { kind == WHITESPACE }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
impl fmt::Debug for Whitespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("Whitespace").field("syntax", &self.syntax).finish() }
}
impl Clone for Whitespace {
    fn clone(&self) -> Self { Self { syntax: self.syntax.clone() } }
}
impl hash::Hash for Whitespace {
    fn hash<H: hash::Hasher>(&self, state: &mut H) { self.syntax.hash(state); }
}
impl Eq for Whitespace {}
impl PartialEq for Whitespace {
    fn eq(&self, other: &Self) -> bool { self.syntax == other.syntax }
}
