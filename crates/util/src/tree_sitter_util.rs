use tree_sitter::{Node, TreeCursor};

pub fn children_by_field_name_count(node: &Node, field_name: &str) -> usize {
    let mut cursor = node.walk();
    node.children_by_field_name(field_name, &mut cursor)
        .filter(|c| c.is_named())
        .count()
}

pub fn children_by_field_id_count(node: &Node, field_id: u16) -> usize {
    let mut cursor = node.walk();
    node.children_by_field_id(field_id, &mut cursor)
        .filter(|c| c.is_named())
        .count()
}

pub fn named_children_by_field_name<'a, 'b>(
    node: &Node<'a>,
    cursor: &'b mut TreeCursor<'a>,
    field_name: &'b str,
) -> impl Iterator<Item = Node<'a>> + 'b {
    node.children_by_field_name(field_name, cursor)
        .filter(|n| n.is_named())
}

pub fn named_children_by_field_id<'a, 'b>(
    node: &Node<'a>,
    cursor: &'b mut TreeCursor<'a>,
    field_id: u16,
) -> impl Iterator<Item = Node<'a>> + 'b {
    node.children_by_field_id(field_id, cursor)
        .filter(|n| n.is_named())
}
