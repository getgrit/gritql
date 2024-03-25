use std::sync::OnceLock;

use crate::language::{fields_for_nodes, Field, Language, SortId, TSLanguage};

static NODE_TYPES_STRING: &str = include_str!("../../../resources/node-types/html-node-types.json");

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
    tree_sitter_html::language().into()
}

#[derive(Debug, Clone)]
pub struct Html {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    language: &'static TSLanguage,
}

impl Html {
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {
        let language = LANGUAGE.get_or_init(|| lang.unwrap_or_else(language));
        let node_types = NODE_TYPES.get_or_init(|| fields_for_nodes(language, NODE_TYPES_STRING));
        let metavariable_sort = language.id_for_node_kind("grit_metavariable", true);
        Self {
            node_types,
            metavariable_sort,
            language,
        }
    }
    pub(crate) fn is_initialized() -> bool {
        LANGUAGE.get().is_some()
    }
}

impl Language for Html {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn language_name(&self) -> &'static str {
        "HTML"
    }
    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[("", "")]
    }

    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }
}




#[cfg(test)]
mod tests {
    use marzano_util::print_node::print_node;

    use crate::language::nodes_from_indices;

    use super::*;

    #[test]
    fn import_variable() {
        let snippet = r#"<Hello attr="foo"></Hello>"#;
        let lang = Html::new(None);
        let mut parser = tree_sitter::Parser::new().unwrap();
        parser.set_language(lang.get_ts_language()).unwrap();
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
        print_node(&nodes[0].node);
    }

    #[test]
    fn print_sexp() {
        let code = r#"<Hello attr="foo"></Hello>"#;
        let mut parser = tree_sitter::Parser::new().unwrap();
        let lang = Html::new(None);
        parser.set_language(lang.get_ts_language()).unwrap();
        let tree = parser.parse(code, None).unwrap().unwrap();
        let root = tree.root_node();
        let sexp = root.to_sexp();
        println!("{sexp}");
    }
}