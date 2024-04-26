use crate::{
    js_like::{
        js_like_disregarded_field_values, js_like_get_statement_sorts, js_like_is_comment,
        js_like_skip_snippet_compilation_sorts, jslike_check_replacements, MarzanoJsLikeParser,
    },
    language::{
        check_disregarded_field_map, fields_for_nodes, kind_and_field_id_for_field_map,
        kind_and_field_id_for_names, Field, FieldId, MarzanoLanguage, NodeTypes, SortId,
        TSLanguage, Tree,
    },
};
use grit_util::{AstNode, Language, Parser, Range, Replacement};
use marzano_util::node_with_source::NodeWithSource;
use std::{sync::OnceLock};

static NODE_TYPES_STRING: &str = include_str!("../../../resources/node-types/tsx-node-types.json");
static NODE_TYPES: OnceLock<Vec<Vec<Field>>> = OnceLock::new();
static LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static SKIP_SNIPPET_COMPILATION_SORTS: OnceLock<Vec<(SortId, FieldId)>> = OnceLock::new();
static STATEMENT_SORTS: OnceLock<Vec<SortId>> = OnceLock::new();
static DISREGARDED_SNIPPET_FIELDS: OnceLock<Vec<(SortId, FieldId, Option<Vec<&str>>)>> =
    OnceLock::new();

#[cfg(not(feature = "builtin-parser"))]
fn language() -> TSLanguage {
    unimplemented!(
        "tree-sitter parser must be initialized before use when [builtin-parser] is off."
    )
}
#[cfg(feature = "builtin-parser")]
fn language() -> TSLanguage {
    tree_sitter_typescript::language_tsx().into()
}

#[derive(Debug, Clone)]
pub struct Tsx {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    comment_sort: SortId,
    jsx_sort: SortId,
    statement_sorts: &'static [SortId],
    language: &'static TSLanguage,
    skip_snippet_compilation_sorts: &'static Vec<(SortId, FieldId)>,
    disregarded_snippet_fields: &'static Vec<(SortId, FieldId, Option<Vec<&'static str>>)>,
}

impl Tsx {
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {
        let language = LANGUAGE.get_or_init(|| lang.unwrap_or_else(language));
        let node_types = NODE_TYPES.get_or_init(|| fields_for_nodes(language, NODE_TYPES_STRING));
        let metavariable_sort = language.id_for_node_kind("grit_metavariable", true);
        let comment_sort = language.id_for_node_kind("comment", true);
        let jsx_sort = language.id_for_node_kind("jsx_expression", true);
        let skip_snippet_compilation_sorts = SKIP_SNIPPET_COMPILATION_SORTS.get_or_init(|| {
            kind_and_field_id_for_names(language, js_like_skip_snippet_compilation_sorts())
        });

        let disregarded_snippet_fields = DISREGARDED_SNIPPET_FIELDS.get_or_init(|| {
            kind_and_field_id_for_field_map(language, js_like_disregarded_field_values())
        });

        let statement_sorts = STATEMENT_SORTS.get_or_init(|| js_like_get_statement_sorts(language));

        Self {
            node_types,
            metavariable_sort,
            comment_sort,
            jsx_sort,
            statement_sorts,
            language,
            skip_snippet_compilation_sorts,
            disregarded_snippet_fields,
        }
    }
    pub(crate) fn is_initialized() -> bool {
        LANGUAGE.get().is_some()
    }
}

impl NodeTypes for Tsx {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

impl Language for Tsx {
    type Node<'a> = NodeWithSource<'a>;

    fn language_name(&self) -> &'static str {
        "TSX"
    }

    fn alternate_metavariable_kinds(&self) -> &[&'static str] {
        &["template_content", "template_literal_type_content"]
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
            ("<f>", " </f>"),
            ("<f ", " />"),
            ("function GRIT_FN(", ") {}"),
            ("var ", ";"),
            ("", " class GRIT_CLASS {}"),
            ("function GRIT_FN(GRIT_ARG:", ") { }"),
            ("import { ", " } from 'GRIT_PACKAGE'"),
            ("function GRIT_FN(GRIT_ARG", ") { }"),
            ("GRIT_FN<{ ", " }>();"),
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

impl<'a> MarzanoLanguage<'a> for Tsx {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn get_parser(&self) -> Box<dyn Parser<Tree = Tree>> {
        Box::new(MarzanoJsLikeParser::new(self))
    }

    fn is_disregarded_snippet_field(
        &self,
        sort_id: SortId,
        field_id: crate::language::FieldId,
        field_node: &Option<NodeWithSource<'_>>,
    ) -> bool {
        check_disregarded_field_map(
            self.disregarded_snippet_fields,
            sort_id,
            field_id,
            field_node,
        )
    }

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
    use marzano_util::print_node::print_node;

    use crate::language::nodes_from_indices;

    use super::*;
    #[test]
    fn pair_snippet() {
        let snippet = "$key: $val";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn call_snippet() {
        let snippet = "$setter($val)";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn destructured_assignment_snippet() {
        let snippet = "const { isOpen } = this.state";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn destructured_assignment_snippet_metavar() {
        let snippet = "const {$props} = this.props";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn class_property_assignment() {
        let snippet = "this.state = { $states }";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn method_two_args() {
        let snippet = "this.setState($setStateBody, $secondArg)";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn method_one_arg() {
        let snippet = "this.setState($setStateBody)";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn object() {
        let snippet = "{ $stateUpdate }";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn arrow_function() {
        let snippet = "() => { $bodyLike }";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn property_assign() {
        let snippet = "this.$name = $value";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn property_access_meta() {
        let snippet = "this.$name";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn identifier() {
        let snippet = "viewState";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn member_expression() {
        let snippet = "PageContainer.Header";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
        println!("{:#?}", nodes);
    }

    #[test]
    fn double_property() {
        let snippet = "this.state.$name";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn property_access() {
        let snippet = "this.props";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn constructor() {
        let snippet = "constructor($_) { $constructorBody }";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn assign_snippet() {
        let snippet = "$name = $obj";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn assign_object() {
        let snippet = "const stdlib = { $activities } as const";
        let lang = Tsx::new(None);
        let mut parser = tree_sitter::Parser::new().unwrap();
        parser.set_language(lang.get_ts_language()).unwrap();
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn assign_array() {
        let snippet = "const stdlib = [$old] as const";
        let lang = Tsx::new(None);
        let mut parser = tree_sitter::Parser::new().unwrap();
        parser.set_language(lang.get_ts_language()).unwrap();
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn import_variable() {
        let snippet = r#"import $legacy_image from "next/legacy/image""#;
        let lang = Tsx::new(None);
        let mut parser = tree_sitter::Parser::new().unwrap();
        parser.set_language(lang.get_ts_language()).unwrap();
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
        print_node(&nodes[0].node);
    }

    #[test]
    fn condition_snippet() {
        let snippet = "if ($cond) { $cond_true }";
        let lang = Tsx::new(None);
        let mut parser = tree_sitter::Parser::new().unwrap();
        parser.set_language(lang.get_ts_language()).unwrap();
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn as_snippet() {
        let snippet = "as";
        let lang = Tsx::new(None);
        let mut parser = tree_sitter::Parser::new().unwrap();
        parser.set_language(lang.get_ts_language()).unwrap();
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn template_snippet() {
        let snippet = r#"`foo ${bar}`"#;
        let lang = Tsx::new(None);
        let mut parser = tree_sitter::Parser::new().unwrap();
        parser.set_language(lang.get_ts_language()).unwrap();
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn extends_component() {
        let snippet = r#"extends Component"#;
        let lang = Tsx::new(None);
        let mut parser = tree_sitter::Parser::new().unwrap();
        parser.set_language(lang.get_ts_language()).unwrap();
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn optional_chaining() {
        let snippet = r#"!$x.length"#;
        let lang = Tsx::new(None);
        let mut parser = tree_sitter::Parser::new().unwrap();
        parser.set_language(lang.get_ts_language()).unwrap();
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        println!("{:#?}", nodes);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn not_equal() {
        let snippet = r#"=="#;
        let lang = Tsx::new(None);
        let mut parser = tree_sitter::Parser::new().unwrap();
        parser.set_language(lang.get_ts_language()).unwrap();
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn required_parameters() {
        let snippet = r#"Symbol = $_"#;
        let lang = Tsx::new(None);
        let mut parser = tree_sitter::Parser::new().unwrap();
        parser.set_language(lang.get_ts_language()).unwrap();
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        println!("{:#?}", nodes);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn type_annotation_pair() {
        let snippet = r#"showingCount?: boolean"#;
        let lang = Tsx::new(None);
        let mut parser = tree_sitter::Parser::new().unwrap();
        parser.set_language(lang.get_ts_language()).unwrap();
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        println!("{:#?}", nodes);
        assert!(!nodes.is_empty());
    }
}
