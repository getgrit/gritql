use anyhow::{anyhow, Result};
use enum_dispatch::enum_dispatch;
use grit_util::{traverse, Order};
use itertools::{self, Itertools};
use lazy_static::lazy_static;
use marzano_util::{
    analysis_logs::{AnalysisLogBuilder, AnalysisLogs},
    cursor_wrapper::CursorWrapper,
    position::{len, Position, Range},
};
use regex::Regex;
use serde_json::Value;
use std::{borrow::Cow, cmp::max, collections::HashMap};
pub(crate) use tree_sitter::Language as TSLanguage;
use tree_sitter::{Parser, Tree};
// todo decide where this belongs, not good to
// define static variable twice. (also in core/config.rs)
pub static GRIT_METAVARIABLE_PREFIX: &str = "$";
pub type SortId = u16;
pub type FieldId = u16;

#[derive(Debug, Clone)]
pub(crate) struct LeafNormalizer {
    sort: SortId,
    normalizer: fn(&str) -> Option<&str>,
}

impl LeafNormalizer {
    fn normalize<'a>(&self, s: &'a str) -> Option<&'a str> {
        (self.normalizer)(s)
    }

    pub(crate) fn new(sort: SortId, normalizer: fn(&str) -> Option<&str>) -> Self {
        Self { sort, normalizer }
    }

    pub(crate) fn sort(&self) -> SortId {
        self.sort
    }
}

#[derive(Debug, Clone)]
pub struct LeafEquivalenceClass {
    representative: String,
    class: Vec<LeafNormalizer>,
}

impl LeafEquivalenceClass {
    pub fn are_equivalent(&self, sort: SortId, text: &str) -> bool {
        self.class
            .iter()
            .find(|eq| eq.sort == sort)
            .is_some_and(|norm| {
                norm.normalize(text)
                    .is_some_and(|s| s == self.representative)
            })
    }
    pub(crate) fn new(
        representative: &str,
        sort: SortId,
        members: &[LeafNormalizer],
    ) -> Result<Option<Self>, String> {
        if let Some(normalizer) = members.iter().find(|norm| norm.sort == sort) {
            let rep = normalizer.normalize(representative).ok_or_else(|| {
                "Node type matched equivalence class, but failed to normalize".to_owned()
            })?;
            Ok(Some(Self {
                representative: rep.to_owned(),
                class: members.to_owned(),
            }))
        } else {
            Ok(None)
        }
    }
}

lazy_static! {
    pub static ref EXACT_VARIABLE_REGEX: Regex =
        Regex::new(r"^\$([A-Za-z_][A-Za-z0-9_]*)$").unwrap();
    pub static ref EXACT_REPLACED_VARIABLE_REGEX: Regex =
        Regex::new(r"^µ([A-Za-z_][A-Za-z0-9_]*)$").unwrap();
    pub static ref VARIABLE_REGEX: Regex =
        Regex::new(r"\$(\.\.\.|[A-Za-z_][A-Za-z0-9_]*)").unwrap();
    pub static ref REPLACED_VARIABLE_REGEX: Regex =
        Regex::new(r"µ(\.\.\.|[A-Za-z_][A-Za-z0-9_]*)").unwrap();
    pub static ref BRACKET_VAR_REGEX: Regex =
        Regex::new(r"\$\[([A-Za-z_][A-Za-z0-9_]*)\]").unwrap();
}

#[derive(Debug)]
pub struct Field {
    name: String,
    id: FieldId,
    required: bool,
    multiple: bool,
    // for now empty, eventually we'll want to capture possible sort types
    sorts: Vec<SortId>,
}

pub enum GritMetaValue {
    Underscore,
    Dots,
    Variable(String),
}

impl Field {
    pub fn new(name: String, id: FieldId, required: bool, multiple: bool) -> Self {
        Self {
            name,
            id,
            required,
            multiple,
            sorts: vec![],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> FieldId {
        self.id
    }

    pub fn required(&self) -> bool {
        self.required
    }

    pub fn multiple(&self) -> bool {
        self.multiple
    }

    pub fn sorts(&self) -> &Vec<SortId> {
        &self.sorts
    }
}

pub(crate) fn normalize_identity(s: &str) -> Option<&str> {
    Some(s)
}

pub(crate) fn normalize_double_quote_string(s: &str) -> Option<&str> {
    s.strip_prefix('"')?.strip_suffix('"')
}

#[enum_dispatch(TargetLanguage)]
pub trait Language {
    /// tree sitter language to parse the source
    fn get_ts_language(&self) -> &TSLanguage;

    fn language_name(&self) -> &'static str;

    /// ignore trivial tokens in language matching
    /// such as extras in the tree-sitter grammar
    fn skippable_sorts(&self) -> &'static [u16] {
        &[]
    }

    fn skip_snippet_compilation_of_field(&self, _sort_id: SortId, _field_id: FieldId) -> bool {
        false
    }

    fn mandatory_empty_field(&self, _sort_id: SortId, _field_id: FieldId) -> bool {
        false
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)];

    // todo maybe iterate over every node and check for is_missing/is_error?
    // has error only checks for unresolved errors.
    fn parse_snippet_contexts(&self, source: &str) -> Vec<SnippetTree> {
        let source = self.substitute_metavariable_prefix(source);
        self.snippet_context_strings()
            .iter()
            .map(|(pre, post)| self.src_to_snippet(pre, &source, post))
            .filter(|result| {
                !(result.parse_tree.root_node().has_error()
                    || result.parse_tree.root_node().is_error()
                    || result.parse_tree.root_node().is_missing())
            })
            .collect()
    }

    fn alternate_metavariable_kinds(&self) -> &[&'static str] {
        &[]
    }

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

    fn is_comment(&self, _id: SortId) -> bool {
        false
    }

    fn is_comment_wrapper(&self, _node: &tree_sitter::Node) -> bool {
        false
    }

    fn is_statement(&self, _id: SortId) -> bool {
        false
    }

    // assumes trim doesn't do anything otherwise range is off
    fn comment_text<'a>(
        &self,
        node: &tree_sitter::Node,
        text: &'a str,
    ) -> Option<(Cow<'a, str>, Range)> {
        let text = node.utf8_text(text.as_bytes()).unwrap();
        let range = node.range().into();
        // text.trim
        Some((text, range))
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

    fn src_to_snippet(&self, pre: &'static str, source: &str, post: &'static str) -> SnippetTree {
        let mut parser = Parser::new().unwrap();
        parser.set_language(self.get_ts_language()).unwrap();
        let context = format!("{}{}{}", pre, source, post);
        SnippetTree {
            parse_tree: parser.parse(&context, None).unwrap().unwrap(),
            snippet_prefix: pre,
            snippet_postfix: post,
            snippet_start: (len(pre) + len(source) - len(source.trim_start())),
            snippet_end: (len(&context) - len(post) - len(source) + len(source.trim_end())),
            snippet: source.to_owned(),
            context,
            _metavariable_sort: self.metavariable_sort(),
        }
    }

    fn node_types(&self) -> &[Vec<Field>];

    fn metavariable_sort(&self) -> SortId;

    fn check_orphaned(
        &self,
        _n: tree_sitter::Node<'_>,
        _src: &str,
        _orphan_ranges: &mut Vec<tree_sitter::Range>,
    ) {
    }

    fn get_equivalence_class(
        &self,
        _sort: SortId,
        _text: &str,
    ) -> Result<Option<LeafEquivalenceClass>, String> {
        Ok(None)
    }

    fn take_padding(&self, current: char, _next: Option<&char>) -> Option<char> {
        if current.is_whitespace() {
            Some(current)
        } else {
            None
        }
    }

    fn parse_file(
        &self,
        name: &str,
        body: &str,
        logs: &mut AnalysisLogs,
        new: bool,
    ) -> Result<Option<Tree>> {
        default_parse_file(self.get_ts_language(), name, body, logs, new)
    }
}

pub(crate) fn default_parse_file(
    lang: &TSLanguage,
    name: &str,
    body: &str,
    logs: &mut AnalysisLogs,
    new: bool,
) -> Result<Option<Tree>> {
    let mut parser = Parser::new()?;
    parser.set_language(lang)?;
    let tree = parser
        .parse(body, None)?
        .ok_or_else(|| anyhow!("failed to parse tree"))?;
    let mut errors = file_parsing_error(&tree, name, body, new)?;
    logs.append(&mut errors);
    Ok(Some(tree))
}

fn file_parsing_error(
    tree: &Tree,
    file_name: &str,
    body: &str,
    is_new: bool,
) -> Result<AnalysisLogs> {
    let mut errors = vec![];
    let cursor = tree.walk();
    let mut log_builder = AnalysisLogBuilder::default();
    let level: u16 = if is_new { 531 } else { 300 };
    log_builder
        .level(level)
        .engine_id("marzano(0.1)".to_owned())
        .file(file_name.to_owned());

    for n in traverse(CursorWrapper::new(cursor, body), Order::Pre) {
        let node = &n.node;
        if node.is_error() || node.is_missing() {
            let position: Position = node.range().start_point().into();
            let message = format!("Error parsing source code at {}:{} in {}. This may cause otherwise applicable queries to not match.",
                node.range().start_point().row() + 1, node.range().start_point().column() + 1, file_name);
            let log = log_builder
                .clone()
                .message(message)
                .position(position)
                .build()?;
            errors.push(log);
        }
    }
    Ok(errors.into())
}

#[derive(Debug, Clone)]
pub struct SnippetTree {
    pub parse_tree: Tree,
    pub snippet_prefix: &'static str,
    pub snippet_postfix: &'static str,
    pub snippet: String,
    pub context: String,
    pub snippet_start: u32,
    pub snippet_end: u32,
    _metavariable_sort: SortId,
}

#[derive(Debug, Clone)]
pub struct SnippetNode<'a> {
    pub node: tree_sitter::Node<'a>,
    pub context: String,
}

impl<'a> SnippetNode<'a> {
    pub fn new(node: tree_sitter::Node<'a>, context: String) -> Self {
        Self { node, context }
    }
}

impl SnippetTree {
    // pub for testing
    pub fn snippet_nodes_from_index(&self) -> Option<SnippetNode> {
        let mut snippet_root = self.parse_tree.root_node();
        let mut cursor = self.parse_tree.walk();
        let mut id = snippet_root.id();

        // find the the most senior node with the same index as the snippet
        while snippet_root.start_byte() < self.snippet_start
            || snippet_root.end_byte() > self.snippet_end
        {
            if snippet_root.named_child_count() == 0 {
                if snippet_root
                    .utf8_text(self.context.as_bytes())
                    .unwrap()
                    .trim()
                    == self.snippet.trim()
                {
                    return Some(SnippetNode::new(snippet_root, self.context.clone()));
                } else {
                    return None;
                }
            }
            for child in snippet_root.named_children(&mut cursor) {
                if child.start_byte() <= self.snippet_start && child.end_byte() >= self.snippet_end
                {
                    snippet_root = child;
                    break;
                }
            }
            // sanity check to avoid infinite loop
            if snippet_root.id() == id {
                if snippet_root
                    .utf8_text(self.context.as_bytes())
                    .unwrap()
                    .trim() != self.snippet.trim()
                {
                    return None;
                }
                break;
            }
            id = snippet_root.id();
        }
        // in order to handle white space and other superfluos
        // stuff in the snippet we assume the root
        // is correct as long as it's the largest node within
        // the snippet length. Maybe this is too permissive?
        let mut nodes = vec![];
        let root_start = snippet_root.start_byte();
        let root_end = snippet_root.end_byte();
        if root_start > self.snippet_start || root_end < self.snippet_end {
            return None;
        }
        while snippet_root.start_byte() == root_start && snippet_root.end_byte() == root_end {
            nodes.push(SnippetNode::new(snippet_root.clone(), self.context.clone()));
            if let Some(child) = snippet_root.named_child(0) {
                snippet_root = child
            } else {
                break;
            }
        }
        nodes.last().cloned()
    }
}

pub fn nodes_from_indices(indices: &[SnippetTree]) -> Vec<SnippetNode> {
    indices
        .iter()
        .flat_map(|s| s.snippet_nodes_from_index())
        .unique_by(|n| n.node.kind_id())
        .collect()
}

// todo
// also extract multiple and required?
pub fn fields_for_nodes(language: &TSLanguage, types: &str) -> Vec<Vec<Field>> {
    let mut fields = HashMap::new();
    let node_types: Value = serde_json::from_str(types).unwrap();
    let node_types = node_types.as_array().unwrap();
    let mut max_kind = 0;
    for node in node_types {
        let node_type = node["type"].as_str().unwrap();
        let node_id = language.id_for_node_kind(node_type, node["named"].as_bool().unwrap());
        if node_id == 0 {
            // typescript has node_types for tsx elements that are inherited from the
            // shared grammar definition with tsx, but does not have ids for them
            // or the the contianed fields.
            continue;
        }
        max_kind = max(max_kind, node_id);
        let mut field_ids = vec![];
        if let Some(fields) = node.get("fields") {
            for (field_name, value) in fields.as_object().unwrap() {
                let field_id = language.field_id_for_name(field_name).unwrap();
                let required = value.get("required").unwrap().as_bool().unwrap();
                let multiple = value.get("multiple").unwrap().as_bool().unwrap();
                let field = Field::new(field_name.to_owned(), field_id, required, multiple);
                field_ids.push(field);
            }
        }
        fields.insert(node_id, field_ids);
    }
    (0..=max_kind)
        .map(|kind| fields.remove(&kind).unwrap_or_default())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::nodes_from_indices;
    use crate::{language::Language, tsx::Tsx};
    use tree_sitter::Parser;
    use trim_margin::MarginTrimmable;

    #[test]
    fn extract_nodes_from_snippet() {
        let pre = "class Pattern1a {\n  pattern0(param1) {";
        let post = "\n  }\n}";
        let snippet = "\n   foo('moment')  \n".to_string();
        let lang = Tsx::new(None);
        let mut parser = Parser::new().unwrap();
        parser.set_language(lang.get_ts_language()).unwrap();
        let snippet_index = lang.src_to_snippet(pre, &snippet, post);
        let node = snippet_index.snippet_nodes_from_index();
        assert!(node.is_some())
    }

    #[test]
    fn snippet_to_nodes() {
        let snippet = "foo('bar')";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn snippet_to_nodes_meta() {
        let snippet = "console.log($foo)";
        let lang = Tsx::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn test_substitute_variable() {
        let snippet = "$foo$('$bar')";
        let lang = Tsx::new(None);
        let subbed = lang.substitute_metavariable_prefix(snippet);
        assert_eq!(subbed, "µfoo$('µbar')");
    }

    #[test]
    fn test_substitute_variable_template() {
        let snippet = r#"
        |language js
        |
        |js"styled`
        |text-decoration: ${$_};
        |&:hover {
        |  text-decoration: underline;
        |  color: ${$foo};
        |}`"
        |"#
        .trim_margin()
        .unwrap();
        let lang = Tsx::new(None);
        let subbed = lang.substitute_metavariable_prefix(&snippet);
        let expected = r#"
        |language js
        |
        |js"styled`
        |text-decoration: ${µ_};
        |&:hover {
        |  text-decoration: underline;
        |  color: ${µfoo};
        |}`"
        |"#
        .trim_margin()
        .unwrap();
        assert_eq!(subbed, expected);
    }
}
