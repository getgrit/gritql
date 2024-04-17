use super::{
    accessor::Accessor,
    code_snippet::CodeSnippet,
    dynamic_snippet::{DynamicPattern, DynamicSnippet, DynamicSnippetPart},
    functions::GritCall,
    list_index::{to_unsigned, ListIndex},
    paths::absolutize,
    patterns::Pattern,
    state::{FilePtr, FileRegistry, State},
};
use crate::{
    binding::{Binding, Constant},
    context::{ExecContext, QueryContext},
    pattern::{container::PatternOrResolved, patterns::PatternName},
    problem::{Effect, EffectKind},
};
use anyhow::{anyhow, bail, Result};
use grit_util::CodeRange;
use im::{vector, Vector};
use itertools::Itertools;
use marzano_language::language::{FieldId, Language};
use marzano_util::{
    analysis_logs::AnalysisLogs, node_with_source::NodeWithSource, position::Range,
};
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    path::Path,
};

/**
 * This file contains data structures for everything resolved.
 */

#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedSnippet<'a> {
    // if refering to a dynamic_snippet, we can use the &str variant,
    // but if referring to the result of a BuiltIn, we need the
    // String variant
    Text(Cow<'a, str>),
    Binding(Binding<'a>),
    LazyFn(Box<LazyBuiltIn<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedFile<'a> {
    name: ResolvedPattern<'a>,
    body: ResolvedPattern<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum File<'a> {
    Resolved(Box<ResolvedFile<'a>>),
    Ptr(FilePtr),
}

impl<'a> File<'a> {
    pub(crate) fn name(&self, files: &FileRegistry<'a>) -> ResolvedPattern<'a> {
        match self {
            File::Resolved(resolved) => resolved.name.clone(),
            File::Ptr(ptr) => ResolvedPattern::from_path_binding(&files.get_file(*ptr).name),
        }
    }

    pub(crate) fn absolute_path(
        &self,
        files: &FileRegistry<'a>,
        language: &impl Language,
    ) -> Result<ResolvedPattern<'a>> {
        match self {
            File::Resolved(resolved) => {
                let name = resolved.name.text(files, language)?;
                let absolute_path = absolutize(name.as_ref())?;
                Ok(ResolvedPattern::Constant(Constant::String(absolute_path)))
            }
            File::Ptr(ptr) => Ok(ResolvedPattern::from_path_binding(
                &files.get_file(*ptr).absolute_path,
            )),
        }
    }

    pub(crate) fn body(&self, files: &FileRegistry<'a>) -> ResolvedPattern<'a> {
        match self {
            File::Resolved(resolved) => resolved.body.clone(),
            File::Ptr(ptr) => {
                let file = &files.get_file(*ptr);
                let range = file.tree.root_node().range().into();
                ResolvedPattern::from_range_binding(range, &file.source)
            }
        }
    }

    pub(crate) fn binding(&self, files: &FileRegistry<'a>) -> ResolvedPattern<'a> {
        match self {
            File::Resolved(resolved) => resolved.body.clone(),
            File::Ptr(ptr) => {
                let file = &files.get_file(*ptr);
                let node = file.tree.root_node();
                ResolvedPattern::from_node_binding(NodeWithSource::new(node, &file.source))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct JoinFn<'a> {
    pub(crate) list: Vector<ResolvedPattern<'a>>,
    separator: String,
}

impl<'a> JoinFn<'a> {
    pub(crate) fn from_patterns(
        patterns: impl Iterator<Item = ResolvedPattern<'a>>,
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
        effects: &[Effect<'a>],
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

#[derive(Debug, Clone, PartialEq)]
pub enum LazyBuiltIn<'a> {
    Join(JoinFn<'a>),
}

impl<'a> LazyBuiltIn<'a> {
    fn linearized_text(
        &self,
        language: &impl Language,
        effects: &[Effect<'a>],
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
pub enum ResolvedPattern<'a> {
    Binding(Vector<Binding<'a>>),
    Snippets(Vector<ResolvedSnippet<'a>>),
    List(Vector<ResolvedPattern<'a>>),
    Map(BTreeMap<String, ResolvedPattern<'a>>),
    File(File<'a>),
    Files(Box<ResolvedPattern<'a>>),
    Constant(Constant),
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

impl<'a> ResolvedSnippet<'a> {
    pub fn from_binding(binding: Binding) -> ResolvedSnippet {
        ResolvedSnippet::Binding(binding)
    }

    // if the snippet is text consisting of newlines followed by spaces, returns the number of spaces.
    // might not be general enough, but should be good for a first pass
    fn padding(&self, state: &FileRegistry<'a>, language: &impl Language) -> Result<usize> {
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
        effects: &[Effect<'a>],
        files: &FileRegistry<'a>,
        memo: &mut HashMap<CodeRange, Option<String>>,
        distributed_indent: Option<usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Cow<str>> {
        let res = match self {
            ResolvedSnippet::Text(text) => {
                if let Some(indent) = distributed_indent {
                    Ok(pad_text(text, indent).into())
                } else {
                    Ok(text.clone())
                }
            }
            ResolvedSnippet::Binding(binding) => {
                // we are now taking the unmodified source code, and replacing the binding with the snippet
                // we will want to apply effects next
                binding.linearized_text(language, effects, files, memo, distributed_indent, logs)
            }
            ResolvedSnippet::LazyFn(lazy) => {
                lazy.linearized_text(language, effects, files, memo, distributed_indent, logs)
            }
        };
        res
    }

    pub(crate) fn is_truthy<Q: QueryContext>(
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

impl<'a> ResolvedPattern<'a> {
    pub fn extend(
        &mut self,
        mut with: Self,
        effects: &mut Vector<Effect<'a>>,
        language: &impl Language,
    ) -> Result<()> {
        match self {
            Self::Binding(bindings) => {
                let new_effects: Result<Vec<Effect>> = bindings
                    .iter()
                    .map(|b| {
                        let is_first = !effects.iter().any(|e| e.binding == *b);
                        with.normalize_insert(b, is_first, language)?;
                        Ok(Effect {
                            binding: b.clone(),
                            pattern: with.clone(),
                            kind: EffectKind::Insert,
                        })
                    })
                    .collect();
                let new_effects = new_effects?;
                effects.extend(new_effects);
                Ok(())
            }
            Self::Snippets(snippets) => {
                match with {
                    Self::Snippets(with_snippets) => {
                        snippets.extend(with_snippets);
                    }
                    Self::Binding(binding) => {
                        let binding = binding
                            .last()
                            .ok_or_else(|| anyhow!("cannot extend with empty binding"))?;
                        snippets.push_back(ResolvedSnippet::Binding(binding.clone()));
                    }
                    Self::List(_) => {
                        return Err(anyhow!("cannot extend ResolvedPattern::Snippet with List"))
                    }
                    Self::File(_) => {
                        return Err(anyhow!("cannot extend ResolvedPattern::Snippet with File"))
                    }
                    Self::Files(_) => {
                        return Err(anyhow!("cannot extend ResolvedPattern::Snippet with Files"))
                    }
                    Self::Map(_) => {
                        return Err(anyhow!("cannot extend ResolvedPattern::Snippet with Map"))
                    }
                    Self::Constant(c) => {
                        snippets.push_back(ResolvedSnippet::Text(c.to_string().into()));
                    }
                }
                Ok(())
            }
            // do we want to auto flattern?
            // for now not since don't know what shape we want,
            // but probably will soon
            Self::List(lst) => {
                lst.push_back(with);
                Ok(())
            }
            Self::File(_) => Err(anyhow!("cannot extend ResolvedPattern::File")),
            Self::Files(_) => Err(anyhow!("cannot extend ResolvedPattern::Files")),
            Self::Map(_) => Err(anyhow!("cannot extend ResolvedPattern::Map")),
            Self::Constant(Constant::Integer(i)) => {
                if let Self::Constant(Constant::Integer(j)) = with {
                    *i += j;
                    Ok(())
                } else {
                    Err(anyhow!(
                        "can only extend Constant::Integer with another Constant::Integer"
                    ))
                }
            }
            Self::Constant(Constant::Float(x)) => {
                if let Self::Constant(Constant::Float(y)) = with {
                    *x += y;
                    Ok(())
                } else {
                    Err(anyhow!(
                        "can only extend Constant::Float with another Constant::Float"
                    ))
                }
            }
            Self::Constant(_) => Err(anyhow!("cannot extend ResolvedPattern::Constant")),
        }
    }

    pub(crate) fn position(&self, language: &impl Language) -> Option<Range> {
        if let Self::Binding(binding) = self {
            if let Some(binding) = binding.last() {
                return binding.position(language);
            }
        }
        None
    }

    pub(crate) fn from_binding(binding: Binding<'a>) -> Self {
        Self::Binding(vector![binding])
    }

    pub(crate) fn undefined() -> Self {
        Self::from_constant(Constant::Undefined)
    }

    pub fn from_constant(constant: Constant) -> Self {
        Self::Constant(constant)
    }

    pub fn from_constant_binding(constant: &'a Constant) -> Self {
        Self::from_binding(Binding::from_constant(constant))
    }

    pub(crate) fn from_file_pointer(file: FilePtr) -> Self {
        Self::File(File::Ptr(file))
    }

    pub(crate) fn from_files(files: Self) -> Self {
        Self::Files(Box::new(files))
    }

    pub(crate) fn from_node_binding(node: NodeWithSource<'a>) -> Self {
        Self::from_binding(Binding::from_node(node))
    }

    pub(crate) fn from_list_parts(parts: impl Iterator<Item = ResolvedPattern<'a>>) -> Self {
        Self::List(parts.collect())
    }

    pub(crate) fn from_list_binding(node: NodeWithSource<'a>, field_id: FieldId) -> Self {
        Self::from_binding(Binding::List(node, field_id))
    }

    pub(crate) fn from_empty_binding(node: NodeWithSource<'a>, field_id: FieldId) -> Self {
        Self::from_binding(Binding::Empty(node, field_id))
    }

    pub(crate) fn from_path_binding(path: &'a Path) -> Self {
        Self::from_binding(Binding::from_path(path))
    }

    pub(crate) fn from_range_binding(range: Range, src: &'a str) -> Self {
        Self::from_binding(Binding::from_range(range, src))
    }

    pub fn from_string(string: String) -> Self {
        Self::Snippets(vector![ResolvedSnippet::Text(string.into())])
    }

    pub(crate) fn from_resolved_snippet(snippet: ResolvedSnippet<'a>) -> Self {
        Self::Snippets(vector![snippet])
    }

    fn to_snippets(&self) -> Result<Vector<ResolvedSnippet<'a>>> {
        match self {
            Self::Snippets(snippets) => Ok(snippets.clone()),
            Self::Binding(bindings) => Ok(vector![ResolvedSnippet::from_binding(
                bindings
                    .last()
                    .ok_or_else(|| {
                        anyhow!("cannot create resolved snippet from unresolved binding")
                    })?
                    .to_owned(),
            )]),
            Self::List(elements) => {
                // merge separated by space
                let mut snippets = Vec::new();
                for pattern in elements {
                    snippets.extend(pattern.to_snippets()?);
                    snippets.push(ResolvedSnippet::Text(" ".into()));
                }
                snippets.pop();
                Ok(snippets.into())
            }
            Self::Map(map) => {
                let mut snippets = Vec::new();
                snippets.push(ResolvedSnippet::Text("{".into()));
                for (key, value) in map {
                    snippets.push(ResolvedSnippet::Text(format!("\"{}\": ", key).into()));
                    snippets.extend(value.to_snippets()?);
                    snippets.push(ResolvedSnippet::Text(", ".into()));
                }
                snippets.pop();
                snippets.push(ResolvedSnippet::Text("}".into()));
                Ok(snippets.into())
            }
            Self::File(_) => Err(anyhow!(
                "cannot convert ResolvedPattern::File to ResolvedSnippet"
            )),
            Self::Files(_) => Err(anyhow!(
                "cannot convert ResolvedPattern::Files to ResolvedSnippet"
            )),
            Self::Constant(c) => Ok(vector![ResolvedSnippet::Text(c.to_string().into(),)]),
        }
    }

    pub fn get_bindings(&self) -> Option<impl Iterator<Item = &Binding<'a>>> {
        if let Self::Binding(bindings) = self {
            Some(bindings.iter())
        } else {
            None
        }
    }

    pub fn get_file(&self) -> Option<&File<'a>> {
        if let Self::File(file) = self {
            Some(file)
        } else {
            None
        }
    }

    pub fn get_file_pointers(&self) -> Option<Vec<FilePtr>> {
        match self {
            Self::Binding(_) => None,
            Self::Snippets(_) => None,
            Self::List(_) => handle_files(self),
            Self::Map(_) => None,
            Self::File(file) => extract_file_pointer(file).map(|f| vec![f]),
            Self::Files(files) => handle_files(files),
            Self::Constant(_) => None,
        }
    }

    pub fn get_files(&self) -> Option<&Self> {
        if let Self::Files(files) = self {
            Some(files)
        } else {
            None
        }
    }

    pub fn get_last_binding(&self) -> Option<&Binding<'a>> {
        if let Self::Binding(bindings) = self {
            bindings.last()
        } else {
            None
        }
    }

    pub fn get_list_item_at(&self, index: isize) -> Option<&Self> {
        if let Self::List(items) = self {
            to_unsigned(index, items.len()).and_then(|index| items.get(index))
        } else {
            None
        }
    }

    pub fn get_list_item_at_mut(&mut self, index: isize) -> Option<&mut Self> {
        if let Self::List(items) = self {
            to_unsigned(index, items.len()).and_then(|index| items.get_mut(index))
        } else {
            None
        }
    }

    pub fn get_list_items(&self) -> Option<impl Iterator<Item = &Self>> {
        if let Self::List(items) = self {
            Some(items.iter())
        } else {
            None
        }
    }

    pub fn get_list_binding_items(&self) -> Option<impl Iterator<Item = Self> + Clone> {
        self.get_last_binding()
            .and_then(Binding::list_items)
            .map(|items| items.map(ResolvedPattern::from_node_binding))
    }

    pub fn get_map(&self) -> Option<&BTreeMap<String, Self>> {
        if let Self::Map(map) = self {
            Some(map)
        } else {
            None
        }
    }

    pub fn get_map_mut(&mut self) -> Option<&mut BTreeMap<String, Self>> {
        if let Self::Map(map) = self {
            Some(map)
        } else {
            None
        }
    }

    pub fn get_snippets(&self) -> Option<impl Iterator<Item = &ResolvedSnippet<'a>>> {
        if let Self::Snippets(snippets) = self {
            Some(snippets.iter())
        } else {
            None
        }
    }

    pub fn is_binding(&self) -> bool {
        matches!(self, Self::Binding(_))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }

    pub fn push_binding(&mut self, binding: Binding<'a>) -> Result<()> {
        let Self::Binding(bindings) = self else {
            bail!("can only push to bindings");
        };

        bindings.push_back(binding);
        Ok(())
    }

    pub fn set_list_item_at_mut(&mut self, index: isize, value: Self) -> Result<bool> {
        let Self::List(items) = self else {
            bail!("can only set items on a list")
        };

        let Some(index) = to_unsigned(index, items.len()) else {
            return Ok(false);
        };

        items.insert(index, value);
        Ok(true)
    }

    pub fn from_dynamic_snippet<Q: QueryContext>(
        snippet: &'a DynamicSnippet,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        let mut parts = Vec::new();
        for part in &snippet.parts {
            match part {
                DynamicSnippetPart::String(string) => {
                    parts.push(ResolvedSnippet::Text(string.into()));
                }
                DynamicSnippetPart::Variable(var) => {
                    let content = &state.bindings[var.scope].last().unwrap()[var.index];
                    let name = &content.name;
                    // feels weird not sure if clone is correct
                    let value = if let Some(value) = &content.value {
                        value.clone()
                    } else if let Some(pattern) = content.pattern {
                        Self::from_pattern(pattern, state, context, logs)?
                    } else {
                        bail!(
                            "cannot create resolved snippet from unresolved variable {}",
                            name
                        )
                    };
                    let value = value.to_snippets()?;
                    parts.extend(value);
                }
            }
        }
        Ok(Self::Snippets(parts.into()))
    }

    pub fn from_dynamic_pattern<Q: QueryContext>(
        pattern: &'a DynamicPattern<Q>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        match pattern {
            DynamicPattern::Variable(var) => {
                let content = &state.bindings[var.scope].last().unwrap()[var.index];
                let name = &content.name;
                // feels weird not sure if clone is correct
                if let Some(value) = &content.value {
                    Ok(value.clone())
                } else if let Some(pattern) = content.pattern {
                    Self::from_pattern(pattern, state, context, logs)
                } else {
                    bail!(
                        "cannot create resolved snippet from unresolved variable {}",
                        name
                    )
                }
            }
            DynamicPattern::Accessor(accessor) => {
                Self::from_accessor(accessor, state, context, logs)
            }
            DynamicPattern::ListIndex(index) => Self::from_list_index(index, state, context, logs),
            DynamicPattern::List(list) => {
                let mut elements = Vec::new();
                for element in &list.elements {
                    elements.push(Self::from_dynamic_pattern(element, state, context, logs)?);
                }
                Ok(Self::List(elements.into()))
            }
            DynamicPattern::Snippet(snippet) => {
                Self::from_dynamic_snippet(snippet, state, context, logs)
            }
            DynamicPattern::CallBuiltIn(built_in) => built_in.call(state, context, logs),
            DynamicPattern::CallFunction(func) => func.call(state, context, logs),
            DynamicPattern::CallForeignFunction(func) => func.call(state, context, logs),
        }
    }

    pub(crate) fn from_accessor<Q: QueryContext>(
        accessor: &'a Accessor<Q>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        match accessor.get(state, context.language())? {
            Some(PatternOrResolved::Pattern(pattern)) => {
                Self::from_pattern(pattern, state, context, logs)
            }
            Some(PatternOrResolved::ResolvedBinding(resolved)) => Ok(resolved),
            Some(PatternOrResolved::Resolved(resolved)) => Ok(resolved.clone()),
            None => Ok(Self::from_constant_binding(&Constant::Undefined)),
        }
    }

    pub(crate) fn from_list_index<Q: QueryContext>(
        index: &'a ListIndex<Q>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        match index.get(state, context.language())? {
            Some(PatternOrResolved::Pattern(pattern)) => {
                Self::from_pattern(pattern, state, context, logs)
            }
            Some(PatternOrResolved::ResolvedBinding(resolved)) => Ok(resolved),
            Some(PatternOrResolved::Resolved(resolved)) => Ok(resolved.clone()),
            None => Ok(Self::from_constant_binding(&Constant::Undefined)),
        }
    }

    pub fn from_pattern<Q: QueryContext>(
        pattern: &'a Pattern<Q>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        match pattern {
            Pattern::Dynamic(pattern) => Self::from_dynamic_pattern(pattern, state, context, logs),
            Pattern::CodeSnippet(CodeSnippet {
                dynamic_snippet: Some(pattern),
                ..
            }) => Self::from_dynamic_pattern(pattern, state, context, logs),
            Pattern::CallBuiltIn(built_in) => built_in.call(state, context, logs),
            Pattern::CallFunction(func) => func.call(state, context, logs),
            Pattern::CallForeignFunction(func) => func.call(state, context, logs),
            Pattern::StringConstant(string) => Ok(Self::Snippets(vector![ResolvedSnippet::Text(
                (&string.text).into(),
            )])),
            Pattern::IntConstant(int) => Ok(Self::Constant(Constant::Integer(int.value))),
            Pattern::FloatConstant(double) => Ok(Self::Constant(Constant::Float(double.value))),
            Pattern::BooleanConstant(bool) => Ok(Self::Constant(Constant::Boolean(bool.value))),
            Pattern::Variable(var) => {
                let content = &state.bindings[var.scope].last().unwrap()[var.index];
                let name = &content.name;
                // feels weird not sure if clone is correct
                if let Some(value) = &content.value {
                    Ok(value.clone())
                } else if let Some(pattern) = content.pattern {
                    Self::from_pattern(pattern, state, context, logs)
                } else {
                    bail!(
                        "cannot create resolved snippet from unresolved variable {}",
                        name
                    )
                }
            }
            Pattern::List(list) => list
                .patterns
                .iter()
                .map(|pattern| Self::from_pattern(pattern, state, context, logs))
                .collect::<Result<Vector<_>>>()
                .map(Self::List),
            Pattern::ListIndex(index) => Self::from_list_index(index, state, context, logs),
            Pattern::Map(map) => map
                .elements
                .iter()
                .map(|(key, value)| {
                    Ok((
                        key.clone(),
                        Self::from_pattern(value, state, context, logs)?,
                    ))
                })
                .collect::<Result<BTreeMap<_, _>>>()
                .map(Self::Map),
            Pattern::Accessor(accessor) => Self::from_accessor(accessor, state, context, logs),
            Pattern::File(file_pattern) => {
                let name = &file_pattern.name;
                let body = &file_pattern.body;
                let name = Self::from_pattern(name, state, context, logs)?;
                let name = name.text(&state.files, context.language())?;
                let name = Self::Constant(Constant::String(name.to_string()));
                let body = Self::from_pattern(body, state, context, logs)?;
                // todo: replace GENERATED_SOURCE with a computed source once linearization and
                // on-the-fly rewrites are in place
                Ok(Self::File(File::Resolved(Box::new(ResolvedFile {
                    name,
                    body,
                }))))
            }
            Pattern::Add(add_pattern) => add_pattern.call(state, context, logs),
            Pattern::Subtract(subtract_pattern) => subtract_pattern.call(state, context, logs),
            Pattern::Multiply(multiply_pattern) => multiply_pattern.call(state, context, logs),
            Pattern::Divide(divide_pattern) => divide_pattern.call(state, context, logs),
            Pattern::Modulo(modulo_pattern) => modulo_pattern.call(state, context, logs),
            Pattern::Before(before) => before.prev_pattern(state, context, logs),
            Pattern::After(after) => after.next_pattern(state, context, logs),
            Pattern::AstNode(_)
            | Pattern::CodeSnippet(_)
            | Pattern::Call(_)
            | Pattern::Regex(_)
            | Pattern::Files(_)
            | Pattern::Bubble(_)
            | Pattern::Limit(_)
            | Pattern::Assignment(_)
            | Pattern::Accumulate(_)
            | Pattern::And(_)
            | Pattern::Or(_)
            | Pattern::Maybe(_)
            | Pattern::Any(_)
            | Pattern::Not(_)
            | Pattern::If(_)
            | Pattern::Undefined
            | Pattern::Top
            | Pattern::Bottom
            | Pattern::Underscore
            | Pattern::AstLeafNode(_)
            | Pattern::Rewrite(_)
            | Pattern::Log(_)
            | Pattern::Range(_)
            | Pattern::Contains(_)
            | Pattern::Includes(_)
            | Pattern::Within(_)
            | Pattern::Where(_)
            | Pattern::Some(_)
            | Pattern::Every(_)
            | Pattern::Dots
            | Pattern::Like(_)
            | Pattern::Sequential(_) => Err(anyhow!(format!(
                "cannot make resolved pattern from arbitrary pattern {}",
                pattern.name()
            ))),
        }
    }

    pub(crate) fn linearized_text(
        &self,
        language: &impl Language,
        effects: &[Effect<'a>],
        files: &FileRegistry<'a>,
        memo: &mut HashMap<CodeRange, Option<String>>,
        should_pad_snippet: bool,
        logs: &mut AnalysisLogs,
    ) -> Result<Cow<'a, str>> {
        match self {
            // if whitespace is significant we need to distribute indentations
            // across lines within the snippet
            Self::Snippets(snippets) => {
                if should_pad_snippet {
                    let mut res = String::new();
                    let mut padding = 0;
                    for snippet in snippets {
                        let text = snippet.linearized_text(
                            language,
                            effects,
                            files,
                            memo,
                            Some(padding),
                            logs,
                        )?;
                        padding = snippet.padding(files, language)?;
                        res.push_str(&text);
                    }
                    Ok(res.into())
                } else {
                    Ok(snippets
                        .iter()
                        .map(|snippet| {
                            snippet.linearized_text(language, effects, files, memo, None, logs)
                        })
                        .collect::<Result<Vec<_>>>()?
                        .join("")
                        .into())
                }
            }
            // we may have to distribute indentations as we did for snippets above
            Self::List(list) => Ok(list
                .iter()
                .map(|pattern| {
                    pattern.linearized_text(
                        language,
                        effects,
                        files,
                        memo,
                        should_pad_snippet,
                        logs,
                    )
                })
                .collect::<Result<Vec<_>>>()?
                .join(",")
                .into()),
            Self::Map(map) => Ok(("{".to_string()
                + &map
                    .iter()
                    .map(|(key, value)| {
                        let linearized = match value.linearized_text(
                            language,
                            effects,
                            files,
                            memo,
                            should_pad_snippet,
                            logs,
                        ) {
                            Ok(linearized) => linearized,
                            Err(err) => {
                                return Err(err);
                            }
                        };
                        Ok((key, linearized))
                    })
                    .collect::<Result<Vec<_>>>()?
                    .iter()
                    .map(|(key, value)| format!("\"{}\": {}", key, value))
                    .collect::<Vec<_>>()
                    .join(", ")
                + "}")
                .into()),
            // might have to handle differently for ResolvedPattern::List containing indent followed by binding
            Self::Binding(binding) => Ok(binding
                .last()
                .ok_or_else(|| anyhow!("cannot grab text of resolved_pattern with no binding"))?
                .linearized_text(
                    language,
                    effects,
                    files,
                    memo,
                    should_pad_snippet.then_some(0),
                    logs,
                )?),
            Self::File(file) => Ok(format!(
                "{}:\n{}",
                file.name(files)
                    .linearized_text(language, effects, files, memo, false, logs)?,
                file.body(files).linearized_text(
                    language,
                    effects,
                    files,
                    memo,
                    should_pad_snippet,
                    logs,
                )?
            )
            .into()),
            // unsure if this is correct, taken from text
            Self::Files(_files) => {
                bail!("cannot linearize files pattern, not implemented yet");
            }
            // unsure if this is correct, taken from text
            Self::Constant(c) => Ok(c.to_string().into()),
        }
    }

    pub(crate) fn float(&self, state: &FileRegistry<'a>, language: &impl Language) -> Result<f64> {
        match self {
            Self::Constant(c) => match c {
                Constant::Float(d) => Ok(*d),
                Constant::Integer(i) => Ok(*i as f64),
                Constant::String(s) => Ok(s.parse::<f64>()?),
                Constant::Boolean(_) | Constant::Undefined => Err(anyhow!("Cannot convert constant to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")),
            },
            Self::Snippets(s) => {
                let text = s
                    .iter()
                    .map(|snippet| snippet.text(state, language))
                    .collect::<Result<Vec<_>>>()?
                    .join("");
                text.parse::<f64>().map_err(|_| {
                    anyhow!("Failed to convert snippet to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")
                })
            }
            Self::Binding(binding) => {
                let text = binding
                    .last()
                    .ok_or_else(|| anyhow!("cannot grab text of resolved_pattern with no binding"))?
                    .text(language)?;
                text.parse::<f64>().map_err(|_| {
                    anyhow!("Failed to convert binding to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")
                })
            }
            Self::List(_) | Self::Map(_) | Self::File(_) | Self::Files(_) => Err(anyhow!("Cannot convert type to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")),
        }
    }

    pub(crate) fn matches_undefined(&self) -> bool {
        match self {
            Self::Binding(b) => b
                .last()
                .and_then(Binding::as_constant)
                .map_or(false, Constant::is_undefined),
            Self::Constant(Constant::Undefined) => true,
            Self::Constant(_)
            | Self::Snippets(_)
            | Self::List(_)
            | Self::Map(_)
            | Self::File(_)
            | Self::Files(_) => false,
        }
    }

    pub(crate) fn matches_false_or_undefined(&self) -> bool {
        // should this match a binding to the constant `false` as well?
        matches!(self, Self::Constant(Constant::Boolean(false))) || self.matches_undefined()
    }

    // should we instead return an Option?
    pub fn text(&self, state: &FileRegistry<'a>, language: &impl Language) -> Result<Cow<'a, str>> {
        match self {
            Self::Snippets(snippets) => Ok(snippets
                .iter()
                .map(|snippet| snippet.text(state, language))
                .collect::<Result<Vec<_>>>()?
                .join("")
                .into()),
            Self::List(list) => Ok(list
                .iter()
                .map(|pattern| pattern.text(state, language))
                .collect::<Result<Vec<_>>>()?
                .join(",")
                .into()),
            Self::Map(map) => Ok(("{".to_string()
                + &map
                    .iter()
                    .map(|(key, value)| {
                        format!(
                            "\"{}\": {}",
                            key,
                            value
                                .text(state, language)
                                .expect("failed to get text of map value")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
                + "}")
                .into()),
            Self::Binding(binding) => Ok(binding
                .last()
                .ok_or_else(|| anyhow!("cannot grab text of resolved_pattern with no binding"))?
                .text(language)?
                .into_owned()
                .into()),
            Self::File(file) => Ok(format!(
                "{}:\n{}",
                file.name(state).text(state, language)?,
                file.body(state).text(state, language)?
            )
            .into()),
            Self::Files(files) => files.text(state, language),
            Self::Constant(constant) => Ok(constant.to_string().into()),
        }
    }

    pub(crate) fn normalize_insert(
        &mut self,
        binding: &Binding<'a>,
        is_first: bool,
        language: &impl Language,
    ) -> Result<()> {
        let Self::Snippets(ref mut snippets) = self else {
            return Ok(());
        };
        let Some(ResolvedSnippet::Text(text)) = snippets.front() else {
            return Ok(());
        };
        if let Some(padding) = binding.get_insertion_padding(text, is_first, language) {
            if padding.chars().next() != binding.text(language)?.chars().last() {
                snippets.push_front(ResolvedSnippet::Text(padding.into()));
            }
        }
        Ok(())
    }

    pub(crate) fn is_truthy<Q: QueryContext>(
        &self,
        state: &mut State<'a, Q>,
        language: &impl Language,
    ) -> Result<bool> {
        let truthiness = match self {
            Self::Binding(bindings) => bindings.last().map_or(false, Binding::is_truthy),
            Self::List(elements) => !elements.is_empty(),
            Self::Map(map) => !map.is_empty(),
            Self::Constant(c) => c.is_truthy(),
            Self::Snippets(s) => {
                if let Some(s) = s.last() {
                    s.is_truthy(state, language)?
                } else {
                    false
                }
            }
            Self::File(..) => true,
            Self::Files(..) => true,
        };
        Ok(truthiness)
    }
}

pub(crate) fn pattern_to_binding<'a, Q: QueryContext>(
    pattern: &'a Pattern<Q>,
    state: &mut State<'a, Q>,
    context: &'a Q::ExecContext<'a>,
    logs: &mut AnalysisLogs,
) -> Result<Binding<'a>> {
    let resolved = ResolvedPattern::from_pattern(pattern, state, context, logs)?;
    if let ResolvedPattern::Binding(binding) = resolved {
        Ok(binding
            .last()
            .ok_or_else(|| anyhow!("cannot create binding from resolved pattern with no binding"))?
            .to_owned())
    } else {
        bail!("pattern did not resolve to binding")
    }
}

// borrow here seems off I think we want Vec<&ResolvedPattern>
// since we'll be getting pointers to var_content
pub fn patterns_to_resolved<'a, Q: QueryContext>(
    patterns: &'a [Option<Pattern<Q>>],
    state: &mut State<'a, Q>,
    context: &'a Q::ExecContext<'a>,
    logs: &mut AnalysisLogs,
) -> Result<Vec<Option<ResolvedPattern<'a>>>> {
    patterns
        .iter()
        .map(|p| match p {
            Some(pattern) => Ok(Some(ResolvedPattern::from_pattern(
                pattern, state, context, logs,
            )?)),
            None => Ok(None),
        })
        .collect::<Result<Vec<_>>>()
}

fn extract_file_pointer(file: &File) -> Option<FilePtr> {
    match file {
        File::Resolved(_) => None,
        File::Ptr(ptr) => Some(*ptr),
    }
}

fn handle_files(files_list: &ResolvedPattern) -> Option<Vec<FilePtr>> {
    let files = files_list.get_list_items()?;
    files
        .map(|r| r.get_file().and_then(extract_file_pointer))
        .collect()
}

/*

# On regular pattern matching

pattern foo($xparam, $yparam) = pair(key = $xparam, value = $yparam))

foo(xparam = $xarg, yparam = `bar($yarg)`)

on call:
xparam -> {
    pattern = $xarg
}
yparam -> {
    pattern = `bar($yarg)`
}

## on "key = $xparam"

Let's say this leads to `5` <: $xparam.

If $xparam has a .pattern (like in this case), also try to pattern match the pattern
(happening inside `Matcher<Q> for Variable`). Similar to what we do now with `.assigned`, but doing it with `.pattern`.

xparam -> {
    pattern = $arg
    value = `5`
}
$xarg -> {
    value = `5`
}

So we already have the backpropagation of stuff to $arg.

## on "value = $yparam"

Let's say this leads to `bar(6)` <: $yparam.

When matching against pattern (as above) we do `bar(6)` <: `bar($yarg)`

Which leads to:

yparam -> {
    pattern = `bar($yarg)`
    value = `bar(6)`
}

$yarg -> {
    value = `6`
}

Again, we have nice backpropagation.

# On assignments

foo(xparam = $xarg, yparam = `bar($yarg)`)

pattern foo($xparam, $yparam) = {
    $xparam = `5`
    $yparam = `bar(6)`
}

## on "$xparam = `5`"

$xparam -> {
    pattern = $arg
    value = `5`
}

If `.pattern` is just a variable, propagate the assignment to it:

$xarg -> {
    value = `5`
}

If $arg already had a `.value`, deal with it the way we do generally, a reassignment.

## on "$yparam = `bar(6)`"

$yparam -> {
    pattern = `bar($yarg)`
    value = `bar(6)`
}

The `.pattern` is not just a variable, so there is no obvious way to propagate the assignment to it.
I would actually fail with error in this case.

Even if they look the same, `bar(6)` and `bar(6)` are not truly related.

If one wants to do this, they'll call `foo(xparam = $xarg, yparam = $yarg)` and then process `$yarg` further.


*/
