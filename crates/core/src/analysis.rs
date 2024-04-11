use crate::pattern_compiler::parse_one;
use anyhow::{anyhow, Result};
use grit_util::AstNode;
use grit_util::{traverse, Order};
use marzano_util::{cursor_wrapper::CursorWrapper, node_with_source::NodeWithSource};
use std::collections::BTreeMap;
use std::ffi::OsStr;
use tree_sitter::Parser;

/// Walks the call tree and returns true if the predicate is true for any node.
/// This is potentially error-prone, so not entirely recommended
fn walk_call_tree(
    node: &NodeWithSource,
    libs: &BTreeMap<String, String>,
    grit_parser: &mut Parser,
    predicate: &dyn Fn(&NodeWithSource) -> Result<bool>,
) -> Result<bool> {
    let cursor = node.node.walk();
    for n in traverse(CursorWrapper::new(cursor, node.source), Order::Pre) {
        if predicate(&n)? {
            return Ok(true);
        }
        if !(n.node.is_named() && n.node.kind() == "nodeLike") {
            continue;
        }
        let name = n
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of nodeLike"))?;
        let name = OsStr::new(name.text().trim());
        let maybe_call = libs
            .iter()
            .find(|(k, _)| k.strip_suffix(".grit").unwrap_or(k) == name);
        if let Some((k, v)) = maybe_call {
            let src_tree = parse_one(grit_parser, v, k)?;
            let source_file = NodeWithSource::new(src_tree.root_node(), v);
            return walk_call_tree(&source_file, libs, grit_parser, predicate);
        }
    }
    Ok(false)
}

pub fn is_multifile(
    root: &NodeWithSource,
    libs: &BTreeMap<String, String>,
    grit_parser: &mut Parser,
) -> Result<bool> {
    walk_call_tree(root, libs, grit_parser, &|n| Ok(n.node.kind() == "files"))
}

pub fn has_limit(
    root: &NodeWithSource,
    libs: &BTreeMap<String, String>,
    grit_parser: &mut Parser,
) -> Result<bool> {
    walk_call_tree(root, libs, grit_parser, &|n| {
        Ok(n.node.kind() == "patternLimit")
    })
}

#[allow(dead_code)]
pub fn is_async(
    root: &NodeWithSource,
    libs: &BTreeMap<String, String>,
    grit_parser: &mut Parser,
) -> Result<bool> {
    walk_call_tree(root, libs, grit_parser, &|n| {
        if n.node.kind() != "nodeLike" {
            return Ok(false);
        }
        let name = n
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of nodeLike"))?;
        let name = name.text().trim();
        Ok(name == "llm_chat")
    })
}

/// Return true if the pattern attempts to define itself
pub fn defines_itself(root: &NodeWithSource, root_name: &str) -> Result<bool> {
    let cursor = root.node.walk();
    for n in traverse(CursorWrapper::new(cursor, root.source), Order::Pre) {
        if n.node.kind() != "patternDefinition" {
            continue;
        }
        let name = n
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of patternDefinition"))?;
        let name = name.text().trim();
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
        assert!(!is_async(
            &NodeWithSource::new(parsed.root_node(), &src_code),
            &libs,
            &mut parser
        )
        .unwrap());
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
        assert!(is_async(
            &NodeWithSource::new(parsed.root_node(), &src_code),
            &libs,
            &mut parser
        )
        .unwrap());
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
        assert!(is_async(
            &NodeWithSource::new(parsed.root_node(), &src_code),
            &libs,
            &mut parser
        )
        .unwrap());
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
        let decided = is_async(
            &NodeWithSource::new(parsed.root_node(), &src_code),
            &libs,
            &mut parser,
        )
        .unwrap();
        assert!(decided);
    }
}
