use super::{
    accessor::Accessor,
    dynamic_snippet::{DynamicPattern, DynamicSnippet},
    list_index::ListIndex,
    patterns::Pattern,
    state::{FilePtr, FileRegistry, State},
};
use crate::{binding::Binding, constant::Constant, context::QueryContext, problem::Effect};
use anyhow::Result;
use grit_util::CodeRange;
use im::Vector;
use itertools::Itertools;
use marzano_language::language::Language;
use marzano_util::{analysis_logs::AnalysisLogs, position::Range};
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    path::Path,
};

pub trait ResolvedPattern<'a, Q: QueryContext>: Clone + Debug + PartialEq {
    fn from_binding(binding: Q::Binding<'a>) -> Self;

    fn from_constant(constant: Constant) -> Self;

    fn from_constant_binding(constant: &'a Constant) -> Self {
        Self::from_binding(Binding::from_constant(constant))
    }

    fn from_file_pointer(file: FilePtr) -> Self;

    fn from_files(files: Self) -> Self;

    fn from_list_parts(parts: impl Iterator<Item = Self>) -> Self;

    fn from_node_binding(node: Q::Node<'a>) -> Self {
        Self::from_binding(Binding::from_node(node))
    }

    fn from_path_binding(path: &'a Path) -> Self {
        Self::from_binding(Binding::from_path(path))
    }

    fn from_range_binding(range: Range, src: &'a str) -> Self {
        Self::from_binding(Binding::from_range(range, src))
    }

    fn from_string(string: String) -> Self;

    fn from_resolved_snippet(snippet: ResolvedSnippet<'a, Q>) -> Self;

    fn from_dynamic_snippet(
        snippet: &'a DynamicSnippet,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self>;

    fn from_dynamic_pattern(
        pattern: &'a DynamicPattern<Q>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self>;

    fn from_accessor(
        accessor: &'a Accessor<Q>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self>;

    fn from_list_index(
        index: &'a ListIndex<Q>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self>;

    fn from_pattern(
        pattern: &'a Pattern<Q>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self>;

    fn from_patterns(
        patterns: &'a [Option<Pattern<Q>>],
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Vec<Option<Self>>> {
        patterns
            .iter()
            .map(|p| match p {
                Some(pattern) => Ok(Some(Self::from_pattern(pattern, state, context, logs)?)),
                None => Ok(None),
            })
            .collect()
    }

    fn undefined() -> Self {
        Self::from_constant(Constant::Undefined)
    }

    fn extend(
        &mut self,
        with: Q::ResolvedPattern<'a>,
        effects: &mut Vector<Effect<'a, Q>>,
        language: &impl Language,
    ) -> Result<()>;

    fn float(&self, state: &FileRegistry<'a>, language: &impl Language) -> Result<f64>;

    fn get_bindings(&self) -> Option<impl Iterator<Item = Q::Binding<'a>>>;

    fn get_file(&self) -> Option<&Q::File<'a>>;

    fn get_file_pointers(&self) -> Option<Vec<FilePtr>>;

    fn get_files(&self) -> Option<&Self>;

    fn get_last_binding(&self) -> Option<&Q::Binding<'a>>;

    fn get_list_item_at(&self, index: isize) -> Option<&Self>;

    fn get_list_item_at_mut(&mut self, index: isize) -> Option<&mut Self>;

    fn get_list_items(&self) -> Option<impl Iterator<Item = &Self>>;

    fn get_list_binding_items(&self) -> Option<impl Iterator<Item = Self> + Clone>;

    fn get_map(&self) -> Option<&BTreeMap<String, Self>>;

    fn get_map_mut(&mut self) -> Option<&mut BTreeMap<String, Self>>;

    fn get_snippets(&self) -> Option<impl Iterator<Item = ResolvedSnippet<'a, Q>>>;

    fn is_binding(&self) -> bool;

    fn is_list(&self) -> bool;

    fn is_truthy(&self, state: &mut State<'a, Q>, language: &impl Language) -> Result<bool>;

    fn linearized_text(
        &self,
        language: &impl Language,
        effects: &[Effect<'a, Q>],
        files: &FileRegistry<'a>,
        memo: &mut HashMap<CodeRange, Option<String>>,
        should_pad_snippet: bool,
        logs: &mut AnalysisLogs,
    ) -> Result<Cow<'a, str>>;

    fn matches_undefined(&self) -> bool;

    fn matches_false_or_undefined(&self) -> bool;

    fn normalize_insert(
        &mut self,
        binding: &Q::Binding<'a>,
        is_first: bool,
        language: &impl Language,
    ) -> Result<()>;

    fn position(&self, language: &impl Language) -> Option<Range>;

    fn push_binding(&mut self, binding: Q::Binding<'a>) -> Result<()>;

    fn set_list_item_at_mut(&mut self, index: isize, value: Self) -> Result<bool>;

    // should we instead return an Option?
    fn text(&self, state: &FileRegistry<'a>, language: &impl Language) -> Result<Cow<'a, str>>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedSnippet<'a, Q: QueryContext> {
    // if refering to a dynamic_snippet, we can use the &str variant,
    // but if referring to the result of a BuiltIn, we need the
    // String variant
    Text(Cow<'a, str>),
    Binding(Q::Binding<'a>),
    LazyFn(Box<LazyBuiltIn<'a, Q>>),
}

impl<'a, Q: QueryContext> ResolvedSnippet<'a, Q> {
    pub fn from_binding(binding: Q::Binding<'a>) -> ResolvedSnippet<Q> {
        Self::Binding(binding)
    }

    // if the snippet is text consisting of newlines followed by spaces, returns the number of spaces.
    // might not be general enough, but should be good for a first pass
    pub(crate) fn padding(
        &self,
        state: &FileRegistry<'a>,
        language: &impl Language,
    ) -> Result<usize> {
        let text = self.text(state, language)?;
        let len = text.len();
        let trim_len = text.trim_end_matches(' ').len();
        Ok(len - trim_len)
    }

    pub(crate) fn text(
        &self,
        state: &FileRegistry<'a>,
        language: &impl Language,
    ) -> Result<Cow<'a, str>> {
        match self {
            ResolvedSnippet::Text(text) => Ok(text.clone()),
            ResolvedSnippet::Binding(binding) => {
                // we are now taking the unmodified source code, and replacing the binding with the snippet
                // we will want to apply effects next
                binding.text(language).map(|c| c.into_owned().into())
            }
            ResolvedSnippet::LazyFn(lazy) => lazy.text(state, language),
        }
    }

    pub(crate) fn linearized_text(
        &self,
        language: &impl Language,
        effects: &[Effect<'a, Q>],
        files: &FileRegistry<'a>,
        memo: &mut HashMap<CodeRange, Option<String>>,
        distributed_indent: Option<usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Cow<str>> {
        let res = match self {
            Self::Text(text) => {
                if let Some(indent) = distributed_indent {
                    Ok(pad_text(text, indent).into())
                } else {
                    Ok(text.clone())
                }
            }
            Self::Binding(binding) => {
                // we are now taking the unmodified source code, and replacing the binding with the snippet
                // we will want to apply effects next
                binding.linearized_text(language, effects, files, memo, distributed_indent, logs)
            }
            Self::LazyFn(lazy) => {
                lazy.linearized_text(language, effects, files, memo, distributed_indent, logs)
            }
        };
        res
    }

    pub(crate) fn is_truthy(
        &self,
        state: &mut State<'a, Q>,
        language: &impl Language,
    ) -> Result<bool> {
        let truthiness = match self {
            Self::Binding(b) => b.is_truthy(),
            Self::Text(t) => !t.is_empty(),
            Self::LazyFn(t) => !t.text(&state.files, language)?.is_empty(),
        };
        Ok(truthiness)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LazyBuiltIn<'a, Q: QueryContext> {
    Join(JoinFn<'a, Q>),
}

impl<'a, Q: QueryContext> LazyBuiltIn<'a, Q> {
    fn linearized_text(
        &self,
        language: &impl Language,
        effects: &[Effect<'a, Q>],
        files: &FileRegistry<'a>,
        memo: &mut HashMap<CodeRange, Option<String>>,
        distributed_indent: Option<usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Cow<str>> {
        match self {
            LazyBuiltIn::Join(join) => {
                join.linearized_text(language, effects, files, memo, distributed_indent, logs)
            }
        }
    }

    pub(crate) fn text(
        &self,
        state: &FileRegistry<'a>,
        language: &impl Language,
    ) -> Result<Cow<'a, str>> {
        match self {
            LazyBuiltIn::Join(join) => join.text(state, language),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct JoinFn<'a, Q: QueryContext> {
    pub(crate) list: Vector<Q::ResolvedPattern<'a>>,
    separator: String,
}

impl<'a, Q: QueryContext> JoinFn<'a, Q> {
    pub(crate) fn from_patterns(
        patterns: impl Iterator<Item = Q::ResolvedPattern<'a>>,
        separator: String,
    ) -> Self {
        Self {
            list: patterns.collect(),
            separator,
        }
    }

    fn linearized_text(
        &self,
        language: &impl Language,
        effects: &[Effect<'a, Q>],
        files: &FileRegistry<'a>,
        memo: &mut HashMap<CodeRange, Option<String>>,
        distributed_indent: Option<usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Cow<str>> {
        let res = self
            .list
            .iter()
            .map(|pattern| {
                pattern.linearized_text(
                    language,
                    effects,
                    files,
                    memo,
                    distributed_indent.is_some(),
                    logs,
                )
            })
            .collect::<Result<Vec<_>>>()?
            .join(&self.separator);
        if let Some(padding) = distributed_indent {
            Ok(pad_text(&res, padding).into())
        } else {
            Ok(res.into())
        }
    }

    fn text(&self, state: &FileRegistry<'a>, language: &impl Language) -> Result<Cow<'a, str>> {
        Ok(self
            .list
            .iter()
            .map(|pattern| pattern.text(state, language))
            .collect::<Result<Vec<_>>>()?
            .join(&self.separator)
            .into())
    }
}

fn pad_text(text: &str, padding: usize) -> String {
    if text.trim().is_empty() {
        text.to_owned()
    } else {
        let mut res = if text.starts_with('\n') {
            "\n".to_owned()
        } else {
            String::new()
        };
        res.push_str(&text.lines().join(&format!("\n{}", " ".repeat(padding))));
        if text.ends_with('\n') {
            res.push('\n')
        };
        res
    }
}

pub trait File<'a, Q: QueryContext> {
    fn name(&self, files: &FileRegistry<'a>) -> Q::ResolvedPattern<'a>;

    fn absolute_path(
        &self,
        files: &FileRegistry<'a>,
        language: &impl Language,
    ) -> Result<Q::ResolvedPattern<'a>>;

    fn body(&self, files: &FileRegistry<'a>) -> Q::ResolvedPattern<'a>;

    fn binding(&self, files: &FileRegistry<'a>) -> Q::ResolvedPattern<'a>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedFile<'a, Q: QueryContext> {
    pub name: Q::ResolvedPattern<'a>,
    pub body: Q::ResolvedPattern<'a>,
}
