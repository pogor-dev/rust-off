//! Generated by `cargo xtask codegen grammar`, do not edit by hand.

#![allow(non_snake_case)]
use crate::{
    ast::{self, support, AstChildren, AstNode},
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken, T,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ArrayExpr {
    #[inline]
    pub fn exprs(&self) -> AstChildren<Expr> { support::children(&self.syntax) }
    #[inline]
    pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
    #[inline]
    pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Body {
    pub(crate) syntax: SyntaxNode,
}
impl Body {
    #[inline]
    pub fn indirect_object_exprs(&self) -> AstChildren<IndirectObjectExpr> { support::children(&self.syntax) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DictionaryExpr {
    pub(crate) syntax: SyntaxNode,
}
impl DictionaryExpr {
    #[inline]
    pub fn dictionary_item_exprs(&self) -> AstChildren<DictionaryItemExpr> { support::children(&self.syntax) }
    #[inline]
    pub fn l_dict_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![<<]) }
    #[inline]
    pub fn r_dict_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![>>]) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DictionaryItemExpr {
    pub(crate) syntax: SyntaxNode,
}
impl DictionaryItemExpr {
    #[inline]
    pub fn dictionary_item_key_expr(&self) -> Option<DictionaryItemKeyExpr> { support::child(&self.syntax) }
    #[inline]
    pub fn dictionary_item_value_expr(&self) -> Option<DictionaryItemValueExpr> { support::child(&self.syntax) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DictionaryItemKeyExpr {
    pub(crate) syntax: SyntaxNode,
}
impl DictionaryItemKeyExpr {
    #[inline]
    pub fn literal(&self) -> Option<Literal> { support::child(&self.syntax) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DictionaryItemValueExpr {
    pub(crate) syntax: SyntaxNode,
}
impl DictionaryItemValueExpr {
    #[inline]
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndirectObjectExpr {
    pub(crate) syntax: SyntaxNode,
}
impl IndirectObjectExpr {
    #[inline]
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
    #[inline]
    pub fn indirect_object_id(&self) -> Option<IndirectObjectId> { support::child(&self.syntax) }
    #[inline]
    pub fn stream_expr(&self) -> Option<StreamExpr> { support::child(&self.syntax) }
    #[inline]
    pub fn endobj_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![endobj]) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndirectObjectId {
    pub(crate) syntax: SyntaxNode,
}
impl IndirectObjectId {
    #[inline]
    pub fn generation_number(&self) -> Option<Literal> { support::child(&self.syntax) }
    #[inline]
    pub fn object_number(&self) -> Option<Literal> { support::child(&self.syntax) }
    #[inline]
    pub fn obj_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![obj]) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndirectReferenceExpr {
    pub(crate) syntax: SyntaxNode,
}
impl IndirectReferenceExpr {
    #[inline]
    pub fn generation_number(&self) -> Option<Literal> { support::child(&self.syntax) }
    #[inline]
    pub fn object_number(&self) -> Option<Literal> { support::child(&self.syntax) }
    #[inline]
    pub fn R_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![R]) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub(crate) syntax: SyntaxNode,
}
impl Literal {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PdfDocument {
    pub(crate) syntax: SyntaxNode,
}
impl PdfDocument {
    #[inline]
    pub fn body(&self) -> Option<Body> { support::child(&self.syntax) }
    #[inline]
    pub fn trailer(&self) -> Option<Trailer> { support::child(&self.syntax) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StreamExpr {
    pub(crate) syntax: SyntaxNode,
}
impl StreamExpr {
    #[inline]
    pub fn dictionary_expr(&self) -> Option<DictionaryExpr> { support::child(&self.syntax) }
    #[inline]
    pub fn endstream_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![endstream]) }
    #[inline]
    pub fn stream_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![stream]) }
    #[inline]
    pub fn stream_data_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![stream_data]) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Trailer {
    pub(crate) syntax: SyntaxNode,
}
impl Trailer {
    #[inline]
    pub fn dictionary_expr(&self) -> Option<DictionaryExpr> { support::child(&self.syntax) }
    #[inline]
    pub fn literal(&self) -> Option<Literal> { support::child(&self.syntax) }
    #[inline]
    pub fn startxref_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![startxref]) }
    #[inline]
    pub fn trailer_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![trailer]) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XRefEntry {
    pub(crate) syntax: SyntaxNode,
}
impl XRefEntry {
    #[inline]
    pub fn free_or_used(&self) -> Option<XRefEntryType> { support::child(&self.syntax) }
    #[inline]
    pub fn generation_number(&self) -> Option<Literal> { support::child(&self.syntax) }
    #[inline]
    pub fn offset(&self) -> Option<Literal> { support::child(&self.syntax) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XRefEntryType {
    pub(crate) syntax: SyntaxNode,
}
impl XRefEntryType {
    #[inline]
    pub fn f_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![f]) }
    #[inline]
    pub fn n_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![n]) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XRefSection {
    pub(crate) syntax: SyntaxNode,
}
impl XRefSection {
    #[inline]
    pub fn x_ref_subsections(&self) -> AstChildren<XRefSubsection> { support::children(&self.syntax) }
    #[inline]
    pub fn xref_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![xref]) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XRefSubsection {
    pub(crate) syntax: SyntaxNode,
}
impl XRefSubsection {
    #[inline]
    pub fn count(&self) -> Option<Literal> { support::child(&self.syntax) }
    #[inline]
    pub fn first_object(&self) -> Option<Literal> { support::child(&self.syntax) }
    #[inline]
    pub fn x_ref_entrys(&self) -> AstChildren<XRefEntry> { support::children(&self.syntax) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XRefTable {
    pub(crate) syntax: SyntaxNode,
}
impl XRefTable {
    #[inline]
    pub fn x_ref_sections(&self) -> AstChildren<XRefSection> { support::children(&self.syntax) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    ArrayExpr(ArrayExpr),
    DictionaryExpr(DictionaryExpr),
    IndirectObjectExpr(IndirectObjectExpr),
    IndirectReferenceExpr(IndirectReferenceExpr),
    Literal(Literal),
}
impl AstNode for ArrayExpr {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        ARRAY_EXPR
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == ARRAY_EXPR }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Body {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        BODY
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == BODY }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for DictionaryExpr {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        DICTIONARY_EXPR
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == DICTIONARY_EXPR }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for DictionaryItemExpr {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        DICTIONARY_ITEM_EXPR
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == DICTIONARY_ITEM_EXPR }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for DictionaryItemKeyExpr {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        DICTIONARY_ITEM_KEY_EXPR
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == DICTIONARY_ITEM_KEY_EXPR }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for DictionaryItemValueExpr {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        DICTIONARY_ITEM_VALUE_EXPR
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == DICTIONARY_ITEM_VALUE_EXPR }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for IndirectObjectExpr {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        INDIRECT_OBJECT_EXPR
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == INDIRECT_OBJECT_EXPR }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for IndirectObjectId {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        INDIRECT_OBJECT_ID
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == INDIRECT_OBJECT_ID }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for IndirectReferenceExpr {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        INDIRECT_REFERENCE_EXPR
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == INDIRECT_REFERENCE_EXPR }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Literal {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        LITERAL
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == LITERAL }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PdfDocument {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        PDF_DOCUMENT
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == PDF_DOCUMENT }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for StreamExpr {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        STREAM_EXPR
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == STREAM_EXPR }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Trailer {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        TRAILER
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == TRAILER }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for XRefEntry {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        X_REF_ENTRY
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == X_REF_ENTRY }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for XRefEntryType {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        X_REF_ENTRY_TYPE
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == X_REF_ENTRY_TYPE }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for XRefSection {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        X_REF_SECTION
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == X_REF_SECTION }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for XRefSubsection {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        X_REF_SUBSECTION
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == X_REF_SUBSECTION }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for XRefTable {
    #[inline]
    fn kind() -> SyntaxKind
    where
        Self: Sized,
    {
        X_REF_TABLE
    }
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { kind == X_REF_TABLE }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl From<ArrayExpr> for Expr {
    #[inline]
    fn from(node: ArrayExpr) -> Expr { Expr::ArrayExpr(node) }
}
impl From<DictionaryExpr> for Expr {
    #[inline]
    fn from(node: DictionaryExpr) -> Expr { Expr::DictionaryExpr(node) }
}
impl From<IndirectObjectExpr> for Expr {
    #[inline]
    fn from(node: IndirectObjectExpr) -> Expr { Expr::IndirectObjectExpr(node) }
}
impl From<IndirectReferenceExpr> for Expr {
    #[inline]
    fn from(node: IndirectReferenceExpr) -> Expr { Expr::IndirectReferenceExpr(node) }
}
impl From<Literal> for Expr {
    #[inline]
    fn from(node: Literal) -> Expr { Expr::Literal(node) }
}
impl AstNode for Expr {
    #[inline]
    fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, ARRAY_EXPR | DICTIONARY_EXPR | INDIRECT_OBJECT_EXPR | INDIRECT_REFERENCE_EXPR | LITERAL) }
    #[inline]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            ARRAY_EXPR => Expr::ArrayExpr(ArrayExpr { syntax }),
            DICTIONARY_EXPR => Expr::DictionaryExpr(DictionaryExpr { syntax }),
            INDIRECT_OBJECT_EXPR => Expr::IndirectObjectExpr(IndirectObjectExpr { syntax }),
            INDIRECT_REFERENCE_EXPR => Expr::IndirectReferenceExpr(IndirectReferenceExpr { syntax }),
            LITERAL => Expr::Literal(Literal { syntax }),
            _ => return None,
        };
        Some(res)
    }
    #[inline]
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::ArrayExpr(it) => &it.syntax,
            Expr::DictionaryExpr(it) => &it.syntax,
            Expr::IndirectObjectExpr(it) => &it.syntax,
            Expr::IndirectReferenceExpr(it) => &it.syntax,
            Expr::Literal(it) => &it.syntax,
        }
    }
}
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for ArrayExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for DictionaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for DictionaryItemExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for DictionaryItemKeyExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for DictionaryItemValueExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for IndirectObjectExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for IndirectObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for IndirectReferenceExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for PdfDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for StreamExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for Trailer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for XRefEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for XRefEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for XRefSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for XRefSubsection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
impl std::fmt::Display for XRefTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { std::fmt::Display::fmt(self.syntax(), f) }
}
