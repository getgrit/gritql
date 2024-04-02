use crate::position::Range;
use grit_util::AstNode;
use tree_sitter::Node;

/// A TreeSitter node, including a reference to the source code from which it
/// was parsed.
pub struct NodeWithSource<'a> {
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

    fn text(&self) -> &str {
        let range = Range::from(self.node.range());
        &self.source[range.start_byte as usize..range.end_byte as usize]
    }
}
