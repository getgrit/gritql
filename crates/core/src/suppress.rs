use marzano_language::{
    language::Language,
    parent_traverse::{ParentTraverse, TreeSitterParentCursor},
};
use tree_sitter::{Node, Range};

use crate::binding::Binding;
use crate::resolve;
use anyhow::Result;

pub(crate) fn is_binding_suppressed(
    binding: &Binding,
    lang: &impl Language,
    current_name: &Option<String>,
) -> Result<bool> {
    let (src, node) = match binding {
        Binding::Node(src, node) => (src, node),
        Binding::String(_, _) => return Ok(false),
        Binding::List(src, node, _) => (src, node),
        Binding::Empty(src, node, _) => (src, node),
        Binding::FileName(_) => return Ok(false),
        Binding::ConstantRef(_) => return Ok(false),
    };
    let target_range = node.range();
    for n in
        node.children(&mut node.walk())
            .chain(ParentTraverse::new(TreeSitterParentCursor::new(
                node.clone(),
            )))
    {
        let mut cursor = n.walk();
        let children = n.children(&mut cursor);
        for c in children {
            if !(lang.is_comment(c.kind_id()) || lang.is_comment_wrapper(&c)) {
                continue;
            }
            if is_suppress_comment(&c, src, &target_range, current_name, lang)? {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn is_suppress_comment(
    comment_node: &Node,
    src: &str,
    target_range: &Range,
    current_name: &Option<String>,
    lang: &impl Language,
) -> Result<bool> {
    let child_range = comment_node.range();
    let text = comment_node.utf8_text(src.as_bytes())?;
    let inline_suppress = child_range.end_point().row() >= target_range.start_point().row()
        && child_range.end_point().row() <= target_range.end_point().row();
    if !inline_suppress {
        let pre_suppress = comment_applies_to_range(comment_node, target_range, lang, src)?
            && comment_occupies_entire_line(text.as_ref(), &comment_node.range(), src)?;
        if !pre_suppress {
            return Ok(false);
        }
    }
    if !text.contains("grit-ignore") {
        return Ok(false);
    }
    let comment_text = text.trim();
    let ignore_spec = match comment_text.split("grit-ignore").collect::<Vec<_>>().get(1) {
        Some(s) => match s.split(':').next() {
            Some(s) => s.trim(),
            None => return Ok(true),
        },
        None => return Ok(true),
    };
    if ignore_spec.is_empty()
        || ignore_spec
            .chars()
            .next()
            .is_some_and(|c| !c.is_alphanumeric() && c != '_')
    {
        return Ok(true);
    }
    if current_name.is_none() {
        return Ok(false);
    }
    let current_name = current_name.as_ref().unwrap();
    let ignored_rules = ignore_spec.split(',').map(|s| s.trim()).collect::<Vec<_>>();
    Ok(ignored_rules.contains(&&current_name[..]))
}

fn comment_applies_to_range(
    comment_node: &Node,
    range: &Range,
    lang: &impl Language,
    src: &str,
) -> Result<bool> {
    let mut applicable = resolve!(comment_node.next_named_sibling());
    while let Some(next) = applicable.next_named_sibling() {
        if !lang.is_comment(applicable.kind_id())
            && !lang.is_comment_wrapper(&applicable)
            // Some languages have significant whitespace; continue until we find a non-whitespace non-comment node
            && !applicable.utf8_text(src.as_bytes())?.trim().is_empty()
        {
            break;
        }
        applicable = next;
    }
    let applicable_range = applicable.range();
    Ok(applicable_range.start_point().row() == range.start_point().row())
}

fn comment_occupies_entire_line(text: &str, range: &Range, src: &str) -> Result<bool> {
    let code = src
        .lines()
        .skip(range.start_point().row() as usize)
        .take((range.end_point().row() - range.start_point().row() + 1) as usize)
        .collect::<Vec<_>>()
        .join("\n");
    Ok(code.trim() == text.trim())
}
