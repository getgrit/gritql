use super::cursor_wrapper::CursorWrapper;
use grit_util::{error::GritResult, AstCursor, AstNode, ByteRange, CodeRange, Position, Range};
use std::{borrow::Cow, ptr};
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

    pub fn child_by_field_id(&self, field_id: u16) -> Option<Self> {
        self.node
            .child_by_field_id(field_id)
            .map(|child| Self::new(child, self.source))
    }

    pub fn child_by_field_name(&self, field_name: &str) -> Option<Self> {
        self.node
            .child_by_field_name(field_name)
            .map(|child| Self::new(child, self.source))
    }

    pub fn children_by_field_id(&self, field_id: u16) -> impl Iterator<Item = Self> + Clone {
        ChildrenByFieldIterator::new(self, field_id)
    }

    pub fn children_by_field_name(&self, field_name: &str) -> impl Iterator<Item = Self> {
        let field_id = self
            .node
            .language()
            .field_id_for_name(field_name)
            .unwrap_or_default();
        self.children_by_field_id(field_id)
    }

    pub fn named_children(&self) -> impl Iterator<Item = Self> {
        ChildrenIterator::new(self).filter(|child| child.node.is_named())
    }

    pub fn named_children_by_field_id(&self, field_id: u16) -> impl Iterator<Item = Self> + Clone {
        ChildrenByFieldIterator::new(self, field_id).filter(|child| child.node.is_named())
    }

    pub fn named_children_by_field_name(&self, field_name: &str) -> impl Iterator<Item = Self> {
        let field_id = self
            .node
            .language()
            .field_id_for_name(field_name)
            .unwrap_or_default();
        self.named_children_by_field_id(field_id)
    }

    pub fn range(&self) -> Range {
        let ts_range = self.node.range();
        let start = ts_range.start_point();
        let end = ts_range.end_point();
        Range {
            start: Position::new(start.row() + 1, start.column() + 1),
            end: Position::new(end.row() + 1, end.column() + 1),
            start_byte: ts_range.start_byte(),
            end_byte: ts_range.end_byte(),
        }
    }

    pub fn print_node_tree(&self) {
        let mut stack = vec![(self.node.clone(), 0)];
        while let Some((node, depth)) = stack.pop() {
            let sort_id = node.kind_id();
            println!("{:indent$}{}: {:?}", "", sort_id, node, indent = depth * 2);
            for i in (0..node.child_count()).rev() {
                if let Some(child) = node.child(i) {
                    stack.push((child, depth + 1));
                }
            }
        }
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

    fn next_named_node(&self) -> Option<Self> {
        let mut current_node = self.node.clone();
        loop {
            if let Some(sibling) = current_node.next_named_sibling() {
                return Some(Self::new(sibling, self.source));
            }
            current_node = current_node.parent()?;
        }
    }

    fn previous_named_node(&self) -> Option<Self> {
        let mut current_node = self.node.clone();
        loop {
            if let Some(sibling) = current_node.prev_named_sibling() {
                return Some(Self::new(sibling, self.source));
            }
            current_node = current_node.parent()?;
        }
    }

    fn next_sibling(&self) -> Option<Self> {
        self.node
            .next_sibling()
            .map(|sibling| Self::new(sibling, self.source))
    }

    fn previous_sibling(&self) -> Option<Self> {
        self.node
            .prev_sibling()
            .map(|sibling| Self::new(sibling, self.source))
    }

    fn text(&self) -> GritResult<Cow<str>> {
        Ok(self.node.utf8_text(self.source.as_bytes())?)
    }

    fn byte_range(&self) -> ByteRange {
        ByteRange::new(
            self.node.start_byte() as usize,
            self.node.end_byte() as usize,
        )
    }

    fn code_range(&self) -> CodeRange {
        CodeRange::new(self.node.start_byte(), self.node.end_byte(), self.source)
    }

    fn walk(&self) -> impl AstCursor<Node = Self> {
        CursorWrapper::new(self.node.walk(), self.source)
    }
}

impl<'a> From<NodeWithSource<'a>> for CodeRange {
    fn from(value: NodeWithSource<'a>) -> Self {
        Self::new(value.node.start_byte(), value.node.end_byte(), value.source)
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
