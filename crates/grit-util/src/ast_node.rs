use crate::{AstCursor, CodeRange, Range};
use std::{borrow::Cow, str::Utf8Error};

/// Represents an AST node and offers convenient AST-specific functionality.
///
/// This trait should be free from dependencies on TreeSitter.
pub trait AstNode: std::fmt::Debug + Sized {
    // returns the id of the node kind
    fn kind_id(&self) -> u16;

    // returns the node kind
    fn kind(&self) -> Cow<str>;

    /// Returns an iterator over the node's ancestors, starting with the node
    /// itself and moving up to the root.
    fn ancestors(&self) -> impl Iterator<Item = Self>;

    /// Returns an iterator over the node's children.
    fn children(&self) -> impl Iterator<Item = Self>;

    /// Returns the node's parent.
    ///
    /// Returns `None` if this is the root node.
    fn parent(&self) -> Option<Self>;

    /// Returns the next node in the tree, ignoring trivia such as whitespace.
    fn next_named_node(&self) -> Option<Self>;

    /// Returns the previous node in the tree, ignoring trivia such as
    /// whitespace.
    fn previous_named_node(&self) -> Option<Self>;

    /// Returns the next adjacent node.
    fn next_sibling(&self) -> Option<Self>;

    /// Returns the previous adjacent node.
    fn previous_sibling(&self) -> Option<Self>;

    /// Returns the text representation of the node.
    fn text(&self) -> Result<Cow<str>, Utf8Error>;

    /// Returns the range of the node.
    fn range(&self) -> Range;

    /// Returns the code range of the node.
    fn code_range(&self) -> CodeRange;

    /// Returns the full source code of the parse tree to which the node
    /// belongs.
    fn full_source(&self) -> &str;

    /// Returns a cursor for traversing the tree, starting at the current node.
    fn walk(&self) -> impl AstCursor<Node = Self>;
}
