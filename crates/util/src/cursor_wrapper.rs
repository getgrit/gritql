use crate::node_with_source::NodeWithSource;
use grit_util::AstCursor;
use tree_sitter::TreeCursor;

pub struct CursorWrapper<'a> {
    cursor: TreeCursor<'a>,
    source: &'a str,
}

impl<'a> CursorWrapper<'a> {
    pub fn new(cursor: TreeCursor<'a>, source: &'a str) -> Self {
        Self { cursor, source }
    }
}

impl<'a> AstCursor for CursorWrapper<'a> {
    type Node = NodeWithSource<'a>;

    fn goto_first_child(&mut self) -> bool {
        self.cursor.goto_first_child()
    }

    fn goto_parent(&mut self) -> bool {
        self.cursor.goto_parent()
    }

    fn goto_next_sibling(&mut self) -> bool {
        self.cursor.goto_next_sibling()
    }

    fn node(&self) -> Self::Node {
        NodeWithSource::new(self.cursor.node(), self.source)
    }
}
