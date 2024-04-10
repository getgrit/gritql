use anyhow::Result;
use grit_util::{traverse, Order};
use marzano_language::target_language::TargetLanguage;
use marzano_util::cursor_wrapper::CursorWrapper;
use tree_sitter::Tree;

/// Heals the invalid AST by fixing small errors that have a clear solution
pub(crate) fn heal_invalid_ast(
    tree: &Tree,
    mut src: String,
    lang: &TargetLanguage,
) -> Result<String> {
    let cursor = tree.walk();
    for n in traverse(CursorWrapper::new(cursor, &src), Order::Pre) {
        if n.node.kind() == "function_definition" {
            println!("Checking orphaned node: {:?}", n.node.kind());
        }
    }

    // Heal the tree
    Ok(src)
}
