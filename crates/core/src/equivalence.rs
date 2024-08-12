use grit_util::AstNode;
use itertools::{EitherOrBoth, Itertools};
use marzano_util::node_with_source::NodeWithSource;

/// Checks whether two nodes are equivalent.
///
/// We define two nodes to be equivalent if they have the same sort (kind) and
/// equivalent named fields.
///
/// TODO: Improve performance. Equivalence checks happen often so we want them to
/// be fast. The current implementation requires a traversal of the tree on all
/// named fields, which can be slow for large nodes. It also creates a cursor
/// at each traversal step.
///
/// Potential improvements:
/// 1. Use cursors that are passed as arguments -- not clear if this would be faster.
/// 2. Precompute hashes on all nodes, which define the equivalence relation. The check then becomes O(1).
pub fn are_equivalent(node1: &NodeWithSource, node2: &NodeWithSource) -> bool {
    // If the source is identical, we consider the nodes equivalent.
    // This covers most cases of constant nodes.
    // We may want a more precise check here eventually, but this is a good start.
    if node1
        .text()
        .is_ok_and(|text1| node2.text().is_ok_and(|text2| text1 == text2))
    {
        return true;
    }

    // If the node kinds are different, then the nodes are not equivalent.
    // Common case, should go first.
    // TODO: this check is wrong, eg. we want to match
    // shorthand_property_identifier and identifier in js.
    // currently fixed by moving this check to after string matching, but
    // still not enough consider that a given snippet could be any one
    // of several nested nodes.
    if node1.node.kind_id() != node2.node.kind_id() {
        return false;
    }

    // If the node kinds are the same, then we need to check the named fields.
    let named_fields1 = node1.named_children();
    let named_fields2 = node2.named_children();

    // If there are no children, this is effectively a leaf node. If two leaf
    // nodes have different sources (see above), then they are not equivalent.
    // If they do not have the same sources, we consider them different.
    let mut is_empty = true;

    // Recurse through the named fields to find the first mismatch.
    // Differences in length are caught by the use of `EitherOrBoth`.
    // This also covers the case of mistached optional and "multiple" fields.
    let are_equivalent = named_fields1
        .zip_longest(named_fields2)
        .all(|zipped| match zipped {
            EitherOrBoth::Both(child1, child2) => {
                is_empty = false;
                are_equivalent(&child1, &child2)
            }
            EitherOrBoth::Left(_) | EitherOrBoth::Right(_) => false,
        });

    are_equivalent && !is_empty
}
