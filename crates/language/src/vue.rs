use crate::language::{fields_for_nodes, Field, MarzanoLanguage, NodeTypes, SortId, TSLanguage};
use grit_util::{
    error::{GritPatternError, GritResult},
    traverse, Language, Order,
};
use marzano_util::{cursor_wrapper::CursorWrapper, node_with_source::NodeWithSource};
use std::sync::OnceLock;
use tree_sitter::{Node, Parser, Range};

static NODE_TYPES_STRING: &str = include_str!("../../../resources/node-types/vue-node-types.json");
static NODE_TYPES: OnceLock<Vec<Vec<Field>>> = OnceLock::new();
static LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();

#[cfg(not(feature = "builtin-parser"))]
fn language() -> TSLanguage {
    unimplemented!(
        "tree-sitter parser must be initialized before use when [builtin-parser] is off."
    )
}
#[cfg(feature = "builtin-parser")]
fn language() -> TSLanguage {
    tree_sitter_vue::language().into()
}

#[derive(Debug, Clone)]
pub struct Vue {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    comment_sort: SortId,
    language: &'static TSLanguage,
}

impl Vue {
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {
        let language = LANGUAGE.get_or_init(|| lang.unwrap_or_else(language));
        let node_types = NODE_TYPES.get_or_init(|| fields_for_nodes(language, NODE_TYPES_STRING));
        let metavariable_sort = language.id_for_node_kind("grit_metavariable", true);
        let comment_sort = language.id_for_node_kind("comment", true);
        Self {
            node_types,
            metavariable_sort,
            comment_sort,
            language,
        }
    }
    pub(crate) fn is_initialized() -> bool {
        LANGUAGE.get().is_some()
    }
}

impl NodeTypes for Vue {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

impl Language for Vue {
    use_marzano_delegate!();

    fn language_name(&self) -> &'static str {
        "Vue"
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[("", "")]
    }

    fn make_single_line_comment(&self, text: &str) -> String {
        format!("<!-- {} -->\n", text)
    }
}

impl<'a> MarzanoLanguage<'a> for Vue {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn is_comment_sort(&self, id: SortId) -> bool {
        id == self.comment_sort
    }

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }
}

fn is_lang_attribute(node: &Node, text: &[u8], name_array: Option<&[&str]>) -> bool {
    node.child_by_field_name("name")
        .is_some_and(|name| name.utf8_text(text).unwrap().trim() == "lang")
        && node
            .child_by_field_name("value")
            .and_then(|n| {
                if n.kind() == "attribute_value" {
                    Some(n)
                } else if n.kind() == "quoted_attribute_value" {
                    n.child_by_field_name("value")
                } else {
                    None
                }
            })
            .is_some_and(|lang| {
                name_array
                    .map(|name_array| name_array.contains(&lang.utf8_text(text).unwrap().trim()))
                    .unwrap_or(true)
            })
}

fn append_code_range(
    node: &Node,
    text: &[u8],
    ranges: &mut Vec<Range>,
    parent_node_kind: &str,
    name_array: Option<&[&str]>,
) {
    if node.kind() == parent_node_kind {
        let mut cursor = node.walk();
        if let Some(mut attributes) = node
            .child_by_field_name("start_tag")
            // nb. This type matches the grammar
            .map(|n| n.children_by_field_name("atributes", &mut cursor))
        {
            if attributes.any(|n| is_lang_attribute(&n, text, name_array)) {
                if let Some(code) = node.child_by_field_name("text") {
                    ranges.push(code.range())
                }
            }
        };
    }
}

// could probably be done better using a tree-sitter query?
pub(crate) fn get_vue_ranges(
    file: &str,
    parent_node_kind: &str,
    name_array: Option<&[&str]>,
) -> GritResult<Vec<Range>> {
    let vue = Vue::new(None);
    let mut parser = Parser::new().map_err(|e| GritPatternError::new(e.to_string()))?;
    let text = file.as_bytes();
    parser
        .set_language(vue.get_ts_language())
        .map_err(|e| GritPatternError::new(e.to_string()))?;
    let tree = parser
        .parse(file, None)
        .map_err(|e| GritPatternError::new(e.to_string()))?
        .ok_or(GritPatternError::new("missing tree"))?;
    let cursor = tree.walk();
    let mut ranges = Vec::new();
    for n in traverse(CursorWrapper::new(cursor, file), Order::Pre) {
        append_code_range(&n.node, text, &mut ranges, parent_node_kind, name_array)
    }
    Ok(ranges)
}

#[cfg(test)]
mod tests {

    use crate::language::nodes_from_indices;

    use super::*;

    #[test]
    fn simple_vue() {
        let snippet = "<template> <h1>{{ message }}</h1> </template>";
        let lang = Vue::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }
}
