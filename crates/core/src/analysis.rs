use anyhow::{anyhow, Result};
use grit_util::{traverse, Ast, AstNode, Order};
use marzano_language::grit_parser::MarzanoGritParser;
use marzano_util::{cursor_wrapper::CursorWrapper, node_with_source::NodeWithSource};
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::Path;

/// Walks the call tree and returns true if the predicate is true for any node.
/// This is potentially error-prone, so not entirely recommended
fn walk_call_tree(
    node: &NodeWithSource,
    libs: &BTreeMap<String, String>,
    grit_parser: &mut MarzanoGritParser,
    predicate: &dyn Fn(&NodeWithSource) -> Result<bool>,
) -> Result<bool> {
    let cursor = node.walk();
    for n in traverse(cursor, Order::Pre) {
        if predicate(&n)? {
            return Ok(true);
        }
        if !(n.node.is_named() && n.node.kind() == "nodeLike") {
            continue;
        }
        let name = n
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of nodeLike"))?;
        let name = name.text()?;
        let name = OsStr::new(name.trim());
        let maybe_call = libs
            .iter()
            .find(|(k, _)| k.strip_suffix(".grit").unwrap_or(k) == name);
        if let Some((file_name, body)) = maybe_call {
            let src_tree = grit_parser.parse(body, Some(Path::new(file_name)))?;
            return walk_call_tree(&src_tree.root_node(), libs, grit_parser, predicate);
        }
    }
    Ok(false)
}

pub fn is_multifile(
    root: &NodeWithSource,
    libs: &BTreeMap<String, String>,
    grit_parser: &mut MarzanoGritParser,
) -> Result<bool> {
    walk_call_tree(root, libs, grit_parser, &|n| Ok(n.node.kind() == "files"))
}

pub fn has_limit(
    root: &NodeWithSource,
    libs: &BTreeMap<String, String>,
    grit_parser: &mut MarzanoGritParser,
) -> Result<bool> {
    walk_call_tree(root, libs, grit_parser, &|n| {
        Ok(n.node.kind() == "patternLimit")
    })
}

#[allow(dead_code)]
pub fn is_async(
    root: &NodeWithSource,
    libs: &BTreeMap<String, String>,
    grit_parser: &mut MarzanoGritParser,
) -> Result<bool> {
    walk_call_tree(root, libs, grit_parser, &|n| {
        if n.node.kind() != "nodeLike" {
            return Ok(false);
        }
        let name = n
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of nodeLike"))?;
        let name = name.text()?;
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
        let name = name.text()?;
        let name = name.trim();
        if name == root_name {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_non_async() {
        let src_code = r#"
            `console.log` => log(message="hello")
        "#
        .to_string();
        let libs = BTreeMap::new();
        let mut parser = MarzanoGritParser::new().unwrap();
        let parsed = parser
            .parse(&src_code, Some(Path::new("test.grit")))
            .unwrap();
        assert!(!is_async(&parsed.root_node(), &libs, &mut parser).unwrap());
    }

    #[test]
    fn test_async_direct_call() {
        let src_code = r#"
            `console.log` => llm_chat(messages=[])
        "#
        .to_string();
        let libs = BTreeMap::new();
        let mut parser = MarzanoGritParser::new().unwrap();
        let parsed = parser
            .parse(&src_code, Some(Path::new("test.grit")))
            .unwrap();
        assert!(is_async(&parsed.root_node(), &libs, &mut parser).unwrap());
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
        let mut parser = MarzanoGritParser::new().unwrap();
        let parsed = parser
            .parse(&src_code, Some(Path::new("test.grit")))
            .unwrap();
        assert!(is_async(&parsed.root_node(), &libs, &mut parser).unwrap());
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
        let mut parser = MarzanoGritParser::new().unwrap();
        let parsed = parser
            .parse(&src_code, Some(Path::new("test.grit")))
            .unwrap();
        let decided = is_async(&parsed.root_node(), &libs, &mut parser).unwrap();
        assert!(decided);
    }
}
