use crate::binding::Binding;
use crate::constant::Constant;
use crate::context::QueryContext;
use crate::equivalence::are_equivalent;
use crate::inline_snippets::inline_sorted_snippets_with_offset;
use crate::pattern::{
    resolved_pattern::ResolvedPattern,
    state::{get_top_level_effects, FileRegistry},
};
use crate::problem::{Effect, EffectKind, MarzanoQueryContext};
use crate::smart_insert::calculate_padding;
use crate::suppress::is_suppress_comment;
use anyhow::{anyhow, Result};
use grit_util::{AnalysisLogBuilder, AnalysisLogs, AstNode, CodeRange, Language, Position, Range};
use itertools::{EitherOrBoth, Itertools};
use marzano_language::language::{FieldId, MarzanoLanguage};
use marzano_language::target_language::TargetLanguage;
use marzano_util::node_with_source::NodeWithSource;
use std::ops::Range as StdRange;
use std::path::Path;
use std::{borrow::Cow, collections::HashMap};

#[derive(Debug, Clone)]
// &str points to the file source
pub enum MarzanoBinding<'a> {
    // used by slices that don't correspond to a node
    // currently only comment content.
    String(&'a str, Range),
    FileName(&'a Path),
    Node(NodeWithSource<'a>),
    // tree-sitter lists ("multiple" fields of nodes) do not have a unique identity
    // so we represent them by the parent node and a field id
    List(NodeWithSource<'a>, FieldId),
    Empty(NodeWithSource<'a>, FieldId),
    ConstantRef(&'a Constant),
}

impl PartialEq for MarzanoBinding<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty(_, _), Self::Empty(_, _)) => true,
            (Self::Node(n1), Self::Node(n2)) => n1.text() == n2.text(),
            (Self::String(src1, r1), Self::String(src2, r2)) => {
                src1[r1.start_byte as usize..r1.end_byte as usize]
                    == src2[r2.start_byte as usize..r2.end_byte as usize]
            }
            (Self::List(n1, f1), Self::List(n2, f2)) => n1 == n2 && f1 == f2,
            (Self::ConstantRef(c1), Self::ConstantRef(c2)) => c1 == c2,
            _ => false,
        }
    }
}

pub(crate) fn pad_snippet(padding: &str, snippet: &str, lang: &impl Language) -> Result<String> {
    let mut lines = snippet.split('\n');
    let mut result = lines.next().unwrap_or_default().to_string();

    // Add the rest of lines in the snippet with padding
    let skip_ranges = lang.get_skip_padding_ranges_for_snippet(snippet);
    for line in lines {
        let index = get_slice_byte_offset(snippet, line);
        if !is_index_in_ranges(index, &skip_ranges) {
            result.push_str(&format!("\n{}{}", &padding, line))
        } else {
            result.push_str(&format!("\n{}", line))
        }
    }
    Ok(result)
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

pub(crate) fn is_index_in_ranges(index: u32, skip_ranges: &[CodeRange]) -> bool {
    skip_ranges
        .iter()
        .any(|r| r.start <= index && index < r.end)
}

// safety ensure that sup and sub are slices of the same string.
fn get_slice_byte_offset(sup: &str, sub: &str) -> u32 {
    unsafe { sub.as_ptr().byte_offset_from(sup.as_ptr()) }.unsigned_abs() as u32
}

// in multiline snippets, remove padding from every line equal to the padding of the first line,
// such that the first line is left-aligned.
fn adjust_padding<'a>(
    src: &'a str,
    range: &CodeRange,
    skip_ranges: &[CodeRange],
    new_padding: Option<usize>,
    offset: usize,
    substitutions: &mut [(EffectRange, String)],
    lang: &impl Language,
) -> Result<Cow<'a, str>> {
    if let Some(new_padding) = new_padding {
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
            let index = get_slice_byte_offset(src, line);
            if !is_index_in_ranges(index, skip_ranges) {
                if line.trim().is_empty() {
                    adjust_ranges(substitutions, offset + result.len(), -(line.len() as isize));
                    continue;
                }
                adjust_ranges(substitutions, offset + result.len(), delta);
                let line = line.strip_prefix(&padding).ok_or_else(|| {
                anyhow!(
                    "expected line \n{}\n to start with {} spaces, code is either not indented with spaces, or does not consistently indent code blocks",
                    line,
                    pad_strip_amount
                )
            })?;
                result.push_str(&new_padding);
                result.push_str(line);
            } else {
                result.push_str(line)
            }
        }
        for (_, snippet) in substitutions.iter_mut() {
            *snippet = pad_snippet(&new_padding, snippet, lang)?;
        }
        Ok(result.into())
    } else {
        Ok(src[range.start as usize..range.end as usize].into())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EffectRange {
    pub(crate) kind: EffectKind,
    pub(crate) range: StdRange<usize>,
}

impl EffectRange {
    pub(crate) fn new(kind: EffectKind, range: StdRange<usize>) -> Self {
        Self { kind, range }
    }

    pub(crate) fn start(&self) -> usize {
        self.range.start
    }

    // The range which is actually edited by this effect
    pub(crate) fn effective_range(&self) -> StdRange<usize> {
        match self.kind {
            EffectKind::Rewrite => self.range.clone(),
            EffectKind::Insert => self.range.end..self.range.end,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn linearize_binding<'a, Q: QueryContext>(
    language: &Q::Language<'a>,
    effects: &[Effect<'a, Q>],
    files: &FileRegistry<'a>,
    memo: &mut HashMap<CodeRange, Option<String>>,
    source: Q::Node<'a>,
    range: CodeRange,
    distributed_indent: Option<usize>,
    logs: &mut AnalysisLogs,
) -> Result<(Cow<'a, str>, Vec<StdRange<usize>>)> {
    let effects1 = get_top_level_effects(effects, memo, &range, language, logs)?;

    let effects1 = effects1
        .into_iter()
        .map(|effect| {
            let binding = effect.binding;
            let binding_range = Binding::code_range(&binding, language);
            if let (Some(src), Some(range)) = (binding.source(), binding_range.as_ref()) {
                match effect.kind {
                    EffectKind::Rewrite => {
                        if let Some(o) = memo.get(range) {
                            if let Some(s) = o {
                                return Ok((binding, s.to_owned().into(), effect.kind));
                            } else {
                                let skip_padding_ranges = binding
                                    .as_node()
                                    .map(|n| language.get_skip_padding_ranges(&n))
                                    .unwrap_or_default();

                                return Ok((
                                    binding,
                                    adjust_padding(
                                        src,
                                        range,
                                        &skip_padding_ranges,
                                        distributed_indent,
                                        0,
                                        &mut [],
                                        language,
                                    )?,
                                    effect.kind,
                                ));
                            }
                        } else {
                            memo.insert(range.clone(), None);
                        }
                    }
                    EffectKind::Insert => {}
                }
            } else {
                binding.log_empty_field_rewrite_error(language, logs)?;
            }
            let res = effect.pattern.linearized_text(
                language,
                effects,
                files,
                memo,
                distributed_indent.is_some(),
                logs,
            )?;
            if let Some(range) = binding_range {
                if matches!(effect.kind, EffectKind::Rewrite) {
                    memo.insert(range, Some(res.to_string()));
                }
            }
            Ok((binding, res, effect.kind))
        })
        .collect::<Result<Vec<_>>>()?;

    let mut replacements: Vec<(EffectRange, String)> = effects1
        .iter()
        .map(|(b, s, k)| {
            let range = b
                .position(language)
                .ok_or_else(|| anyhow!("binding has no position"))?;
            match k {
                EffectKind::Insert => Ok((
                    EffectRange::new(
                        EffectKind::Insert,
                        range.start_byte as usize..range.end_byte as usize,
                    ),
                    s.to_string(),
                )),
                EffectKind::Rewrite => Ok((
                    EffectRange::new(
                        EffectKind::Rewrite,
                        range.start_byte as usize..range.end_byte as usize,
                    ),
                    s.to_string(),
                )),
            }
        })
        .collect::<Result<Vec<_>>>()?;

    let skip_padding_ranges = language.get_skip_padding_ranges(&source);
    // we need to update the ranges of the replacements to account for padding discrepency
    let adjusted_source = adjust_padding(
        source.full_source(),
        &range,
        &skip_padding_ranges,
        distributed_indent,
        range.start as usize,
        &mut replacements,
        language,
    )?;
    let (res, offset) = inline_sorted_snippets_with_offset(
        language,
        adjusted_source.to_string(),
        range.start as usize,
        &mut replacements,
        distributed_indent.is_some(),
    )?;
    memo.insert(range, Some(res.clone()));
    Ok((res.into(), offset))
}

impl<'a> Binding<'a, MarzanoQueryContext> for MarzanoBinding<'a> {
    fn from_constant(constant: &'a Constant) -> Self {
        Self::ConstantRef(constant)
    }

    fn from_node(node: NodeWithSource<'a>) -> Self {
        Self::Node(node)
    }

    fn from_path(path: &'a Path) -> Self {
        Self::FileName(path)
    }

    fn from_range(range: Range, source: &'a str) -> Self {
        Self::String(source, range)
    }

    /// Returns the only node bound by this binding.
    ///
    /// This includes list bindings that only match a single child.
    ///
    /// Returns `None` if the binding has no associated node, or if there is
    /// more than one associated node.
    fn singleton(&self) -> Option<NodeWithSource<'a>> {
        match self {
            Self::Node(node) => Some(node.clone()),
            Self::List(parent_node, field_id) => {
                let mut children = parent_node.named_children_by_field_id(*field_id);
                children.next().filter(|_| children.next().is_none())
            }
            Self::String(..) | Self::FileName(..) | Self::Empty(..) | Self::ConstantRef(..) => None,
        }
    }

    fn get_sexp(&self) -> Option<String> {
        match self {
            Self::Node(node) => Some(node.node.to_sexp().to_string()),
            Self::List(parent_node, field_id) => {
                let mut children = parent_node.children_by_field_id(*field_id);
                let mut result = String::new();
                if let Some(node) = children.next() {
                    result.push_str(&node.node.to_sexp());
                    for node in children {
                        result.push_str(",\n");
                        result.push_str(&node.node.to_sexp());
                    }
                }
                Some(result)
            }
            Self::String(..) | Self::FileName(_) | Self::Empty(..) | Self::ConstantRef(_) => None,
        }
    }

    // todo implement for empty and empty list
    fn position(&self, language: &TargetLanguage) -> Option<Range> {
        match self {
            Self::Empty(_, _) => None,
            Self::Node(node) => Some(node.range()),
            Self::String(_, range) => Some(range.to_owned()),
            Self::List(parent_node, field_id) => {
                get_range_nodes_for_list(parent_node, field_id, language).map(
                    |(leading_node, trailing_node)| Range {
                        start: Position::new(
                            leading_node.node.start_position().row() + 1,
                            leading_node.node.start_position().column() + 1,
                        ),
                        end: Position::new(
                            trailing_node.node.end_position().row() + 1,
                            trailing_node.node.end_position().column() + 1,
                        ),
                        start_byte: leading_node.node.start_byte(),
                        end_byte: trailing_node.node.end_byte(),
                    },
                )
            }
            Self::FileName(_) => None,
            Self::ConstantRef(_) => None,
        }
    }

    // todo implement for empty and empty list
    fn code_range(&self, language: &TargetLanguage) -> Option<CodeRange> {
        match self {
            Self::Empty(_, _) => None,
            Self::Node(node) => Some(node.code_range()),
            Self::String(src, range) => Some(CodeRange::new(range.start_byte, range.end_byte, src)),
            Self::List(parent_node, field_id) => {
                get_range_nodes_for_list(parent_node, field_id, language).map(
                    |(leading_node, trailing_node)| {
                        CodeRange::new(
                            leading_node.node.start_byte(),
                            trailing_node.node.end_byte(),
                            parent_node.source,
                        )
                    },
                )
            }
            Self::FileName(_) => None,
            Self::ConstantRef(_) => None,
        }
    }

    fn is_equivalent_to(&self, other: &Self, language: &TargetLanguage) -> bool {
        // covers Node, and List with one element
        if let (Some(s1), Some(s2)) = (self.singleton(), other.singleton()) {
            return are_equivalent(&s1, &s2);
        }

        match self {
            // should never occur covered by singleton
            Self::Node(node1) => match other {
                Self::Node(node2) => are_equivalent(node1, node2),
                Self::String(str, range) => self
                    .text(language)
                    .is_ok_and(|t| t == str[range.start_byte as usize..range.end_byte as usize]),
                Self::FileName(_) | Self::List(..) | Self::Empty(..) | Self::ConstantRef(_) => {
                    false
                }
            },
            Self::List(parent_node1, field1) => match other {
                Self::List(parent_node2, field2) => parent_node1
                    .named_children_by_field_id(*field1)
                    .zip_longest(parent_node2.named_children_by_field_id(*field2))
                    .all(|zipped| match zipped {
                        EitherOrBoth::Both(node1, node2) => are_equivalent(&node1, &node2),
                        EitherOrBoth::Left(_) | EitherOrBoth::Right(_) => false,
                    }),
                Self::String(..)
                | Self::FileName(_)
                | Self::Node(..)
                | Self::Empty(..)
                | Self::ConstantRef(_) => false,
            },
            // I suspect matching kind is too strict
            Self::Empty(node1, field1) => match other {
                Self::Empty(node2, field2) => {
                    node1.node.kind_id() == node2.node.kind_id() && field1 == field2
                }
                Self::String(..)
                | Self::FileName(_)
                | Self::Node(..)
                | Self::List(..)
                | Self::ConstantRef(_) => false,
            },
            Self::ConstantRef(c1) => other.as_constant().map_or(false, |c2| *c1 == c2),
            Self::String(s1, range) => other
                .text(language)
                .is_ok_and(|t| t == s1[range.start_byte as usize..range.end_byte as usize]),
            Self::FileName(s1) => other.as_filename().map_or(false, |s2| *s1 == s2),
        }
    }

    fn is_suppressed(&self, language: &TargetLanguage, current_name: Option<&str>) -> bool {
        let node = match self {
            Self::Node(node) | Self::List(node, _) | Self::Empty(node, _) => node.clone(),
            Self::String(_, _) | Self::FileName(_) | Self::ConstantRef(_) => return false,
        };
        let target_range = node.node.range();
        for n in node.children().chain(node.ancestors()) {
            for c in n.children() {
                if !language.is_comment(&c) {
                    continue;
                }
                if is_suppress_comment(&c, &target_range, current_name, language) {
                    return true;
                }
            }
        }

        false
    }

    fn get_insertion_padding(
        &self,
        text: &str,
        is_first: bool,
        language: &TargetLanguage,
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
                if language.is_statement(node)
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

    fn linearized_text(
        &self,
        language: &TargetLanguage,
        effects: &[Effect<'a, MarzanoQueryContext>],
        files: &FileRegistry<'a>,
        memo: &mut HashMap<CodeRange, Option<String>>,
        distributed_indent: Option<usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Cow<'a, str>> {
        let res: Result<Cow<'a, str>> = match self {
            Self::Empty(_, _) => Ok(Cow::Borrowed("")),
            Self::Node(node) => linearize_binding(
                language,
                effects,
                files,
                memo,
                node.clone(),
                node.code_range(),
                distributed_indent,
                logs,
            )
            .map(|r| r.0),
            // can't linearize until we update source to point to the entire file
            // otherwise file file pointers won't match
            Self::String(s, r) => Ok(Cow::Owned(
                s[r.start_byte as usize..r.end_byte as usize].into(),
            )),
            Self::FileName(s) => Ok(Cow::Owned(s.to_string_lossy().into())),
            Self::List(parent_node, _field_id) => {
                if let Some(pos) = self.position(language) {
                    let range = CodeRange::new(pos.start_byte, pos.end_byte, parent_node.source);
                    linearize_binding(
                        language,
                        effects,
                        files,
                        memo,
                        // ideally we should be passing list as an ast_node
                        // a little tricky atm
                        parent_node.clone(),
                        range,
                        distributed_indent,
                        logs,
                    )
                    .map(|r| r.0)
                } else {
                    Ok("".into())
                }
            }
            Self::ConstantRef(c) => Ok(Cow::Owned(c.to_string())),
        };
        res
    }

    fn text(&self, language: &TargetLanguage) -> Result<Cow<str>> {
        match self {
            Self::Empty(_, _) => Ok("".into()),
            Self::Node(node) => Ok(node.text()?),
            Self::String(s, r) => Ok(s[r.start_byte as usize..r.end_byte as usize].into()),
            Self::FileName(s) => Ok(s.to_string_lossy()),
            Self::List(node, _) => Ok(if let Some(pos) = self.position(language) {
                node.source[pos.start_byte as usize..pos.end_byte as usize].into()
            } else {
                "".into()
            }),
            Self::ConstantRef(c) => Ok(c.to_string().into()),
        }
    }

    fn source(&self) -> Option<&'a str> {
        match self {
            Self::Empty(node, _) => Some(node.source),
            Self::Node(node) => Some(node.source),
            Self::String(source, _) => Some(source),
            Self::List(node, _) => Some(node.source),
            Self::FileName(..) | Self::ConstantRef(..) => None,
        }
    }

    /// Returns the constant this binding binds to, if and only if it is a constant binding.
    fn as_constant(&self) -> Option<&Constant> {
        if let Self::ConstantRef(constant) = self {
            Some(constant)
        } else {
            None
        }
    }

    /// Returns the path of this binding, if and only if it is a filename binding.
    fn as_filename(&self) -> Option<&Path> {
        if let Self::FileName(path) = self {
            Some(path)
        } else {
            None
        }
    }

    /// Returns the node of this binding, if and only if it is a node binding.
    fn as_node(&self) -> Option<NodeWithSource<'a>> {
        if let Self::Node(node) = self {
            Some(node.clone())
        } else {
            None
        }
    }

    /// Returns `true` if and only if this binding is bound to a list.
    fn is_list(&self) -> bool {
        matches!(self, Self::List(..))
    }

    /// Returns an iterator over the items in a list.
    ///
    /// Returns `None` if the binding is not bound to a list.
    fn list_items(&self) -> Option<impl Iterator<Item = NodeWithSource<'a>> + Clone> {
        match self {
            Self::List(parent_node, field_id) => {
                Some(parent_node.named_children_by_field_id(*field_id))
            }
            Self::Empty(..)
            | Self::Node(..)
            | Self::String(..)
            | Self::ConstantRef(..)
            | Self::FileName(..) => None,
        }
    }

    /// Returns the parent node of this binding.
    ///
    /// Returns `None` if the binding has no relation to a node.
    fn parent_node(&self) -> Option<NodeWithSource<'a>> {
        match self {
            Self::Node(node) => node.parent(),
            Self::List(node, _) => Some(node.clone()),
            Self::Empty(node, _) => Some(node.clone()),
            Self::String(..) | Self::FileName(..) | Self::ConstantRef(..) => None,
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Self::Empty(..) => false,
            Self::List(node, field_id) => {
                let child_count = node.named_children_by_field_id(*field_id).count();
                child_count > 0
            }
            Self::Node(..) => true,
            // This refers to a slice of the source code, not a Grit string literal, so it is truthy
            Self::String(..) => true,
            Self::FileName(_) => true,
            Self::ConstantRef(c) => c.is_truthy(),
        }
    }

    fn log_empty_field_rewrite_error(
        &self,
        language: &TargetLanguage,
        logs: &mut AnalysisLogs,
    ) -> Result<()> {
        match self {
            Self::Empty(node, field) | Self::List(node, field) => {
                let range = node.range();
                let log = AnalysisLogBuilder::default()
                        .level(441_u16)
                        .source(node.source)
                        .position(range.start)
                        .range(range)
                        .message(format!(
                            "Error: failed to rewrite binding, cannot derive range of empty field {} of node {}",
                            language.get_ts_language().field_name_for_id(*field).unwrap(),
                            node.node.kind()
                        ))
                        .build()?;
                logs.push(log);
            }
            Self::String(_, _) | Self::FileName(_) | Self::Node(_) | Self::ConstantRef(_) => {}
        }

        Ok(())
    }
}

fn get_range_nodes_for_list<'a>(
    parent_node: &NodeWithSource<'a>,
    field_id: &FieldId,
    language: &impl Language<Node<'a> = NodeWithSource<'a>>,
) -> Option<(NodeWithSource<'a>, NodeWithSource<'a>)> {
    let mut children = parent_node.children_by_field_id(*field_id);
    let first_node = children.next()?;

    let end_node = match children.last() {
        None => first_node.clone(),
        Some(last_node) => last_node,
    };

    let mut leading_comment = first_node.clone();
    while let Some(comment) = leading_comment.previous_sibling() {
        if language.is_comment(&comment) {
            leading_comment = comment;
        } else {
            break;
        }
    }
    let mut trailing_comment = end_node;
    while let Some(comment) = trailing_comment.next_sibling() {
        if language.is_comment(&comment) {
            trailing_comment = comment;
        } else {
            break;
        }
    }

    Some((leading_comment, trailing_comment))
}
