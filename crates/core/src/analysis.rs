use anyhow::{anyhow, Result};
use grit_util::{traverse, Ast, AstNode, Order};
use marzano_language::grit_parser::MarzanoGritParser;
use marzano_util::{cursor_wrapper::CursorWrapper, node_with_source::NodeWithSource};
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::Path;

use grit_pattern_matcher::{constants::DEFAULT_FILE_NAME, context::QueryContext, pattern::Pattern};

use crate::pattern_compiler::compiler::{defs_to_filenames, DefsToFilenames};

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
            let src_tree = grit_parser.parse_file(body, Some(Path::new(file_name)))?;
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

/// Using source code alone, find dependents of a pattern.
/// This is *NOT* a good implementation and has numerous performance issues,
/// but it is good enough for `grit patterns test --watch`
///
/// Consider refactoring if this is used in a more performance-critical context
pub fn get_dependents_of_target_patterns_by_traversal_from_src(
    libs: &BTreeMap<String, String>,
    src: &str,
    parser: &mut MarzanoGritParser,
    target_patterns: &[&String],
) -> Result<Vec<String>> {
    let mut dependents = <Vec<String>>::new();
    let node_like = "nodeLike";
    let predicate_call = "predicateCall";

    let tree = parser.parse_file(src, Some(Path::new(DEFAULT_FILE_NAME)))?;

    let DefsToFilenames {
        patterns: pattern_file,
        predicates: predicate_file,
        functions: function_file,
        foreign_functions: foreign_file,
    } = defs_to_filenames(libs, parser, tree.root_node())?;

    let name_to_filename: BTreeMap<&String, &String> = pattern_file
        .iter()
        .chain(predicate_file.iter())
        .chain(function_file.iter())
        .chain(foreign_file.iter())
        .collect();

    let mut traversed_stack = <Vec<String>>::new();
    let mut stack: Vec<marzano_language::language::Tree> = vec![tree];
    while let Some(tree) = stack.pop() {
        let root = tree.root_node();
        let cursor = root.walk();

        for n in traverse(cursor, Order::Pre).filter(|n| {
            n.node.is_named() && (n.node.kind() == node_like || n.node.kind() == predicate_call)
        }) {
            let name = n
                .child_by_field_name("name")
                .ok_or_else(|| anyhow!("missing name of nodeLike"))?;
            let name = name.text()?;
            let name = name.trim().to_string();

            if target_patterns.contains(&&name) {
                while let Some(e) = traversed_stack.pop() {
                    dependents.push(e);
                }
            }
            if let Some(file_name) = name_to_filename.get(&name) {
                if let Some(tree) = find_child_tree_definition(
                    file_name,
                    parser,
                    libs,
                    &mut traversed_stack,
                    &name,
                )? {
                    stack.push(tree);
                }
            }
        }
    }
    Ok(dependents)
}

/// Attempt to find where a pattern is defined
fn find_child_tree_definition(
    file_name: &str,
    parser: &mut MarzanoGritParser,
    libs: &BTreeMap<String, String>,
    traversed_stack: &mut Vec<String>,
    name: &str,
) -> Result<Option<marzano_language::language::Tree>> {
    if !traversed_stack.contains(&name.to_string()) {
        if let Some(file_body) = libs.get(file_name) {
            traversed_stack.push(name.to_owned());
            let tree = parser.parse_file(file_body, Some(Path::new(file_name)))?;
            return Ok(Some(tree));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use grit_pattern_matcher::has_rewrite;
    use marzano_language::{python::Python, target_language::TargetLanguage};

    use crate::pattern_compiler::src_to_problem_libs;

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
            .parse_file(&src_code, Some(Path::new("test.grit")))
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
            .parse_file(&src_code, Some(Path::new("test.grit")))
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
            .parse_file(&src_code, Some(Path::new("test.grit")))
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
            .parse_file(&src_code, Some(Path::new("test.grit")))
            .unwrap();
        let decided = is_async(&parsed.root_node(), &libs, &mut parser).unwrap();
        assert!(decided);
    }

    #[test]
    fn test_is_rewrite() {
        let pattern_src = r#"
            `console.log` => `console.error`
        "#
        .to_string();
        let libs = BTreeMap::new();
        let problem = src_to_problem_libs(
            pattern_src.to_string(),
            &libs,
            TargetLanguage::default(),
            None,
            None,
            None,
            None,
        )
        .unwrap()
        .problem;

        println!("problem: {:?}", problem);

        assert!(has_rewrite(&problem.pattern));
    }

    #[test]
    fn test_is_not_rewrite() {
        let pattern_src = r#"
            `console.log($msg)` where {
                $msg <: not contains `foo`
            }
        "#
        .to_string();
        let libs = BTreeMap::new();
        let problem = src_to_problem_libs(
            pattern_src.to_string(),
            &libs,
            TargetLanguage::default(),
            None,
            None,
            None,
            None,
        )
        .unwrap()
        .problem;

        println!("problem: {:?}", problem);

        assert!(!has_rewrite(&problem.pattern));
    }

    #[test]
    fn test_is_rewrite_with_pattern_call() {
        let pattern_src = r#"
            pattern pattern_with_rewrite() {
                `console.log($msg)` => `console.error($msg)`
            }
            pattern_with_rewrite()
        "#
        .to_string();
        let mut libs = BTreeMap::new();
        let problem = src_to_problem_libs(
            pattern_src.to_string(),
            &libs,
            TargetLanguage::default(),
            None,
            None,
            None,
            None,
        )
        .unwrap()
        .problem;

        println!("problem: {:?}", problem);

        assert!(has_rewrite(&problem.pattern));
    }
}
