// iteratate through parents of a node, beginning with the node itself.
// should we wrap this in the same interface used by tree-sitter-traversal?
pub struct ParentTraverse<P> {
    cursor: Option<P>,
}

pub trait HasParent {
    /// The type of the nodes which the cursor points at; the cursor is always pointing
    /// at exactly one of this type.
    type Node;

    /// Move this cursor to the parent of its current node.
    ///
    /// This returns `true` if the cursor successfully moved, and returns `false`
    /// if there was no parent node (the cursor was already on the root node).
    fn goto_parent(&mut self) -> bool;

    /// Get the node which the cursor is currently pointing at.
    fn node(&self) -> Self::Node;
}

impl<P> ParentTraverse<P> {
    pub fn new(c: P) -> Self {
        ParentTraverse { cursor: Some(c) }
    }
}

pub struct TreeSitterParentCursor<'a> {
    node: tree_sitter::Node<'a>,
}

impl<'a> TreeSitterParentCursor<'a> {
    pub fn new(node: tree_sitter::Node<'a>) -> Self {
        TreeSitterParentCursor { node }
    }
}
impl<'a> From<tree_sitter::Node<'a>> for TreeSitterParentCursor<'a> {
    fn from(node: tree_sitter::Node<'a>) -> Self {
        TreeSitterParentCursor { node }
    }
}

impl<'a> HasParent for TreeSitterParentCursor<'a> {
    type Node = tree_sitter::Node<'a>;

    fn goto_parent(&mut self) -> bool {
        let parent = self.node.parent();
        if parent.is_none() {
            return false;
        }
        self.node = parent.unwrap();
        true
    }

    fn node(&self) -> Self::Node {
        self.node.clone()
    }
}

impl<P> Iterator for ParentTraverse<P>
where
    P: HasParent,
{
    type Item = P::Node;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.cursor.as_mut()?;
        let node = c.node();
        if !c.goto_parent() {
            self.cursor = None;
        }
        Some(node)
    }
}
