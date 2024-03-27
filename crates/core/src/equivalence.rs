use tree_sitter::Node;

use crate::binding::Binding;
use marzano_util::tree_sitter_util::children_by_field_id_count;

pub fn are_bindings_equivalent(binding1: &Binding, binding2: &Binding) -> bool {
    // covers Node, and List with one element
    if let Some(binding1) = binding1.singleton() {
        if let Some(binding2) = binding2.singleton() {
            return are_equivalent(binding1.0, &binding1.1, binding2.0, &binding2.1);
        }
    }
    match binding1 {
        // should never occur covered by singleton
        Binding::Node(source1, node1) => match binding2 {
            Binding::Node(source2, node2) => are_equivalent(source1, node1, source2, node2),
            Binding::String(str, range) => {
                str[range.start_byte as usize..range.end_byte as usize] == binding1.text()
            }
            Binding::FileName(_)
            | Binding::List(..)
            | Binding::Empty(..)
            | Binding::ConstantRef(_) => false,
        },
        Binding::List(source1, parent_node1, field1) => match binding2 {
            Binding::List(source2, parent_node2, field2) => {
                let mut cursor1 = parent_node1.walk();
                let mut cursor2 = parent_node2.walk();
                children_by_field_id_count(parent_node1, *field1)
                    == children_by_field_id_count(parent_node2, *field2)
                    && parent_node1
                        .children_by_field_id(*field1, &mut cursor1)
                        .zip(parent_node2.children_by_field_id(*field2, &mut cursor2))
                        .all(|(node1, node2)| are_equivalent(source1, &node1, source2, &node2))
            }
            Binding::String(..)
            | Binding::FileName(_)
            | Binding::Node(..)
            | Binding::Empty(..)
            | Binding::ConstantRef(_) => false,
        },
        // I suspect matching kind is too strict
        Binding::Empty(_, node1, field1) => match binding2 {
            Binding::Empty(_, node2, field2) => {
                node1.kind_id() == node2.kind_id() && field1 == field2
            }
            Binding::String(..)
            | Binding::FileName(_)
            | Binding::Node(..)
            | Binding::List(..)
            | Binding::ConstantRef(_) => false,
        },
        Binding::ConstantRef(c1) => match binding2 {
            Binding::ConstantRef(c2) => c1 == c2,
            Binding::String(..)
            | Binding::FileName(_)
            | Binding::Node(..)
            | Binding::List(..)
            | Binding::Empty(..) => false,
        },
        Binding::String(s1, range) => {
            s1[range.start_byte as usize..range.end_byte as usize] == binding2.text()
        }
        Binding::FileName(s1) => binding2.as_filename().map_or(false, |s2| *s1 == s2),
    }
}

/**
 * Checks whether two nodes are equivalent.
 *
 * We define two nodes to be equivalent if they have the same sort (kind) and
 * equivalent named fields.
 *
 * TODO: Improve performance. Equivalence checks happen often so we want them to
 * be fast. The current implementation requires a traversal of the tree on all
 * named fields, which can be slow for large nodes. It also creates a cursor
 * at each traversal step.
 *
 * Potential improvements:
 * 1. Use cursors that are passed as arguments -- not clear if this would be faster.
 * 2. Precompute hashes on all nodes, which define the equivalence relation. The check then becomes O(1).
 */
pub fn are_equivalent(source1: &str, node1: &Node, source2: &str, node2: &Node) -> bool {
    // If the source is identical, we consider the nodes equivalent.
    // This covers most cases of constant nodes.
    // We may want a more precise check here eventually, but this is a good start.
    if source1[node1.start_byte() as usize..node1.end_byte() as usize]
        == source2[node2.start_byte() as usize..node2.end_byte() as usize]
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
    if node1.kind_id() != node2.kind_id() {
        return false;
    }

    // If the node kinds are the same, then we need to check the named fields.
    let mut cursor1 = node1.walk();
    let mut cursor2 = node2.walk();
    let named_fields1 = node1.named_children(&mut cursor1);
    let named_fields2 = node2.named_children(&mut cursor2);

    // If the number of named fields is different, then the nodes are not equivalent.
    // This also covers the case of mistached optional and "multiple" fields.
    if named_fields1.len() != named_fields2.len() {
        return false;
    }

    // This is effectively a leaf node. If two leaf nodes have different sources (see above),
    // then they are not equivalent.
    // If they do not have the same sources, we consider them different.
    if named_fields1.len() == 0 {
        return false;
    }

    // And now recursing on the named fields.
    for (child1, child2) in named_fields1.zip(named_fields2) {
        if !are_equivalent(source1, &child1, source2, &child2) {
            return false;
        }
    }

    true
}
