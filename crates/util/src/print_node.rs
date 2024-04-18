fn print_node_rec(node: &tree_sitter::Node, depth: usize) {
    let indent = " ".repeat(depth * 2);
    let kind = node.kind();
    let start = node.start_byte();
    let end = node.end_byte();
    // grit-ignore no_println_in_core
    println!("{}{}: {}-{}", indent, kind, start, end);
    for child in node.children(&mut node.walk()) {
        print_node_rec(&child, depth + 1);
    }
}

pub fn print_node(node: &tree_sitter::Node) {
    print_node_rec(node, 0);
}
