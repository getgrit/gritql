use itertools::{EitherOrBoth, Itertools};
use marzano_language::{
    language::Language,
    parent_traverse::{ParentTraverse, TreeSitterParentCursor},
};
use tree_sitter::{Node, Range};

use crate::binding::Binding;

impl<'a> Binding<'a> {
    pub(crate) fn is_suppressed(&self, lang: &impl Language, current_name: Option<&str>) -> bool {
        let (src, node) = match self {
            Self::Node(src, node) | Self::List(src, node, _) | Self::Empty(src, node, _) => {
                (src, node)
            }
            Self::String(_, _) | Self::FileName(_) | Self::ConstantRef(_) => return false,
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
                if is_suppress_comment(&c, src, &target_range, current_name, lang) {
                    return true;
                }
            }
        }

        false
    }
}

fn is_suppress_comment(
    comment_node: &Node,
    src: &str,
    target_range: &Range,
    current_name: Option<&str>,
    lang: &impl Language,
) -> bool {
    let child_range = comment_node.range();
    let Ok(text) = comment_node.utf8_text(src.as_bytes()) else {
        return false;
    };
    let inline_suppress = child_range.end_point().row() >= target_range.start_point().row()
        && child_range.end_point().row() <= target_range.end_point().row();
    if !inline_suppress {
        let pre_suppress = comment_applies_to_range(comment_node, target_range, lang, src)
            && comment_occupies_entire_line(text.as_ref(), &comment_node.range(), src);
        if !pre_suppress {
            return false;
        }
    }
    if !text.contains("grit-ignore") {
        return false;
    }
    let comment_text = text.trim();
    let ignore_spec = match comment_text.split_once("grit-ignore") {
        Some((_, s)) => match s.split(':').next() {
            Some(s) => s.trim(),
            None => return true,
        },
        None => return true,
    };
    if ignore_spec.is_empty()
        || ignore_spec
            .chars()
            .next()
            .is_some_and(|c| !c.is_alphanumeric() && c != '_')
    {
        return true;
    }
    let Some(current_name) = current_name else {
        return false;
    };
    ignore_spec
        .split(',')
        .map(str::trim)
        .contains(&current_name)
}

fn comment_applies_to_range(
    comment_node: &Node,
    range: &Range,
    lang: &impl Language,
    src: &str,
) -> bool {
    let Some(mut applicable) = comment_node.next_named_sibling() else {
        return false;
    };
    while let Some(next) = applicable.next_named_sibling() {
        if !lang.is_comment(applicable.kind_id())
            && !lang.is_comment_wrapper(&applicable)
            // Some languages have significant whitespace; continue until we find a non-whitespace non-comment node
            && !applicable.utf8_text(src.as_bytes()).map_or(true, |text| text.trim().is_empty())
        {
            break;
        }
        applicable = next;
    }
    let applicable_range = applicable.range();
    applicable_range.start_point().row() == range.start_point().row()
}

fn comment_occupies_entire_line(text: &str, range: &Range, src: &str) -> bool {
    src.lines()
        .skip(range.start_point().row() as usize)
        .take((range.end_point().row() - range.start_point().row() + 1) as usize)
        .zip_longest(text.split('\n'))
        .all(|zipped| {
            if let EitherOrBoth::Both(src_line, text_line) = zipped {
                src_line.trim() == text_line.trim()
            } else {
                false
            }
        })
}
