use grit_util::{AstNode, Language};
use itertools::{EitherOrBoth, Itertools};
use marzano_util::node_with_source::NodeWithSource;
use tree_sitter::Range;

pub(crate) fn is_suppress_comment<'a>(
    comment_node: &'a NodeWithSource,
    target_range: &Range,
    current_name: Option<&str>,
    lang: &impl Language<Node<'a> = NodeWithSource<'a>>,
) -> bool {
    let child_range = comment_node.node.range();
    let text = match comment_node.text() {
        Ok(text) => text,
        Err(_) => return false,
    };
    let inline_suppress = child_range.end_point().row() >= target_range.start_point().row()
        && child_range.end_point().row() <= target_range.end_point().row();
    if !inline_suppress {
        let pre_suppress = comment_applies_to_range(comment_node, target_range, lang)
            && comment_occupies_entire_line(&text, comment_node);
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

fn comment_applies_to_range<'a>(
    comment_node: &'a NodeWithSource,
    range: &Range,
    lang: &impl Language<Node<'a> = NodeWithSource<'a>>,
) -> bool {
    let Some(mut applicable) = comment_node.next_named_node() else {
        return false;
    };
    while let Some(next) = applicable.next_named_node() {
        if !lang.is_comment(&applicable)
            // Some languages have significant whitespace; continue until we find a non-whitespace non-comment node
            && applicable.text().is_ok_and(|t| !t.trim().is_empty())
        {
            break;
        }
        applicable = next;
    }
    let applicable_range = applicable.node.range();
    applicable_range.start_point().row() == range.start_point().row()
}

fn comment_occupies_entire_line(text: &str, node: &NodeWithSource) -> bool {
    let start_row = node.node.start_position().row() as usize;
    let end_row = node.node.end_position().row() as usize;
    node.source
        .lines()
        .skip(start_row)
        .take(end_row - start_row + 1)
        .zip_longest(text.split('\n'))
        .all(|zipped| {
            if let EitherOrBoth::Both(src_line, text_line) = zipped {
                src_line.trim() == text_line.trim()
            } else {
                false
            }
        })
}
