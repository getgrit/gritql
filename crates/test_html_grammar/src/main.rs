

fn main() {
    let code = r#"<Hello attr="foo"></Hello>"#;
    let mut parser = tree_sitter::Parser::new().unwrap();
    parser.set_language(&tree_sitter_html::language().into()).unwrap();
    let tree = parser.parse(code, None).unwrap().unwrap();
    let root = tree.root_node();
    let sexp = root.to_sexp();
    println!("{sexp}");
}
