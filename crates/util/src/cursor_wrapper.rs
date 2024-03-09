use tree_sitter::{Node, TreeCursor};
use tree_sitter_traversal::Cursor;

pub struct CursorWrapper<'a>(TreeCursor<'a>);

impl<'a> Cursor for CursorWrapper<'a> {
    type Node = Node<'a>;

    fn goto_first_child(&mut self) -> bool {
        self.0.goto_first_child()
    }

    fn goto_parent(&mut self) -> bool {
        self.0.goto_parent()
    }

    fn goto_next_sibling(&mut self) -> bool {
        self.0.goto_next_sibling()
    }

    fn node(&self) -> Self::Node {
        self.0.node()
    }
}

impl<'a> From<TreeCursor<'a>> for CursorWrapper<'a> {
    fn from(value: TreeCursor<'a>) -> Self {
        Self(value)
    }
}
