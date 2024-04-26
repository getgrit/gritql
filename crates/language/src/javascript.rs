use crate::{
    js_like::{
        js_like_get_statement_sorts, js_like_is_comment, js_skip_snippet_compilation_sorts,
        jslike_check_replacements, MarzanoJsLikeParser,
    },
    language::{
        fields_for_nodes, kind_and_field_id_for_names, Field, FieldId, MarzanoLanguage, NodeTypes,
        SortId, TSLanguage, Tree,
    },
};
use grit_util::{AstNode, Language, Parser, Range, Replacement};
use marzano_util::node_with_source::NodeWithSource;
use std::sync::OnceLock;

static NODE_TYPES_STRING: &str =
    include_str!("../../../resources/node-types/javascript-node-types.json");
static NODE_TYPES: OnceLock<Vec<Vec<Field>>> = OnceLock::new();
static LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static SKIP_SNIPPET_COMPILATION_SORTS: OnceLock<Vec<(SortId, FieldId)>> = OnceLock::new();
static STATEMENT_SORTS: OnceLock<Vec<SortId>> = OnceLock::new();
static OPTIONAL_EMPTY_FIELD_COMPILATION: OnceLock<Vec<(SortId, FieldId)>> = OnceLock::new();

#[cfg(not(feature = "builtin-parser"))]
fn language() -> TSLanguage {
    unimplemented!(
        "tree-sitter parser must be initialized before use when [builtin-parser] is off."
    )
}
#[cfg(feature = "builtin-parser")]
fn language() -> TSLanguage {
    tree_sitter_javascript::language().into()
}

#[derive(Debug, Clone)]
pub struct JavaScript {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    comment_sort: SortId,
    jsx_sort: SortId,
    statement_sorts: &'static [SortId],
    language: &'static TSLanguage,
    skip_snippet_compilation_sorts: &'static Vec<(SortId, FieldId)>,
    optional_empty_field_compilation: &'static Vec<(SortId, FieldId)>,
}

impl JavaScript {
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {
        let language = LANGUAGE.get_or_init(|| lang.unwrap_or_else(language));
        let node_types = NODE_TYPES.get_or_init(|| fields_for_nodes(language, NODE_TYPES_STRING));
        let metavariable_sort = language.id_for_node_kind("grit_metavariable", true);
        let comment_sort = language.id_for_node_kind("comment", true);
        let jsx_sort = language.id_for_node_kind("jsx_expression", true);

        let statement_sorts = STATEMENT_SORTS.get_or_init(|| js_like_get_statement_sorts(language));

        let skip_snippet_compilation_sorts = SKIP_SNIPPET_COMPILATION_SORTS.get_or_init(|| {
            kind_and_field_id_for_names(language, js_skip_snippet_compilation_sorts())
        });

        let optional_empty_field_compilation = OPTIONAL_EMPTY_FIELD_COMPILATION.get_or_init(|| {
            Vec::new()
            // kind_and_field_id_for_names(language, js_optional_empty_field_compilation())
        });

        Self {
            node_types,
            metavariable_sort,
            comment_sort,
            jsx_sort,
            statement_sorts,
            language,
            skip_snippet_compilation_sorts,
            optional_empty_field_compilation,
        }
    }
    pub(crate) fn is_initialized() -> bool {
        LANGUAGE.get().is_some()
    }
}

impl NodeTypes for JavaScript {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

impl Language for JavaScript {
    type Node<'a> = NodeWithSource<'a>;

    fn language_name(&self) -> &'static str {
        "JavaScript"
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[
            ("", ""),
            ("import ", " from 'GRIT_PACKAGE';"),
            ("GRIT_VALUE ", " GRIT_VALUE"),
            ("class GRIT_CLASS ", " {}"),
            ("class GRIT_CLASS { ", " GRIT_PROP = 'GRIT_VALUE'; }"),
            ("", "  function GRIT_FUNCTION() {}"),
            ("GRIT_OBJ = { ", " }"),
            ("class GRIT_CLASS { ", " }"),
            ("GRIT_VAR = ", ""),
            ("<f>", "</f>"),
            ("<f ", " />"),
            ("function GRIT_FN(", ") {}"),
            ("var ", ";"),
            ("", " class GRIT_CLASS {}"),
            ("function GRIT_FN(GRIT_ARG:", ") { }"),
            ("import { ", " } from 'GRIT_PACKAGE'"),
            ("function GRIT_FN(GRIT_ARG", ") { }"),
        ]
    }

    fn is_comment(&self, node: &NodeWithSource) -> bool {
        MarzanoLanguage::is_comment_node(self, node)
    }

    fn is_metavariable(&self, node: &NodeWithSource) -> bool {
        MarzanoLanguage::is_metavariable_node(self, node)
    }

    fn is_statement(&self, node: &NodeWithSource) -> bool {
        self.statement_sorts.contains(&node.node.kind_id())
    }

    fn alternate_metavariable_kinds(&self) -> &[&'static str] {
        &["template_content"]
    }

    // assumes trim doesn't do anything otherwise range is off
    fn comment_text_range(&self, node: &NodeWithSource) -> Option<Range> {
        let content_text = node.text().ok()?;
        let content_text = content_text.trim();
        let mut range = node.range();
        if content_text.starts_with("//") {
            range.adjust_columns(2, 0).then_some(range)
        } else if content_text.starts_with("/*") && content_text.ends_with("*/") {
            range.adjust_columns(2, -2).then_some(range)
        } else {
            None
        }
    }

    fn check_replacements(&self, n: NodeWithSource<'_>, orphan_ranges: &mut Vec<Replacement>) {
        jslike_check_replacements(n, orphan_ranges)
    }
}

impl<'a> MarzanoLanguage<'a> for JavaScript {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn get_parser(&self) -> Box<dyn Parser<Tree = Tree>> {
        Box::new(MarzanoJsLikeParser::new(self))
    }

    // fn optional_empty_field_compilation(
    //     &self,
    //     sort_id: SortId,
    //     field_id: crate::language::FieldId,
    // ) -> bool {
    //     self.optional_empty_field_compilation
    //         .iter()
    //         .any(|(s, f)| *s == sort_id && *f == field_id)
    // }

    fn skip_snippet_compilation_of_field(&self, sort_id: SortId, field_id: FieldId) -> bool {
        self.skip_snippet_compilation_sorts
            .iter()
            .any(|(s, f)| *s == sort_id && *f == field_id)
    }

    fn is_comment_sort(&self, id: SortId) -> bool {
        id == self.comment_sort
    }

    fn is_comment_node(&self, node: &NodeWithSource) -> bool {
        self.is_comment_sort(node.node.kind_id())
            || js_like_is_comment(node, self.comment_sort, self.jsx_sort)
    }

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }
}

#[cfg(test)]
mod tests {

    use crate::language::nodes_from_indices;

    use super::*;

    #[test]
    fn pair_snippet() {
        let snippet = "$key: $value";
        let lang = JavaScript::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }
}
