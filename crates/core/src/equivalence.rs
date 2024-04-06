use crate::binding::Binding;
use grit_util::AstNode;
use itertools::{EitherOrBoth, Itertools};
use marzano_util::node_with_source::NodeWithSource;

impl<'a> Binding<'a> {
    /// Checks whether two bindings are equivalent.
    ///
    /// Bindings are considered equivalent if they refer to the same thing.
    pub fn is_equivalent_to(&self, other: &'a Binding) -> bool {
        // covers Node, and List with one element
        if let (Some(s1), Some(s2)) = (self.singleton(), other.singleton()) {
            return are_equivalent(&s1, &s2);
        }

        match self {
            // should never occur covered by singleton
            Self::Node(node1) => match other {
                Self::Node(node2) => are_equivalent(node1, node2),
                Self::String(str, range) => {
                    str[range.start_byte as usize..range.end_byte as usize] == self.text()
                }
                Self::FileName(_) | Self::List(..) | Self::Empty(..) | Self::ConstantRef(_) => {
                    false
                }
            },
            Self::List(parent_node1, field1) => match other {
                Self::List(parent_node2, field2) => parent_node1
                    .named_children_by_field_id(*field1)
                    .zip_longest(parent_node2.named_children_by_field_id(*field2))
                    .all(|zipped| match zipped {
                        EitherOrBoth::Both(node1, node2) => are_equivalent(&node1, &node2),
                        EitherOrBoth::Left(_) | EitherOrBoth::Right(_) => false,
                    }),
                Self::String(..)
                | Self::FileName(_)
                | Self::Node(..)
                | Self::Empty(..)
                | Self::ConstantRef(_) => false,
            },
            // I suspect matching kind is too strict
            Self::Empty(node1, field1) => match other {
                Self::Empty(node2, field2) => {
                    node1.node.kind_id() == node2.node.kind_id() && field1 == field2
                }
                Self::String(..)
                | Self::FileName(_)
                | Self::Node(..)
                | Self::List(..)
                | Self::ConstantRef(_) => false,
            },
            Self::ConstantRef(c1) => other.as_constant().map_or(false, |c2| *c1 == c2),
            Self::String(s1, range) => {
                s1[range.start_byte as usize..range.end_byte as usize] == other.text()
            }
            Self::FileName(s1) => other.as_filename().map_or(false, |s2| *s1 == s2),
        }
    }
}

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
    if node1.text() == node2.text() {
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
