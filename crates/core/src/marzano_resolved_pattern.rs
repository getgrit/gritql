use crate::{
    binding::Binding,
    constant::Constant,
    marzano_binding::MarzanoBinding,
    pattern::{
        accessor::Accessor,
        code_snippet::CodeSnippet,
        container::PatternOrResolved,
        dynamic_snippet::{DynamicPattern, DynamicSnippet, DynamicSnippetPart},
        functions::GritCall,
        list_index::ListIndex,
        paths::absolutize,
        patterns::{Pattern, PatternName},
        resolved_pattern::{ResolvedPattern, ResolvedSnippet},
        state::{FilePtr, FileRegistry, State},
        MarzanoContext,
    },
    problem::{Effect, EffectKind, MarzanoQueryContext},
};
use anyhow::{anyhow, bail, Result};
use grit_util::CodeRange;
use im::{vector, Vector};
use marzano_language::language::{FieldId, Language};
use marzano_util::{
    analysis_logs::AnalysisLogs, node_with_source::NodeWithSource, position::Range,
};
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    path::Path,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedFile<'a> {
    name: MarzanoResolvedPattern<'a>,
    body: MarzanoResolvedPattern<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum File<'a> {
    Resolved(Box<ResolvedFile<'a>>),
    Ptr(FilePtr),
}

impl<'a> File<'a> {
    pub(crate) fn name(&self, files: &FileRegistry<'a>) -> MarzanoResolvedPattern<'a> {
        match self {
            File::Resolved(resolved) => resolved.name.clone(),
            File::Ptr(ptr) => MarzanoResolvedPattern::from_path(&files.get_file(*ptr).name),
        }
    }

    pub(crate) fn absolute_path(
        &self,
        files: &FileRegistry<'a>,
    ) -> Result<MarzanoResolvedPattern<'a>> {
        match self {
            File::Resolved(resolved) => {
                let name = resolved.name.text(files)?;
                let absolute_path = absolutize(name.as_ref())?;
                Ok(MarzanoResolvedPattern::Constant(Constant::String(
                    absolute_path,
                )))
            }
            File::Ptr(ptr) => Ok(MarzanoResolvedPattern::from_path(
                &files.get_file(*ptr).absolute_path,
            )),
        }
    }

    pub(crate) fn body(&self, files: &FileRegistry<'a>) -> MarzanoResolvedPattern<'a> {
        match self {
            File::Resolved(resolved) => resolved.body.clone(),
            File::Ptr(ptr) => {
                let file = &files.get_file(*ptr);
                let range = file.tree.root_node().range().into();
                MarzanoResolvedPattern::from_range(range, &file.source)
            }
        }
    }

    pub(crate) fn binding(&self, files: &FileRegistry<'a>) -> MarzanoResolvedPattern<'a> {
        match self {
            File::Resolved(resolved) => resolved.body.clone(),
            File::Ptr(ptr) => {
                let file = &files.get_file(*ptr);
                let node = file.tree.root_node();
                MarzanoResolvedPattern::from_node(NodeWithSource::new(node, &file.source))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MarzanoResolvedPattern<'a> {
    Binding(Vector<MarzanoBinding<'a>>),
    Snippets(Vector<ResolvedSnippet<'a, MarzanoQueryContext>>),
    List(Vector<MarzanoResolvedPattern<'a>>),
    Map(BTreeMap<String, MarzanoResolvedPattern<'a>>),
    File(File<'a>),
    Files(Box<MarzanoResolvedPattern<'a>>),
    Constant(Constant),
}

impl<'a> MarzanoResolvedPattern<'a> {
    pub(crate) fn from_list(node: NodeWithSource<'a>, field_id: FieldId) -> Self {
        Self::from_binding(MarzanoBinding::List(node, field_id))
    }

    pub(crate) fn empty_field(node: NodeWithSource<'a>, field_id: FieldId) -> Self {
        Self::from_binding(MarzanoBinding::Empty(node, field_id))
    }

    pub(crate) fn from_path(path: &'a Path) -> Self {
        Self::from_binding(Binding::from_path(path))
    }

    fn to_snippets(&self) -> Result<Vector<ResolvedSnippet<'a, MarzanoQueryContext>>> {
        match self {
            MarzanoResolvedPattern::Snippets(snippets) => Ok(snippets.clone()),
            MarzanoResolvedPattern::Binding(bindings) => {
                Ok(vector![ResolvedSnippet::from_binding(
                    bindings
                        .last()
                        .ok_or_else(|| {
                            anyhow!("cannot create resolved snippet from unresolved binding")
                        })?
                        .to_owned(),
                )])
            }
            MarzanoResolvedPattern::List(elements) => {
                // merge separated by space
                let mut snippets = Vec::new();
                for pattern in elements {
                    snippets.extend(pattern.to_snippets()?);
                    snippets.push(ResolvedSnippet::Text(" ".into()));
                }
                snippets.pop();
                Ok(snippets.into())
            }
            MarzanoResolvedPattern::Map(map) => {
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
            MarzanoResolvedPattern::File(_) => Err(anyhow!(
                "cannot convert ResolvedPattern::File to ResolvedSnippet"
            )),
            MarzanoResolvedPattern::Files(_) => Err(anyhow!(
                "cannot convert ResolvedPattern::Files to ResolvedSnippet"
            )),
            MarzanoResolvedPattern::Constant(c) => {
                Ok(vector![ResolvedSnippet::Text(c.to_string().into(),)])
            }
        }
    }
}

impl<'a> ResolvedPattern<'a, MarzanoQueryContext> for MarzanoResolvedPattern<'a> {
    fn extend(
        &mut self,
        mut with: MarzanoResolvedPattern<'a>,
        effects: &mut Vector<Effect<'a, MarzanoQueryContext>>,
        language: &impl Language,
    ) -> Result<()> {
        match self {
            MarzanoResolvedPattern::Binding(bindings) => {
                let new_effects: Result<Vec<Effect<MarzanoQueryContext>>> = bindings
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
            MarzanoResolvedPattern::Snippets(snippets) => {
                match with {
                    MarzanoResolvedPattern::Snippets(with_snippets) => {
                        snippets.extend(with_snippets);
                    }
                    MarzanoResolvedPattern::Binding(binding) => {
                        let binding = binding
                            .last()
                            .ok_or_else(|| anyhow!("cannot extend with empty binding"))?;
                        snippets.push_back(ResolvedSnippet::Binding(binding.clone()));
                    }
                    MarzanoResolvedPattern::List(_) => {
                        return Err(anyhow!("cannot extend ResolvedPattern::Snippet with List"))
                    }
                    MarzanoResolvedPattern::File(_) => {
                        return Err(anyhow!("cannot extend ResolvedPattern::Snippet with File"))
                    }
                    MarzanoResolvedPattern::Files(_) => {
                        return Err(anyhow!("cannot extend ResolvedPattern::Snippet with Files"))
                    }
                    MarzanoResolvedPattern::Map(_) => {
                        return Err(anyhow!("cannot extend ResolvedPattern::Snippet with Map"))
                    }
                    MarzanoResolvedPattern::Constant(c) => {
                        snippets.push_back(ResolvedSnippet::Text(c.to_string().into()));
                    }
                }
                Ok(())
            }
            // do we want to auto flattern?
            // for now not since don't know what shape we want,
            // but probably will soon
            MarzanoResolvedPattern::List(lst) => {
                lst.push_back(with);
                Ok(())
            }
            MarzanoResolvedPattern::File(_) => Err(anyhow!("cannot extend ResolvedPattern::File")),
            MarzanoResolvedPattern::Files(_) => {
                Err(anyhow!("cannot extend ResolvedPattern::Files"))
            }
            MarzanoResolvedPattern::Map(_) => Err(anyhow!("cannot extend ResolvedPattern::Map")),
            MarzanoResolvedPattern::Constant(Constant::Integer(i)) => {
                if let MarzanoResolvedPattern::Constant(Constant::Integer(j)) = with {
                    *i += j;
                    Ok(())
                } else {
                    Err(anyhow!(
                        "can only extend Constant::Integer with another Constant::Integer"
                    ))
                }
            }
            MarzanoResolvedPattern::Constant(Constant::Float(x)) => {
                if let MarzanoResolvedPattern::Constant(Constant::Float(y)) = with {
                    *x += y;
                    Ok(())
                } else {
                    Err(anyhow!(
                        "can only extend Constant::Float with another Constant::Float"
                    ))
                }
            }
            MarzanoResolvedPattern::Constant(_) => {
                Err(anyhow!("cannot extend ResolvedPattern::Constant"))
            }
        }
    }

    fn position(&self) -> Option<Range> {
        if let MarzanoResolvedPattern::Binding(binding) = self {
            if let Some(binding) = binding.last() {
                return binding.position();
            }
        }
        None
    }

    fn from_binding(binding: MarzanoBinding<'a>) -> Self {
        Self::Binding(vector![binding])
    }

    fn from_constant(constant: Constant) -> Self {
        Self::Constant(constant)
    }

    fn from_constant_binding(constant: &'a Constant) -> Self {
        Self::from_binding(Binding::from_constant(constant))
    }

    fn from_node(node: NodeWithSource<'a>) -> Self {
        Self::from_binding(Binding::from_node(node))
    }

    fn from_range(range: Range, src: &'a str) -> Self {
        Self::from_binding(Binding::from_range(range, src))
    }

    fn from_string(string: String) -> Self {
        Self::Snippets(vector![ResolvedSnippet::Text(string.into())])
    }

    fn from_resolved_snippet(snippet: ResolvedSnippet<'a, MarzanoQueryContext>) -> Self {
        Self::Snippets(vector![snippet])
    }

    fn get_binding(&self) -> Option<&MarzanoBinding<'a>> {
        if let MarzanoResolvedPattern::Binding(bindings) = self {
            bindings.last()
        } else {
            None
        }
    }

    fn from_dynamic_snippet(
        snippet: &'a DynamicSnippet,
        state: &mut State<'a, MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
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

    fn from_dynamic_pattern(
        pattern: &'a DynamicPattern<MarzanoQueryContext>,
        state: &mut State<'a, MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
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

    fn from_accessor(
        accessor: &'a Accessor<MarzanoQueryContext>,
        state: &mut State<'a, MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        match accessor.get(state)? {
            Some(PatternOrResolved::Pattern(pattern)) => {
                MarzanoResolvedPattern::from_pattern(pattern, state, context, logs)
            }
            Some(PatternOrResolved::ResolvedBinding(resolved)) => Ok(resolved),
            Some(PatternOrResolved::Resolved(resolved)) => Ok(resolved.clone()),
            None => Ok(MarzanoResolvedPattern::from_constant_binding(
                &Constant::Undefined,
            )),
        }
    }

    fn from_list_index(
        index: &'a ListIndex<MarzanoQueryContext>,
        state: &mut State<'a, MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        match index.get(state)? {
            Some(PatternOrResolved::Pattern(pattern)) => {
                MarzanoResolvedPattern::from_pattern(pattern, state, context, logs)
            }
            Some(PatternOrResolved::ResolvedBinding(resolved)) => Ok(resolved),
            Some(PatternOrResolved::Resolved(resolved)) => Ok(resolved.clone()),
            None => Ok(MarzanoResolvedPattern::from_constant_binding(
                &Constant::Undefined,
            )),
        }
    }

    fn from_pattern(
        pattern: &'a Pattern<MarzanoQueryContext>,
        state: &mut State<'a, MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
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
            Pattern::IntConstant(int) => Ok(MarzanoResolvedPattern::Constant(Constant::Integer(
                int.value,
            ))),
            Pattern::FloatConstant(double) => Ok(MarzanoResolvedPattern::Constant(
                Constant::Float(double.value),
            )),
            Pattern::BooleanConstant(bool) => Ok(MarzanoResolvedPattern::Constant(
                Constant::Boolean(bool.value),
            )),
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
                let name = MarzanoResolvedPattern::from_pattern(name, state, context, logs)?;
                let name = name.text(&state.files)?;
                let name = MarzanoResolvedPattern::Constant(Constant::String(name.to_string()));
                let body = MarzanoResolvedPattern::from_pattern(body, state, context, logs)?;
                // todo: replace GENERATED_SOURCE with a computed source once linearization and
                // on-the-fly rewrites are in place
                Ok(MarzanoResolvedPattern::File(File::Resolved(Box::new(
                    ResolvedFile { name, body },
                ))))
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

    fn linearized_text(
        &self,
        language: &impl Language,
        effects: &[Effect<'a, MarzanoQueryContext>],
        files: &FileRegistry<'a>,
        memo: &mut HashMap<CodeRange, Option<String>>,
        should_pad_snippet: bool,
        logs: &mut AnalysisLogs,
    ) -> Result<Cow<'a, str>> {
        match self {
            // if whitespace is significant we need to distribute indentations
            // across lines within the snippet
            MarzanoResolvedPattern::Snippets(snippets) => {
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
                        padding = snippet.padding(files)?;
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
            MarzanoResolvedPattern::List(list) => Ok(list
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
            MarzanoResolvedPattern::Map(map) => Ok(("{".to_string()
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
            MarzanoResolvedPattern::Binding(binding) => Ok(binding
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
            MarzanoResolvedPattern::File(file) => Ok(format!(
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
            MarzanoResolvedPattern::Files(_files) => {
                bail!("cannot linearize files pattern, not implemented yet");
            }
            // unsure if this is correct, taken from text
            MarzanoResolvedPattern::Constant(c) => Ok(c.to_string().into()),
        }
    }

    fn float(&self, state: &FileRegistry<'a>) -> Result<f64> {
        match self {
            MarzanoResolvedPattern::Constant(c) => match c {
                Constant::Float(d) => Ok(*d),
                Constant::Integer(i) => Ok(*i as f64),
                Constant::String(s) => Ok(s.parse::<f64>()?),
                Constant::Boolean(_) | Constant::Undefined => Err(anyhow!("Cannot convert constant to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")),
            },
            MarzanoResolvedPattern::Snippets(s) => {
                let text = s
                    .iter()
                    .map(|snippet| snippet.text(state))
                    .collect::<Result<Vec<_>>>()?
                    .join("");
                text.parse::<f64>().map_err(|_| {
                    anyhow!("Failed to convert snippet to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")
                })
            }
            MarzanoResolvedPattern::Binding(binding) => {
                let text = binding
                    .last()
                    .ok_or_else(|| anyhow!("cannot grab text of resolved_pattern with no binding"))?
                    .text();
                text.parse::<f64>().map_err(|_| {
                    anyhow!("Failed to convert binding to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")
                })
            }
            MarzanoResolvedPattern::List(_) | MarzanoResolvedPattern::Map(_) | MarzanoResolvedPattern::File(_) | MarzanoResolvedPattern::Files(_) => Err(anyhow!("Cannot convert type to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")),
        }
    }

    fn matches_undefined(&self) -> bool {
        match self {
            MarzanoResolvedPattern::Binding(b) => b
                .last()
                .and_then(Binding::as_constant)
                .map_or(false, Constant::is_undefined),
            MarzanoResolvedPattern::Constant(Constant::Undefined) => true,
            MarzanoResolvedPattern::Constant(_)
            | MarzanoResolvedPattern::Snippets(_)
            | MarzanoResolvedPattern::List(_)
            | MarzanoResolvedPattern::Map(_)
            | MarzanoResolvedPattern::File(_)
            | MarzanoResolvedPattern::Files(_) => false,
        }
    }

    // should we instead return an Option?
    fn text(&self, state: &FileRegistry<'a>) -> Result<Cow<'a, str>> {
        match self {
            MarzanoResolvedPattern::Snippets(snippets) => Ok(snippets
                .iter()
                .map(|snippet| snippet.text(state))
                .collect::<Result<Vec<_>>>()?
                .join("")
                .into()),
            MarzanoResolvedPattern::List(list) => Ok(list
                .iter()
                .map(|pattern| pattern.text(state))
                .collect::<Result<Vec<_>>>()?
                .join(",")
                .into()),
            MarzanoResolvedPattern::Map(map) => Ok(("{".to_string()
                + &map
                    .iter()
                    .map(|(key, value)| {
                        format!(
                            "\"{}\": {}",
                            key,
                            value.text(state).expect("failed to get text of map value")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
                + "}")
                .into()),
            MarzanoResolvedPattern::Binding(binding) => Ok(binding
                .last()
                .ok_or_else(|| anyhow!("cannot grab text of resolved_pattern with no binding"))?
                .text()
                .into()),
            MarzanoResolvedPattern::File(file) => Ok(format!(
                "{}:\n{}",
                file.name(state).text(state)?,
                file.body(state).text(state)?
            )
            .into()),
            MarzanoResolvedPattern::Files(files) => files.text(state),
            MarzanoResolvedPattern::Constant(constant) => Ok(constant.to_string().into()),
        }
    }

    fn normalize_insert(
        &mut self,
        binding: &MarzanoBinding<'a>,
        is_first: bool,
        language: &impl Language,
    ) -> Result<()> {
        let MarzanoResolvedPattern::Snippets(ref mut snippets) = self else {
            return Ok(());
        };
        let Some(ResolvedSnippet::Text(text)) = snippets.front() else {
            return Ok(());
        };
        if let Some(padding) = binding.get_insertion_padding(text, is_first, language) {
            if padding.chars().next() != binding.text().chars().last() {
                snippets.push_front(ResolvedSnippet::Text(padding.into()));
            }
        }
        Ok(())
    }

    fn is_truthy(&self, state: &mut State<'a, MarzanoQueryContext>) -> Result<bool> {
        let truthiness = match self {
            Self::Binding(bindings) => bindings.last().map_or(false, Binding::is_truthy),
            Self::List(elements) => !elements.is_empty(),
            Self::Map(map) => !map.is_empty(),
            Self::Constant(c) => c.is_truthy(),
            Self::Snippets(s) => {
                if let Some(s) = s.last() {
                    s.is_truthy(state)?
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

fn extract_file_pointer(file: &File) -> Option<FilePtr> {
    match file {
        File::Resolved(_) => None,
        File::Ptr(ptr) => Some(*ptr),
    }
}

fn handle_files<'a>(files_list: &MarzanoResolvedPattern<'a>) -> Option<Vec<FilePtr>> {
    if let MarzanoResolvedPattern::List(files) = files_list {
        files
            .iter()
            .map(|r| {
                if let ResolvedPattern::File(File::Ptr(ptr)) = r {
                    Some(*ptr)
                } else {
                    None
                }
            })
            .collect()
    } else {
        None
    }
}

pub(crate) fn extract_file_pointers<'a>(
    binding: &MarzanoResolvedPattern<'a>,
) -> Option<Vec<FilePtr>> {
    match binding {
        MarzanoResolvedPattern::Binding(_) => None,
        MarzanoResolvedPattern::Snippets(_) => None,
        MarzanoResolvedPattern::List(_) => handle_files(binding),
        MarzanoResolvedPattern::Map(_) => None,
        MarzanoResolvedPattern::File(file) => extract_file_pointer(file).map(|f| vec![f]),
        MarzanoResolvedPattern::Files(files) => handle_files(files),
        MarzanoResolvedPattern::Constant(_) => None,
    }
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
