use anyhow::{anyhow, Result};
use enum_dispatch::enum_dispatch;
use grit_util::{traverse, AnalysisLogBuilder, AnalysisLogs, AstNode, Language, Order};
use itertools::Itertools;
use marzano_util::{cursor_wrapper::CursorWrapper, node_with_source::NodeWithSource};
use serde_json::Value;
use std::{cmp::max, collections::HashMap, path::Path};
pub(crate) use tree_sitter::Language as TSLanguage;
use tree_sitter::{Parser, Tree};

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

#[derive(Debug)]
pub struct Field {
    name: String,
    id: FieldId,
    required: bool,
    multiple: bool,
    // for now empty, eventually we'll want to capture possible sort types
    sorts: Vec<SortId>,
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

pub(crate) fn kind_and_field_id_for_names(
    lang: &TSLanguage,
    names: Vec<(&str, &str)>,
) -> Vec<(u16, u16)> {
    names
        .iter()
        .map(|(kind, field)| {
            (
                lang.id_for_node_kind(kind, true),
                lang.field_id_for_name(field).unwrap(),
            )
        })
        .collect()
}

pub trait NodeTypes {
    fn node_types(&self) -> &[Vec<Field>];
}

pub struct MarzanoParser {
    pub(crate) parser: Parser,
}

impl MarzanoParser {
    pub fn new<'a>(lang: &impl MarzanoLanguage<'a>) -> Self {
        let mut parser = Parser::new().unwrap();
        parser
            .set_language(lang.get_ts_language())
            .expect("failed to set TreeSitter language");
        Self { parser }
    }

    pub fn parse_file(
        &mut self,
        path: &Path,
        body: &str,
        logs: &mut AnalysisLogs,
        new: bool,
    ) -> Result<Option<Tree>> {
        let tree = self
            .parser
            .parse(body, None)?
            .ok_or_else(|| anyhow!("failed to parse tree"))?;
        let mut errors = file_parsing_error(&tree, path, body, new)?;
        logs.append(&mut errors);
        Ok(Some(tree))
    }

    pub fn parse_snippet(
        &mut self,
        pre: &'static str,
        source: &str,
        post: &'static str,
    ) -> SnippetTree {
        let context = format!("{pre}{source}{post}");

        let len = if cfg!(target_arch = "wasm32") {
            |src: &str| src.chars().count() as u32
        } else {
            |src: &str| src.len() as u32
        };

        SnippetTree {
            parse_tree: self.parser.parse(&context, None).unwrap().unwrap(),
            snippet_prefix: pre,
            snippet_postfix: post,
            snippet_start: (len(pre) + len(source) - len(source.trim_start())),
            snippet_end: (len(pre) + len(source.trim_end())),
            snippet: source.to_owned(),
            context,
        }
    }
}

#[enum_dispatch]
pub trait MarzanoLanguage<'a>: Language<Node<'a> = NodeWithSource<'a>> + NodeTypes {
    /// tree sitter language to parse the source
    fn get_ts_language(&self) -> &TSLanguage;

    fn parse_file(
        &self,
        name: &Path,
        body: &str,
        logs: &mut AnalysisLogs,
        new: bool,
    ) -> Result<Option<Tree>> {
        MarzanoParser::new(self).parse_file(name, body, logs, new)
    }

    fn parse_snippet(&self, pre: &'static str, snippet: &str, post: &'static str) -> SnippetTree {
        MarzanoParser::new(self).parse_snippet(pre, snippet, post)
    }

    fn parse_snippet_contexts(&self, source: &str) -> Vec<SnippetTree> {
        let source = self.substitute_metavariable_prefix(source);
        self.snippet_context_strings()
            .iter()
            .map(|(pre, post)| self.parse_snippet(pre, &source, post))
            .filter(|result| {
                !(result.parse_tree.root_node().has_error()
                    || result.parse_tree.root_node().is_error()
                    || result.parse_tree.root_node().is_missing())
            })
            .collect()
    }

    fn skip_snippet_compilation_of_field(&self, _sort_id: SortId, _field_id: FieldId) -> bool {
        false
    }

    /// get a list fields which when not present in a snippet will not be matched against.
    /// by default empty fields will be require the target field to also be empty to match, e.g.,
    /// `function() { $body }` will only match functions with no arguments.
    fn optional_empty_field_compilation(&self, _sort_id: SortId, _field_id: FieldId) -> bool {
        false
    }

    fn is_comment_sort(&self, sort: SortId) -> bool;

    // Same as `Language::is_comment()`.
    //
    // Distinct from `is_comment_sort()` because sometimes a node is a comment
    // but doesn't have a comment sort. For example when parsing JavaScript,
    // comments embedded in JSX dont have the comment sort.
    fn is_comment_node(&self, node: &NodeWithSource<'_>) -> bool {
        self.is_comment_sort(node.node.kind_id())
    }

    fn metavariable_sort(&self) -> SortId;

    fn is_metavariable_node(&self, node: &NodeWithSource<'_>) -> bool {
        node.node.is_named()
            && (node.node.kind_id() == self.metavariable_sort()
                || (self
                    .alternate_metavariable_kinds()
                    .contains(&node.node.kind().as_ref())
                    && node
                        .text()
                        .is_ok_and(|t| self.exact_replaced_variable_regex().is_match(&t))))
    }

    fn get_equivalence_class(
        &self,
        _sort: SortId,
        _text: &str,
    ) -> Result<Option<LeafEquivalenceClass>, String> {
        Ok(None)
    }
}

fn file_parsing_error(
    tree: &Tree,
    file_name: &Path,
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
        if n.node.is_error() || n.node.is_missing() {
            let position = n.range().start;
            let message = format!(
                "Error parsing source code at {position} in {}. This may cause \
                otherwise applicable queries to not match.",
                file_name.display()
            );
            if let Ok(log) = log_builder
                .clone()
                .message(message)
                .position(position)
                .build()
            {
                errors.push(log);
            }
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
}

pub fn nodes_from_indices(indices: &[SnippetTree]) -> Vec<NodeWithSource> {
    indices
        .iter()
        .flat_map(snippet_nodes_from_index)
        .unique_by(|n| n.node.kind_id())
        .collect()
}

fn snippet_nodes_from_index(snippet: &SnippetTree) -> Option<NodeWithSource> {
    let snippet_root = snippet.parse_tree.root_node();
    if snippet_root.is_missing() {
        return None;
    }

    let mut snippet_root = NodeWithSource::new(snippet_root, &snippet.snippet);
    let mut id = snippet_root.node.id();

    // find the the most senior node with the same index as the snippet
    while snippet_root.node.start_byte() < snippet.snippet_start
        || snippet_root.node.end_byte() > snippet.snippet_end
    {
        if snippet_root.named_children().count() == 0 {
            if snippet_root.text().unwrap().trim() == snippet.snippet.trim() {
                return Some(snippet_root);
            } else {
                return None;
            }
        }
        for child in snippet_root.named_children() {
            if child.node.start_byte() <= snippet.snippet_start
                && child.node.end_byte() >= snippet.snippet_end
            {
                snippet_root = child;
                break;
            }
        }
        // sanity check to avoid infinite loop
        if snippet_root.node.id() == id {
            if snippet_root.text().unwrap().trim() != snippet.snippet.trim() {
                return None;
            }
            break;
        }
        id = snippet_root.node.id();
    }

    // in order to handle white space and other superfluos
    // stuff in the snippet we assume the root
    // is correct as long as it's the largest node within
    // the snippet length. Maybe this is too permissive?
    let mut nodes = vec![];
    let root_start = snippet_root.node.start_byte();
    let root_end = snippet_root.node.end_byte();
    if root_start > snippet.snippet_start || root_end < snippet.snippet_end {
        return None;
    }
    while snippet_root.node.start_byte() == root_start && snippet_root.node.end_byte() == root_end {
        let first_child = snippet_root.named_children().next();
        nodes.push(snippet_root);
        if let Some(child) = first_child {
            snippet_root = child
        } else {
            break;
        }
    }
    nodes.last().cloned()
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
    use super::{nodes_from_indices, snippet_nodes_from_index};
    use crate::{
        language::{MarzanoLanguage, MarzanoParser},
        tsx::Tsx,
    };
    use grit_util::Language;
    use trim_margin::MarginTrimmable;

    #[test]
    fn extract_nodes_from_snippet() {
        let pre = "class Pattern1a {\n  pattern0(param1) {";
        let post = "\n  }\n}";
        let snippet = "\n   foo('moment')  \n".to_string();
        let lang = Tsx::new(None);
        let snippet_index = MarzanoParser::new(&lang).parse_snippet(pre, &snippet, post);
        let node = snippet_nodes_from_index(&snippet_index);
        assert!(node.is_some())
    }

    #[test]
    fn snippet_to_nodes() {
        let snippet = "foo('bar')";
        let snippets = Tsx::new(None).parse_snippet_contexts(snippet);
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
