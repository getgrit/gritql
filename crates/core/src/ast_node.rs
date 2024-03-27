use tree_sitter::Node;

/// Represents an AST node and offers convenient AST-specific functionality.
///
/// This trait should be free from dependencies on TreeSitter.
pub trait AstNode: Sized {
    /// Returns the next node, ignoring trivia such as whitespace.
    fn next_non_trivia_node(&self) -> Option<Self>;

    /// Returns the previous node, ignoring trivia such as whitespace.
    fn previous_non_trivia_node(&self) -> Option<Self>;
}

/// A TreeSitter node, including a reference to the source code from which it
/// was parsed.
pub(crate) struct NodeWithSource<'a> {
    pub node: Node<'a>,
    pub source: &'a str,
}

impl<'a> NodeWithSource<'a> {
    pub fn new(node: Node<'a>, source: &'a str) -> Self {
        Self { node, source }
    }
}

impl<'a> AstNode for NodeWithSource<'a> {
    fn next_non_trivia_node(&self) -> Option<Self> {
        let mut current_node = self.node.clone();
        loop {
            if let Some(sibling) = current_node.next_named_sibling() {
                return Some(Self::new(sibling, self.source));
            }
            current_node = current_node.parent()?;
        }
    }

    fn previous_non_trivia_node(&self) -> Option<Self> {
        let mut current_node = self.node.clone();
        loop {
            if let Some(sibling) = current_node.prev_named_sibling() {
                return Some(Self::new(sibling, self.source));
            }
            current_node = current_node.parent()?;
        }
    }
}
