use std::borrow::Cow;
use std::sync::OnceLock;

use crate::language::{fields_for_nodes, Field, FieldId, Language, SortId, TSLanguage};
use crate::xscript_util::{self, jslike_check_orphaned, jslike_get_statement_sorts};
use marzano_util::position::Range;
use tree_sitter::{Node, Parser};

static NODE_TYPES_STRING: &str =
    include_str!("../../../resources/node-types/typescript-node-types.json");
static NODE_TYPES: OnceLock<Vec<Vec<Field>>> = OnceLock::new();
static LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static SKIP_SNIPPET_COMPILATION_SORTS: OnceLock<Vec<(SortId, FieldId)>> = OnceLock::new();
static STATEMENT_SORTS: OnceLock<Vec<SortId>> = OnceLock::new();

#[cfg(not(feature = "builtin-parser"))]
fn language() -> TSLanguage {
    unimplemented!(
        "tree-sitter parser must be initialized before use when [builtin-parser] is off."
    )
}
#[cfg(feature = "builtin-parser")]
fn language() -> TSLanguage {
    tree_sitter_typescript::language_typescript().into()
}

#[derive(Debug, Clone)]
pub struct TypeScript {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    comment_sort: SortId,
    statement_sorts: &'static [SortId],
    language: &'static TSLanguage,
    skip_snippet_compilation_sorts: &'static Vec<(SortId, FieldId)>,
}

impl TypeScript {
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {
        let language = LANGUAGE.get_or_init(|| lang.unwrap_or_else(language));
        let node_types = NODE_TYPES.get_or_init(|| fields_for_nodes(language, NODE_TYPES_STRING));
        let metavariable_sort = language.id_for_node_kind("grit_metavariable", true);
        let comment_sort = language.id_for_node_kind("comment", true);
        let skip_snippet_compilation_sorts = SKIP_SNIPPET_COMPILATION_SORTS.get_or_init(|| {
            vec![
                (
                    language.id_for_node_kind("method_definition", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
                (
                    language.id_for_node_kind("function", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
                (
                    language.id_for_node_kind("function_declaration", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
                (
                    language.id_for_node_kind("generator_function", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
                (
                    language.id_for_node_kind("generator_function_declaration", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
                (
                    language.id_for_node_kind("arrow_function", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
                (
                    language.id_for_node_kind("constructor_type", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
                (
                    language.id_for_node_kind("construct_signature", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
                (
                    language.id_for_node_kind("function_type", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
                (
                    language.id_for_node_kind("method_signature", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
                (
                    language.id_for_node_kind("abstract_method_signature", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
                (
                    language.id_for_node_kind("function_signature", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
                (
                    language.id_for_node_kind("function_signature", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
                (
                    language.id_for_node_kind("function_signature", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
                (
                    language.id_for_node_kind("function_signature", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
                (
                    language.id_for_node_kind("function_signature", true),
                    language.field_id_for_name("parenthesis").unwrap(),
                ),
            ]
        });

        let statement_sorts = STATEMENT_SORTS.get_or_init(|| jslike_get_statement_sorts(language));

        Self {
            node_types,
            metavariable_sort,
            comment_sort,
            statement_sorts,
            language,
            skip_snippet_compilation_sorts,
        }
    }
    pub(crate) fn is_initialized() -> bool {
        LANGUAGE.get().is_some()
    }
}

impl Language for TypeScript {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn skip_snippet_compilation_of_field(&self, sort_id: SortId, field_id: FieldId) -> bool {
        self.skip_snippet_compilation_sorts
            .iter()
            .any(|(s, f)| *s == sort_id && *f == field_id)
    }

    fn alternate_metavariable_kinds(&self) -> &[&'static str] {
        &["template_content", "template_literal_type_content"]
    }

    fn language_name(&self) -> &'static str {
        "TypeScript"
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
            ("GRIT_FN<{ ", " }>();"),
        ]
    }

    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }

    fn is_comment(&self, id: SortId) -> bool {
        id == self.comment_sort
    }

    fn is_comment_wrapper(&self, node: &Node) -> bool {
        node.kind() == "jsx_expression"
            && node.named_child_count() == 1
            && node
                .named_child(0)
                .map(|c| self.is_comment(c.kind_id()))
                .is_some_and(|b| b)
    }

    fn is_statement(&self, id: SortId) -> bool {
        self.statement_sorts.contains(&id)
    }

    // assumes trim doesn't do anything otherwise range is off
    fn comment_text<'a>(
        &self,
        node: &tree_sitter::Node,
        text: &'a str,
    ) -> Option<(Cow<'a, str>, Range)> {
        let text = node.utf8_text(text.as_bytes()).unwrap();
        let text = text.trim();
        let mut range: Range = node.range().into();
        if let Some(text) = text.strip_prefix("//") {
            if !range.adjust_columns(2, 0) {
                return None;
            }
            Some((Cow::Owned(text.to_owned()), range))
        } else if let Some(text) = text.strip_prefix("/*") {
            if !range.adjust_columns(2, -2) {
                return None;
            }
            text.strip_suffix("*/")
                .map(|s| (Cow::Owned(s.to_owned()), range))
        } else {
            None
        }
    }

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }

    fn check_orphaned(&self, n: Node<'_>, src: &str, orphan_ranges: &mut Vec<tree_sitter::Range>) {
        jslike_check_orphaned(n, src, orphan_ranges)
    }

    fn parse_file(
        &self,
        name: &str,
        body: &str,
        logs: &mut marzano_util::analysis_logs::AnalysisLogs,
        new: bool,
    ) -> anyhow::Result<Option<tree_sitter::Tree>> {
        let mut parser = Parser::new().unwrap();
        parser.set_language(self.get_ts_language())?;
        xscript_util::parse_file(self, name, body, logs, new, &mut parser)
    }
}
