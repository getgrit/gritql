use crate::{
    binding::Binding,
    pattern::resolved_pattern::{ResolvedPattern, ResolvedSnippet},
};
use anyhow::Result;
use itertools::Itertools;
use marzano_language::{language::Language, target_language::TargetLanguage};
use tree_sitter::Node;

pub(crate) fn normalize_insert<'a>(
    binding: &Binding<'a>,
    with: &mut ResolvedPattern<'a>,
    is_first: bool,
    language: &TargetLanguage,
) -> Result<()> {
    let ResolvedPattern::Snippets(ref mut snippets) = with else {
        return Ok(());
    };
    let Some(ResolvedSnippet::Text(text)) = snippets.front() else {
        return Ok(());
    };
    let insert_padding = match binding {
        Binding::List(src, node, field_id) => {
            let mut cursor = node.walk();
            let children = node
                .children_by_field_id(*field_id, &mut cursor)
                .collect_vec();
            if children.is_empty() {
                return Ok(());
            }
            calculate_padding(src, &children, text, is_first, language).or_else(|| {
                if children.len() == 1 {
                    let child = children.first().unwrap();
                    let child_text = child.utf8_text(src.as_bytes()).ok()?;
                    if child.end_position().row() > child.start_position().row()
                        && !child_text.ends_with('\n')
                        && !text.starts_with('\n')
                    {
                        return Some("\n".to_string());
                    }
                }
                None
            })
        }
        Binding::Node(src, node) => {
            let node_text = node.utf8_text(src.as_bytes())?;
            if language.is_statement(node.kind_id())
                && !node_text.ends_with('\n')
                && !text.starts_with('\n')
            {
                Some("\n".to_string())
            } else {
                None
            }
        }
        Binding::String(..)
        | Binding::FileName(_)
        | Binding::Empty(..)
        | Binding::ConstantRef(_) => None,
    };
    if let Some(padding) = insert_padding {
        if padding.chars().next() != binding.text().chars().last() {
            snippets.push_front(ResolvedSnippet::Text(padding.into()));
        }
    }
    Ok(())
}

fn calculate_padding(
    src: &str,
    children: &[Node],
    insert: &str,
    is_first: bool,
    language: &TargetLanguage,
) -> Option<String> {
    let named_children: Vec<_> = children.iter().filter(|child| child.is_named()).collect();
    let mut separator: Option<String> = None;

    if named_children.len() == 1 {
        let last_named = named_children.last().unwrap();
        let last_child = children.last().unwrap();
        let trailing =
            src[last_named.end_byte() as usize..last_child.end_byte() as usize].to_string();
        separator = if !trailing.is_empty() {
            Some(trailing)
        } else {
            None
        }
    } else {
        for i in 0..named_children.len() - 1 {
            let child = named_children.get(i).unwrap();
            let next_child = named_children.get(i + 1).unwrap();
            let range_between = child.end_byte()..next_child.start_byte();
            if range_between.start == range_between.end {
                return None;
            }
            let curr = src[range_between.start as usize..range_between.end as usize].to_string();
            if let Some(ref sep) = separator {
                if curr == *sep || curr.contains(sep) {
                    continue;
                }
                if sep.contains(&curr) {
                    separator = Some(curr);
                    continue;
                }
                return None;
            } else {
                separator = Some(curr);
            }
        }
    }

    separator.as_ref()?;
    let separator = separator.unwrap();
    let last_named = named_children.last().unwrap();
    let last_child = children.last().unwrap();
    let trailing = src[last_named.end_byte() as usize..last_child.end_byte() as usize].to_string();
    let separator = if is_first {
        adjust_separator_start(&separator, &trailing)
    } else {
        separator
    };
    let mut separator = adjust_separator_end(&separator, insert);
    // In languages with semantic white space we already pad during linearization, so no need to pad twice
    if language.should_pad_snippet() {
        let no_whitespace = separator.trim_end_matches(|c: char| c.is_whitespace() && c != '\n');
        if no_whitespace.ends_with('\n') {
            separator.truncate(no_whitespace.len());
        }
    }
    if separator.is_empty() {
        None
    } else {
        Some(separator)
    }
}

fn adjust_separator_start(separator: &str, trailing: &str) -> String {
    let separator_chars: Vec<_> = separator.chars().collect();
    let trailing_chars: Vec<_> = trailing.chars().collect();

    for i in 0..std::cmp::min(separator.len(), trailing.len()) {
        if separator_chars[..i + 1] == trailing_chars[trailing_chars.len() - i - 1..] {
            return separator_chars[i + 1..].iter().collect();
        }
    }

    separator.to_string()
}

fn adjust_separator_end(separator: &str, insert: &str) -> String {
    let separator_chars: Vec<_> = separator.chars().collect();
    let insert_chars: Vec<_> = insert.chars().collect();

    for i in 0..std::cmp::min(separator.len(), insert.len()) {
        if separator_chars[separator_chars.len() - i - 1..] == insert_chars[..i + 1] {
            return separator_chars[..separator_chars.len() - i - 1]
                .iter()
                .collect();
        }
    }

    separator.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_separator_start() {
        assert_eq!(adjust_separator_start(", ", ","), " ");
        assert_eq!(adjust_separator_start("\n", ""), "\n");
        assert_eq!(adjust_separator_start("abcdef", "xyzabc"), "def");
        assert_eq!(adjust_separator_start("\n\nabcdef", "xyzabc\n"), "\nabcdef");
    }

    #[test]
    fn test_adjust_separator_end() {
        assert_eq!(adjust_separator_end("Hello, ", ", World"), "Hello");
        assert_eq!(adjust_separator_end("\n", ""), "\n");
        assert_eq!(adjust_separator_end("\n", "\n"), "");
    }
}
