use std::borrow::Cow;

use crate::{
    GreenNodeData, NodeOrToken, TextSize,
    green::{GreenNode, GreenToken, SyntaxKind},
};

use super::GreenTokenData;

pub(super) type GreenElement = NodeOrToken<GreenNode, GreenToken>;
pub(crate) type GreenElementRef<'a> = NodeOrToken<&'a GreenNodeData, &'a GreenTokenData>;

impl From<GreenNode> for GreenElement {
    #[inline]
    fn from(node: GreenNode) -> GreenElement {
        NodeOrToken::Node(node)
    }
}

impl<'a> From<&'a GreenNode> for GreenElementRef<'a> {
    #[inline]
    fn from(node: &'a GreenNode) -> GreenElementRef<'a> {
        NodeOrToken::Node(node)
    }
}

impl From<GreenToken> for GreenElement {
    #[inline]
    fn from(token: GreenToken) -> GreenElement {
        NodeOrToken::Token(token)
    }
}

impl From<Cow<'_, GreenNodeData>> for GreenElement {
    #[inline]
    fn from(cow: Cow<'_, GreenNodeData>) -> Self {
        NodeOrToken::Node(cow.into_owned())
    }
}

impl<'a> From<&'a GreenToken> for GreenElementRef<'a> {
    #[inline]
    fn from(token: &'a GreenToken) -> GreenElementRef<'a> {
        NodeOrToken::Token(token)
    }
}

impl GreenElementRef<'_> {
    pub fn to_owned(self) -> GreenElement {
        match self {
            NodeOrToken::Node(it) => NodeOrToken::Node(it.to_owned()),
            NodeOrToken::Token(it) => NodeOrToken::Token(it.to_owned()),
        }
    }
}

impl GreenElement {
    /// Returns kind of this element.
    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        self.as_deref().kind()
    }

    /// Returns the length of the text covered by this element.
    #[inline]
    pub fn text_len(&self) -> TextSize {
        self.as_deref().text_len()
    }
}

impl GreenElementRef<'_> {
    /// Returns kind of this element.
    #[inline]
    pub fn kind(&self) -> SyntaxKind {
        match self {
            NodeOrToken::Node(it) => it.kind(),
            NodeOrToken::Token(it) => it.kind(),
        }
    }

    /// Returns the length of the text covered by this element.
    #[inline]
    pub fn text_len(self) -> TextSize {
        match self {
            NodeOrToken::Node(it) => it.text_len(),
            NodeOrToken::Token(it) => it.text_len(),
        }
    }
}
