use crate::{constants::*, traverse, AstNode, ByteRange, CodeRange, Order, Range};
use regex::Regex;

pub enum GritMetaValue {
    Underscore,
    Dots,
    Variable(String),
}

pub trait Language: Sized {
    type Node<'a>: AstNode;

    fn language_name(&self) -> &'static str;

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)];

    fn metavariable_prefix(&self) -> &'static str {
        "$"
    }

    fn comment_prefix(&self) -> &'static str {
        "//"
    }

    fn metavariable_prefix_substitute(&self) -> &'static str {
        "µ"
    }

    fn metavariable_regex(&self) -> &'static Regex {
        &VARIABLE_REGEX
    }

    fn replaced_metavariable_regex(&self) -> &'static Regex {
        &REPLACED_VARIABLE_REGEX
    }

    fn metavariable_bracket_regex(&self) -> &'static Regex {
        &BRACKET_VAR_REGEX
    }

    fn exact_variable_regex(&self) -> &'static Regex {
        &EXACT_VARIABLE_REGEX
    }

    fn exact_replaced_variable_regex(&self) -> &'static Regex {
        &EXACT_REPLACED_VARIABLE_REGEX
    }

    fn is_comment(&self, node: &Self::Node<'_>) -> bool;

    fn is_metavariable(&self, node: &Self::Node<'_>) -> bool;

    #[allow(unused_variables)]
    fn is_statement(&self, node: &Self::Node<'_>) -> bool {
        false
    }

    // assumes trim doesn't do anything otherwise range is off
    fn comment_text_range(&self, node: &Self::Node<'_>) -> Option<ByteRange> {
        Some(node.byte_range())
    }

    // in languages we pad such as python or yaml there are
    // some kinds of nodes we don't want to pad, such as python strings.
    // this function identifies those nodes.
    #[allow(unused_variables)]
    fn should_skip_padding(&self, node: &Self::Node<'_>) -> bool {
        false
    }

    #[allow(unused_variables)]
    fn get_skip_padding_ranges_for_snippet(&self, snippet: &str) -> Vec<CodeRange> {
        Vec::new()
    }

    #[allow(unused_variables)]
    fn get_skip_padding_ranges(&self, node: &Self::Node<'_>) -> Vec<CodeRange> {
        let mut ranges = Vec::new();
        for n in traverse(node.walk(), Order::Pre) {
            if self.should_skip_padding(&n) {
                ranges.push(n.code_range())
            }
        }
        ranges
    }

    fn substitute_metavariable_prefix(&self, src: &str) -> String {
        self.metavariable_regex()
            .replace_all(
                src,
                format!("{}$1", self.metavariable_prefix_substitute()).as_str(),
            )
            .to_string()
    }

    fn snippet_metavariable_to_grit_metavariable(&self, src: &str) -> Option<GritMetaValue> {
        src.trim()
            .strip_prefix(self.metavariable_prefix_substitute())
            .map(|s| match s {
                "_" => GritMetaValue::Underscore,
                "..." => GritMetaValue::Dots,
                _ => {
                    let mut s = s.to_owned();
                    s.insert_str(0, self.metavariable_prefix());
                    GritMetaValue::Variable(s)
                }
            })
    }

    /// Check for nodes that should be removed or replaced.
    ///
    /// This is used to "repair" the program after rewriting, such as by
    /// deleting orphaned ranges (like a variable declaration without any
    /// variables). If the node should be removed, it adds a range with a `None`
    /// value. If the node should be replaced, it adds a range with the
    /// replacement value.
    #[allow(unused_variables)]
    fn check_replacements(&self, node: Self::Node<'_>, replacements: &mut Vec<Replacement>) {}

    #[allow(unused_variables)]
    fn take_padding(&self, current: char, next: Option<char>) -> Option<char> {
        if current.is_whitespace() {
            Some(current)
        } else {
            None
        }
    }

    /// Whether snippets should be padded.
    ///
    /// This is generally `true` for languages with relevant whitespace.
    fn should_pad_snippet(&self) -> bool {
        false
    }

    fn make_single_line_comment(&self, text: &str) -> String {
        format!("// {text}\n")
    }
}

#[derive(Clone, Debug)]
pub struct Replacement {
    pub range: Range,
    pub replacement: &'static str,
}

impl Replacement {
    pub fn new(range: Range, replacement: &'static str) -> Self {
        Self { range, replacement }
    }
}

impl From<&Replacement> for (std::ops::Range<usize>, usize) {
    fn from(replacement: &Replacement) -> Self {
        (
            (replacement.range.start_byte as usize)..(replacement.range.end_byte as usize),
            replacement.replacement.len(),
        )
    }
}
