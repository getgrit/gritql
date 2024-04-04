use std::ptr;

use crate::position::Range;

use super::cursor_wrapper::CursorWrapper;
use grit_util::{AstCursor, AstNode};
use tree_sitter::Node;

/// A TreeSitter node, including a reference to the source code from which it
/// was parsed.
#[derive(Clone, Debug)]
pub struct NodeWithSource<'a> {
    pub node: Node<'a>,
    pub source: &'a str,
}

impl<'a> NodeWithSource<'a> {
    pub fn new(node: Node<'a>, source: &'a str) -> Self {
        Self { node, source }
    }

    pub fn children_by_field_id(&self, field_id: u16) -> impl Iterator<Item = Self> + Clone {
        ChildrenByFieldIterator::new(self, field_id)
    }

    pub fn named_children(&self) -> impl Iterator<Item = Self> {
        ChildrenIterator::new(self).filter(|child| child.node.is_named())
    }

    pub fn range(&self) -> Range {
        Range::from(self.node.range())
    }
}

impl<'a> PartialEq for NodeWithSource<'a> {
    fn eq(&self, other: &Self) -> bool {
        // We can compare source by pointer instead of comparing the entire
        // source strings. This implies that two nodes cannot be equal if they
        // point to different (even cloned) source references.
        self.node == other.node && ptr::eq(self.source.as_ptr(), other.source.as_ptr())
    }
}

impl<'a> AstNode for NodeWithSource<'a> {
    fn ancestors(&self) -> impl Iterator<Item = Self> {
        AncestorIterator::new(self)
    }

    fn children(&self) -> impl Iterator<Item = Self> {
        ChildrenIterator::new(self)
    }

    fn parent(&self) -> Option<Self> {
        self.node
            .parent()
            .map(|parent| Self::new(parent, self.source))
    }

    fn next_sibling(&self) -> Option<Self> {
        let mut current_node = self.node.clone();
        loop {
            if let Some(sibling) = current_node.next_sibling() {
                return Some(Self::new(sibling, self.source));
            }
            current_node = current_node.parent()?;
        }
    }

    fn previous_sibling(&self) -> Option<Self> {
        let mut current_node = self.node.clone();
        loop {
            if let Some(sibling) = current_node.prev_sibling() {
                return Some(Self::new(sibling, self.source));
            }
            current_node = current_node.parent()?;
        }
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
        self.node = node.parent();
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

#[derive(Clone)]
pub struct ChildrenByFieldIterator<'a> {
    cursor: Option<CursorWrapper<'a>>,
    field_id: u16,
}

impl<'a> ChildrenByFieldIterator<'a> {
    fn new(node: &NodeWithSource<'a>, field_id: u16) -> Self {
        let mut cursor = CursorWrapper::new(node.node.walk(), node.source);
        Self {
            cursor: cursor.goto_first_child().then_some(cursor),
            field_id,
        }
    }
}

impl<'a> Iterator for ChildrenByFieldIterator<'a> {
    type Item = NodeWithSource<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.cursor.as_mut()?;
        while c.field_id() != Some(self.field_id) {
            if !c.goto_next_sibling() {
                self.cursor = None;
                return None;
            }
        }
        let node = c.node();
        if !c.goto_next_sibling() {
            self.cursor = None;
        }
        Some(node)
    }
}
