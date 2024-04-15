use crate::binding::Binding;
use grit_util::AstNode;
use marzano_language::language::Language;
use marzano_util::node_with_source::NodeWithSource;

impl<'a> Binding<'a> {
    /// Returns the padding to use for inserting the given text.
    pub(crate) fn get_insertion_padding(
        &self,
        text: &str,
        is_first: bool,
        language: &impl Language,
    ) -> Option<String> {
        match self {
            Self::List(node, field_id) => {
                let children: Vec<_> = node.children_by_field_id(*field_id).collect();
                if children.is_empty() {
                    return None;
                }
                calculate_padding(&children, text, is_first, language).or_else(|| {
                    if children.len() == 1 {
                        let child = children.first().unwrap();
                        if child.node.end_position().row() > child.node.start_position().row()
                            && !child.text().is_ok_and(|t| t.ends_with('\n'))
                            && !text.starts_with('\n')
                        {
                            return Some("\n".to_string());
                        }
                    }
                    None
                })
            }
            Self::Node(node) => {
                if language.is_statement(node.node.kind_id())
                    && !node.text().is_ok_and(|t| t.ends_with('\n'))
                    && !text.starts_with('\n')
                {
                    Some("\n".to_string())
                } else {
                    None
                }
            }
            Self::String(..) | Self::FileName(_) | Self::Empty(..) | Self::ConstantRef(_) => None,
        }
    }
}

fn calculate_padding(
    children: &[NodeWithSource],
    insert: &str,
    is_first: bool,
    language: &impl Language,
) -> Option<String> {
    let named_children: Vec<_> = children
        .iter()
        .filter(|child| child.node.is_named())
        .collect();
    let mut separator: Option<String> = None;

    if named_children.len() == 1 {
        let last_named = named_children.last().unwrap();
        let last_child = children.last().unwrap();
        let trailing = last_child.source
            [last_named.node.end_byte() as usize..last_child.node.end_byte() as usize]
            .to_string();
        separator = if !trailing.is_empty() {
            Some(trailing)
        } else {
            None
        }
    } else {
        for i in 0..named_children.len() - 1 {
            let child = named_children.get(i).unwrap();
            let next_child = named_children.get(i + 1).unwrap();
            let range_between = child.node.end_byte()..next_child.node.start_byte();
            if range_between.start == range_between.end {
                return None;
            }
            let curr =
                child.source[range_between.start as usize..range_between.end as usize].to_string();
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
    let trailing = last_child.source
        [last_named.node.end_byte() as usize..last_child.node.end_byte() as usize]
        .to_string();
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
