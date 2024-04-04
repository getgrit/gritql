use super::cursor_wrapper::CursorWrapper;
use grit_util::{AstCursor, AstNode};
use tree_sitter::Node;

/// A TreeSitter node, including a reference to the source code from which it
/// was parsed.
#[derive(Clone)]
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
    fn ancestors(&self) -> impl Iterator<Item = Self> {
        AncestorIterator::new(self)
    }

    fn children(&self) -> impl Iterator<Item = Self> {
        ChildrenIterator::new(self)
    }

    fn next_named_sibling(&self) -> Option<Self> {
        let mut current_node = self.node.clone();
        loop {
            if let Some(sibling) = current_node.next_named_sibling() {
                return Some(Self::new(sibling, self.source));
            }
            current_node = current_node.parent()?;
        }
    }

    fn previous_named_sibling(&self) -> Option<Self> {
        let mut current_node = self.node.clone();
        loop {
            if let Some(sibling) = current_node.prev_named_sibling() {
                return Some(Self::new(sibling, self.source));
            }
            current_node = current_node.parent()?;
        }
    }

    fn text(&self) -> &str {
        let start_byte = self.node.start_byte() as usize;
        let end_byte = self.node.end_byte() as usize;
        &self.source[start_byte..end_byte]
    }
}

pub struct AncestorIterator<'a> {
    node: Option<NodeWithSource<'a>>,
}

impl<'a> AncestorIterator<'a> {
    fn new(node: &NodeWithSource<'a>) -> Self {
        Self {
            node: Some(node.clone()),
        }
    }
}

impl<'a> Iterator for AncestorIterator<'a> {
    type Item = NodeWithSource<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.as_ref().cloned()?;
        self.node = node
            .node
            .parent()
            .map(|parent| NodeWithSource::new(parent, node.source));
        Some(node)
    }
}

pub struct ChildrenIterator<'a> {
    cursor: Option<CursorWrapper<'a>>,
}

impl<'a> ChildrenIterator<'a> {
    fn new(node: &NodeWithSource<'a>) -> Self {
        let mut cursor = CursorWrapper::new(node.node.walk(), node.source);
        Self {
            cursor: cursor.goto_first_child().then_some(cursor),
        }
    }
}

impl<'a> Iterator for ChildrenIterator<'a> {
    type Item = NodeWithSource<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.cursor.as_mut()?;
        let node = c.node();
        if !c.goto_next_sibling() {
            self.cursor = None;
        }
        Some(node)
    }
}
