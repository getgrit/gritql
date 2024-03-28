/// Represents an AST node and offers convenient AST-specific functionality.
///
/// This trait should be free from dependencies on TreeSitter.
pub trait AstNode: Sized {
    /// Returns the next node, ignoring trivia such as whitespace.
    fn next_non_trivia_node(&self) -> Option<Self>;

    /// Returns the previous node, ignoring trivia such as whitespace.
    fn previous_non_trivia_node(&self) -> Option<Self>;

    /// Returns the text representation of the node.
    fn text(&self) -> &str;
}
