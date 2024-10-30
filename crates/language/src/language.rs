use anyhow::{Context, Result};
use enum_dispatch::enum_dispatch;
use grit_util::{
    traverse, AnalysisLogBuilder, AnalysisLogs, Ast, AstNode, CodeRange, EffectRange, FileOrigin,
    Language, Order, Parser, SnippetTree,
};
use itertools::Itertools;
use marzano_util::{cursor_wrapper::CursorWrapper, node_with_source::NodeWithSource};
use serde_json::Value;
use std::{borrow::Cow, cmp::max, collections::HashMap, path::Path};
pub(crate) use tree_sitter::{Language as TSLanguage, Parser as TSParser, Tree as TSTree};

use crate::sourcemap::EmbeddedSourceMap;

pub type SortId = u16;
pub type FieldId = u16;

#[derive(Debug, Clone, Copy)]
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
    pub fn contains(&self, sort: SortId) -> bool {
        self.class.iter().any(|c| c.sort == sort)
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

pub(crate) fn normalize_single_quote_string(s: &str) -> Option<&str> {
    s.strip_prefix('\'')?.strip_suffix('\'')
}

pub(crate) fn kind_and_field_id_for_field_map(
    lang: &TSLanguage,
    names: Vec<(&str, &str, FieldExpectationCondition)>,
) -> Vec<FieldExpectation> {
    names
        .into_iter()
        .map(|(kind, field, val)| {
            (
                lang.id_for_node_kind(kind, true),
                lang.field_id_for_name(field)
                    .context(format!("Field {} not found", field))
                    .unwrap(),
                val,
            )
        })
        .collect()
}

#[derive(Debug, Clone)]
pub(crate) enum FieldExpectationCondition {
    Always,
    OnlyIf(Vec<&'static str>),
}

/// Field expectation is a tuple of (sort_id, field_id, expected_values)
///
/// If the expected_values is None, the field will be disregarded entirely no matter what the field node is.
/// Otherwise, the field will be disregarded only if the field node's text matches one of the expected values.
/// An empty field will have an empty string as its text.
pub(crate) type FieldExpectation = (u16, u16, FieldExpectationCondition);

/// Helper utility for implementing `is_disregarded_snippet_field`.
pub(crate) fn check_disregarded_field_map(
    field_map: &[FieldExpectation],
    sort_id: SortId,
    field_id: crate::language::FieldId,
    field_node: &Option<NodeWithSource>,
) -> bool {
    field_map.iter().any(|(s, f, expected_values)| {
        if *s != sort_id || *f != field_id {
            return false;
        }
        match expected_values {
            FieldExpectationCondition::OnlyIf(expected_values) => {
                let text = field_node
                    .as_ref()
                    .map(|f| f.text().unwrap_or(Cow::Borrowed("")))
                    .unwrap_or(Cow::Borrowed(""));
                let text_ref = text.as_ref();
                expected_values.iter().any(|n| n == &text_ref)
            }
            // Always match
            FieldExpectationCondition::Always => true,
        }
    })
}

pub trait NodeTypes {
    fn node_types(&self) -> &[Vec<Field>];
}

#[derive(Clone, Debug)]
pub struct Tree {
    tree: TSTree,
    /// The pure source code of the tree, which does not necessarily match the original file
    pub source: String,
    /// A source map, if needed
    pub source_map: Option<EmbeddedSourceMap>,
}

impl Tree {
    pub fn new(tree: TSTree, source: impl Into<String>) -> Self {
        Self {
            tree,
            source: source.into(),
            source_map: None,
        }
    }

    pub fn outer_source(&self) -> &str {
        match &self.source_map {
            Some(map) => &map.outer_source,
            None => &self.source,
        }
    }
}

impl Ast for Tree {
    type Node<'a> = NodeWithSource<'a>;

    fn root_node(&self) -> Self::Node<'_> {
        NodeWithSource::new(self.tree.root_node(), &self.source)
    }

    fn source(&self) -> Cow<str> {
        Cow::Borrowed(&self.source)
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        // Equal sources must parse to the same trees.
        self.source == other.source
    }
}

pub struct MarzanoParser {
    pub(crate) parser: TSParser,
}

impl MarzanoParser {
    pub fn new<'a>(lang: &impl MarzanoLanguage<'a>) -> Self {
        let mut parser = TSParser::new().unwrap();
        parser
            .set_language(lang.get_ts_language())
            .expect("failed to set TreeSitter language");
        Self { parser }
    }
}

impl Parser for MarzanoParser {
    type Tree = Tree;

    fn parse_file(
        &mut self,
        body: &str,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
        old_tree: FileOrigin<'_, Tree>,
    ) -> Option<Tree> {
        let tree = self.parser.parse(body, None).ok()??;

        if let Some(path) = path {
            let mut errors = file_parsing_error(&tree, path, body, !old_tree.is_fresh()).ok()?;
            logs.append(&mut errors);
        }

        Some(Tree::new(tree, body))
    }

    fn parse_snippet(
        &mut self,
        prefix: &'static str,
        source: &str,
        postfix: &'static str,
    ) -> SnippetTree<Tree> {
        let context = format!("{prefix}{source}{postfix}");

        let len = if cfg!(target_arch = "wasm32") {
            |src: &str| src.chars().count() as u32
        } else {
            |src: &str| src.len() as u32
        };

        let tree = self.parser.parse(&context, None).unwrap().unwrap();

        SnippetTree {
            tree: Tree::new(tree, context),
            source: source.to_owned(),
            prefix,
            postfix,
            snippet_start: (len(prefix) + len(source) - len(source.trim_start())),
            snippet_end: (len(prefix) + len(source.trim_end())),
        }
    }
}

#[enum_dispatch]
pub trait MarzanoLanguage<'a>: Language<Node<'a> = NodeWithSource<'a>> + NodeTypes {
    /// tree sitter language to parse the source
    fn get_ts_language(&self) -> &TSLanguage;

    fn get_parser(&self) -> Box<dyn Parser<Tree = Tree>> {
        Box::new(MarzanoParser::new(self))
    }

    fn parse_snippet_contexts(&self, source: &str) -> Vec<SnippetTree<Tree>> {
        let source = self.substitute_metavariable_prefix(source);
        self.snippet_context_strings()
            .iter()
            .map(|(pre, post)| self.get_parser().parse_snippet(pre, &source, post))
            .filter(|result| {
                let root_node = &result.tree.root_node().node;
                !(root_node.has_error() || root_node.is_error() || root_node.is_missing())
            })
            .collect()
    }

    fn align_padding<'b>(
        &self,
        node: &NodeWithSource<'b>,
        range: &CodeRange,
        skip_ranges: &[CodeRange],
        new_padding: Option<usize>,
        offset: usize,
        substitutions: &mut [(EffectRange, String)],
    ) -> std::borrow::Cow<'b, str> {
        let src = node.source;
        let Some(new_padding) = new_padding else {
            return src[range.start as usize..range.end as usize].into();
        };

        let newline_index = src[0..range.start as usize].rfind('\n');
        let pad_strip_amount = if let Some(index) = newline_index {
            src[index..range.start as usize]
                .chars()
                .take_while(|c| c.is_whitespace())
                .count()
                - 1
        } else {
            0
        };

        let mut result = String::new();
        let snippet = &src[range.start as usize..range.end as usize];
        let mut lines = snippet.split('\n');

        // assumes codebase uses spaces for indentation
        let delta: isize = (new_padding as isize) - (pad_strip_amount as isize);
        let padding = " ".repeat(pad_strip_amount);
        let new_padding = " ".repeat(new_padding);
        result.push_str(lines.next().unwrap_or_default());
        for line in lines {
            result.push('\n');

            // Safety: Safe because lines are slices of `snippet`, which is a slice of `src`.
            let index = unsafe { get_slice_byte_offset(src, line) };

            if !skip_ranges.iter().any(|r| r.contains(index)) {
                if line.trim().is_empty() {
                    adjust_ranges(substitutions, offset + result.len(), -(line.len() as isize));
                    continue;
                }

                adjust_ranges(substitutions, offset + result.len(), delta);
                if let Some(stripped_line) = line.strip_prefix(&padding) {
                    result.push_str(&new_padding);
                    result.push_str(stripped_line);
                } else {
                    result.push_str(line);
                }
            } else {
                result.push_str(line)
            }
        }

        for (_, snippet) in substitutions.iter_mut() {
            match MarzanoLanguage::pad_snippet(self, snippet, &new_padding) {
                std::borrow::Cow::Owned(padded) => *snippet = padded,
                std::borrow::Cow::Borrowed(_) => {
                    // a borrowed result implies the padding didn't change
                }
            }
        }

        result.into()
    }

    // in languages we pad such as python or yaml there are
    // some kinds of nodes we don't want to pad, such as python strings.
    // this function identifies those nodes.
    #[allow(unused_variables)]
    fn should_skip_padding(&self, node: &Self::Node<'a>) -> bool {
        false
    }

    fn get_skip_padding_ranges(&self, node: &Self::Node<'a>) -> Vec<CodeRange> {
        let mut ranges = Vec::new();
        for n in traverse(node.walk(), Order::Pre) {
            if self.should_skip_padding(&n) {
                ranges.push(n.code_range())
            }
        }
        ranges
    }

    #[allow(unused_variables)]
    fn get_skip_padding_ranges_for_snippet(&self, snippet: &str) -> Vec<CodeRange> {
        Vec::new()
    }

    /// Ordinarily, we want to match on all possible fields, including the absence of nodes within a field.
    /// e.g., `my_function()` should not match `my_function(arg)`.
    ///
    /// However, some fields are trivial or not expected to be part of the snippet, and should be disregarded.
    /// For example, in JavaScript, we want to match both `function name() {}` and `async function name() {}` with the same snippet.
    ///
    /// You can still match on the presence/absence of the field in the snippet by including a metavariable and checking its value.
    /// For example, in JavaScript:
    /// ```grit
    /// `$async func name(args)` where $async <: .
    /// ```
    ///
    /// This method allows you to specify fields that should be (conditionally) disregarded in snippets.
    /// The actual value of the field from the snippet, if any, is passed in as the third argument.
    ///
    /// Note that if a field is always disregarded, you can still switch to ast_node syntax to match on these fields.
    /// For example, in react_to_hooks we match on `arrow_function` and capture `$parenthesis` for inspection.
    ///
    /// ```grit
    /// arrow_function(parameters=$props, $body, $parenthesis) where {
    ///     $props <: contains or { `props`, `inputProps` },
    ///     $body <: not contains `props`,
    ///    if ($parenthesis <: .) {
    ///         $props => `()`
    ///     } else {
    ///         $props => .
    ///     }
    /// }
    /// ```
    fn is_disregarded_snippet_field(
        &self,
        _sort_id: SortId,
        _field_id: FieldId,
        _node: &Option<NodeWithSource<'_>>,
    ) -> bool {
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
        node.node.is_named() && node.node.kind_id() == self.metavariable_sort()
    }

    fn get_equivalence_class(
        &self,
        _sort: SortId,
        _text: &str,
    ) -> Result<Option<LeafEquivalenceClass>, String> {
        Ok(None)
    }

    fn pad_snippet<'b>(&self, snippet: &'b str, padding: &str) -> Cow<'b, str> {
        if padding.is_empty() {
            return snippet.into();
        }

        let mut lines = snippet.split('\n');
        let mut result = Cow::Borrowed(lines.next().unwrap_or_default());

        // Add the rest of lines in the snippet with padding
        let skip_ranges = self.get_skip_padding_ranges_for_snippet(snippet);
        for line in lines {
            // Safety: This is safe because `line` is a slice of `snippet`.
            let index = unsafe { get_slice_byte_offset(snippet, line) };

            let result = result.to_mut();
            result.push('\n');
            if !skip_ranges.iter().any(|r| r.contains(index)) {
                result.push_str(padding)
            }
            result.push_str(line);
        }
        result
    }
}

fn adjust_ranges(substitutions: &mut [(EffectRange, String)], index: usize, delta: isize) {
    for (EffectRange { range, .. }, _) in substitutions.iter_mut() {
        if range.start >= index {
            range.start = (range.start as isize + delta) as usize;
        }
        if range.end >= index {
            range.end = (range.end as isize + delta) as usize;
        }
    }
}

fn file_parsing_error(
    tree: &TSTree,
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

/// Returns the byte offset of `sub` relative to `sup`.
///
/// This function assumes `sub` starts at or after `sup`, since it cannot return
/// negative offsets.
///
/// Safety: `sup` and `sub` must be slices of the same string.
unsafe fn get_slice_byte_offset(sup: &str, sub: &str) -> u32 {
    sub.as_ptr().byte_offset_from(sup.as_ptr()).unsigned_abs() as u32
}

pub fn nodes_from_indices(indices: &[SnippetTree<Tree>]) -> Vec<NodeWithSource> {
    indices
        .iter()
        .flat_map(snippet_nodes_from_index)
        .unique_by(|n| n.node.kind_id())
        .collect()
}

fn snippet_nodes_from_index(snippet: &SnippetTree<Tree>) -> Option<NodeWithSource> {
    let mut snippet_root = snippet.tree.root_node();
    if snippet_root.node.is_missing() {
        return None;
    }

    let mut id = snippet_root.node.id();

    // find the the most senior node with the same index as the snippet
    while snippet_root.node.start_byte() < snippet.snippet_start
        || snippet_root.node.end_byte() > snippet.snippet_end
    {
        if snippet_root.named_children().count() == 0 {
            if snippet_root.text().unwrap().trim() == snippet.source.trim() {
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
            if snippet_root.text().unwrap().trim() != snippet.source.trim() {
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
    use grit_util::{Language, Parser};
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
