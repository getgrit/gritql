/// Represents an AST node and offers convenient AST-specific functionality.
///
/// This trait should be free from dependencies on TreeSitter.
pub trait AstNode: Sized {
    /// Returns an iterator over the node's ancestors, starting with the node
    /// itself and moving up to the root.
    fn ancestors(&self) -> impl Iterator<Item = Self>;

    /// Returns an iterator over the node's children.
    fn children(&self) -> impl Iterator<Item = Self>;

    /// Returns the next node, ignoring trivia such as whitespace.
    fn next_named_sibling(&self) -> Option<Self>;

    /// Returns the previous node, ignoring trivia such as whitespace.
    fn previous_named_sibling(&self) -> Option<Self>;

    /// Returns the text representation of the node.
    fn text(&self) -> &str;
}
