use super::compiler::parse_one;
use anyhow::anyhow;
use anyhow::Result;
use grit_util::{traverse, Order};
use marzano_util::cursor_wrapper::CursorWrapper;
use std::collections::BTreeMap;
use tree_sitter::{Node, Parser};

/// Walks the call tree and returns true if the predicate is true for any node.
/// This is potentially error-prone, so not entirely recommended
pub fn walk_call_tree(
    node: &Node,
    source: &str,
    libs: &BTreeMap<String, String>,
    grit_parser: &mut Parser,
    is_true: &dyn Fn(&Node, &str) -> Result<bool>,
) -> Result<bool> {
    let cursor = node.walk();
    for n in traverse(CursorWrapper::new(cursor, source), Order::Pre) {
        let n = n.node;
        if is_true(&n, source)? {
            return Ok(true);
        }
        if !(n.is_named() && n.kind() == "nodeLike") {
            continue;
        }
        let name = n
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of nodeLike"))?;
        let name = name.utf8_text(source.as_bytes())?;
        let name = name.trim();
        let maybe_call = libs
            .iter()
            .find(|(k, _)| k.strip_suffix(".grit").unwrap_or(k) == name);
        if let Some((k, v)) = maybe_call {
            let src_tree = parse_one(grit_parser, v, k)?;
            let source_file = src_tree.root_node();
            return walk_call_tree(&source_file, v, libs, grit_parser, is_true);
        }
    }
    Ok(false)
}

pub fn is_multifile(
    root: &Node,
    source: &str,
    libs: &BTreeMap<String, String>,
    grit_parser: &mut Parser,
) -> Result<bool> {
    walk_call_tree(root, source, libs, grit_parser, &|n, _| {
        Ok(n.kind() == "files")
    })
}

pub fn has_limit(
    root: &Node,
    source: &str,
    libs: &BTreeMap<String, String>,
    grit_parser: &mut Parser,
) -> Result<bool> {
    walk_call_tree(root, source, libs, grit_parser, &|n, _| {
        Ok(n.kind() == "patternLimit")
    })
}

#[allow(dead_code)]
pub fn is_async(
    root: &Node,
    source: &str,
    libs: &BTreeMap<String, String>,
    grit_parser: &mut Parser,
) -> Result<bool> {
    walk_call_tree(root, source, libs, grit_parser, &|n, current_source| {
        if n.kind() != "nodeLike" {
            return Ok(false);
        }
        let name = n
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of nodeLike"))?;
        let name = name.utf8_text(current_source.as_bytes())?;
        let name = name.trim();
        Ok(name == "llm_chat")
    })
}

/// Return true if the pattern attempts to define itself
pub fn defines_itself(root: &Node, source: &str, root_name: &str) -> Result<bool> {
    let cursor = root.walk();
    for n in traverse(CursorWrapper::new(cursor, source), Order::Pre) {
        let n = n.node;
        if n.kind() != "patternDefinition" {
            continue;
        }
        let name = n
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of patternDefinition"))?;
        let name = name.utf8_text(source.as_bytes())?;
        let name = name.trim();
        if name == root_name {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::parse::make_grit_parser;

    use super::*;

    #[test]
    fn test_non_async() {
        let src_code = r#"
            `console.log` => log(message="hello")
        "#
        .to_string();
        let libs = BTreeMap::new();
        let mut parser = make_grit_parser().unwrap();
        let parsed = parse_one(&mut parser, &src_code, "test.grit").unwrap();
        assert!(!is_async(&parsed.root_node(), &src_code, &libs, &mut parser).unwrap());
    }

    #[test]
    fn test_async_direct_call() {
        let src_code = r#"
            `console.log` => llm_chat(messages=[])
        "#
        .to_string();
        let libs = BTreeMap::new();
        let mut parser = make_grit_parser().unwrap();
        let parsed = parse_one(&mut parser, &src_code, "test.grit").unwrap();
        assert!(is_async(&parsed.root_node(), &src_code, &libs, &mut parser).unwrap());
    }

    #[test]
    fn test_inline_indirect_call() {
        let src_code = r#"
            function foo() {
                return llm_chat(messages=[])
            }
            `console.log` => foo()
        "#
        .to_string();
        let libs = BTreeMap::new();
        let mut parser = make_grit_parser().unwrap();
        let parsed = parse_one(&mut parser, &src_code, "test.grit").unwrap();
        assert!(is_async(&parsed.root_node(), &src_code, &libs, &mut parser).unwrap());
    }

    #[test]
    fn test_async_module_indirect() {
        let src_code = r#"
            `console.log` => async_foo()
        "#
        .to_string();
        let mut libs = BTreeMap::new();
        libs.insert(
            "async_foo.grit".to_string(),
            "llm_chat(messages=[])".to_string(),
        );
        let mut parser = make_grit_parser().unwrap();
        let parsed = parse_one(&mut parser, &src_code, "test.grit").unwrap();
        let decided = is_async(&parsed.root_node(), &src_code, &libs, &mut parser).unwrap();
        assert!(decided);
    }
}
