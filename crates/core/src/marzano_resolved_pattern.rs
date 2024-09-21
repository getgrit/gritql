use crate::{
    marzano_binding::MarzanoBinding, marzano_code_snippet::MarzanoCodeSnippet,
    marzano_context::MarzanoContext, paths::absolutize, problem::MarzanoQueryContext,
};
use grit_pattern_matcher::pattern::Matcher;
use grit_pattern_matcher::{
    binding::Binding,
    constant::Constant,
    context::ExecContext,
    effects::Effect,
    pattern::{
        to_unsigned, Accessor, DynamicPattern, DynamicSnippet, DynamicSnippetPart, File, FilePtr,
        FileRegistry, GritCall, ListIndex, Pattern, PatternName, PatternOrResolved, ResolvedFile,
        ResolvedPattern, ResolvedSnippet, State,
    },
};
use grit_util::{
    error::{GritPatternError, GritResult},
    AnalysisLogs, Ast, AstNode, CodeRange, EffectKind, Range,
};
use im::{vector, Vector};
use marzano_language::{language::FieldId, target_language::TargetLanguage};
use marzano_util::node_with_source::NodeWithSource;
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    path::Path,
};

#[derive(Debug, Clone, PartialEq)]
pub enum MarzanoResolvedPattern<'a> {
    Binding(Vector<MarzanoBinding<'a>>),
    Snippets(Vector<ResolvedSnippet<'a, MarzanoQueryContext>>),
    List(Vector<MarzanoResolvedPattern<'a>>),
    Map(BTreeMap<String, MarzanoResolvedPattern<'a>>),
    File(MarzanoFile<'a>),
    Files(Box<MarzanoResolvedPattern<'a>>),
    Constant(Constant),
}

impl<'a> MarzanoResolvedPattern<'a> {
    pub(crate) fn from_empty_binding(node: NodeWithSource<'a>, field_id: FieldId) -> Self {
        Self::from_binding(MarzanoBinding::Empty(node, field_id))
    }

    pub(crate) fn from_list_binding(node: NodeWithSource<'a>, field_id: FieldId) -> Self {
        Self::from_binding(MarzanoBinding::List(node, field_id))
    }

    /// Check if a pattern matches a provided pattern
    ///
    /// Note this leaks memory, so should only be used in short-lived programs
    #[allow(dead_code)]
    pub(crate) fn matches(
        &self,
        pattern: &Pattern<MarzanoQueryContext>,
        state: &mut State<'a, MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        let borrowed_pattern: &'static Pattern<MarzanoQueryContext> =
            Box::leak(Box::new(pattern.clone()));

        let matches = borrowed_pattern.execute(self, state, context, logs)?;
        Ok(matches)
    }

    fn to_snippets(&self) -> GritResult<Vector<ResolvedSnippet<'a, MarzanoQueryContext>>> {
        match self {
            MarzanoResolvedPattern::Snippets(snippets) => Ok(snippets.clone()),
            MarzanoResolvedPattern::Binding(bindings) => {
                Ok(vector![ResolvedSnippet::from_binding(
                    bindings
                        .last()
                        .ok_or_else(|| {
                            GritPatternError::new(
                                "cannot create resolved snippet from unresolved binding",
                            )
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
            MarzanoResolvedPattern::File(_) => Err(GritPatternError::new(
                "cannot convert ResolvedPattern::File to ResolvedSnippet",
            )),
            MarzanoResolvedPattern::Files(_) => Err(GritPatternError::new(
                "cannot convert ResolvedPattern::Files to ResolvedSnippet",
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
        language: &TargetLanguage,
    ) -> GritResult<()> {
        match self {
            Self::Binding(bindings) => {
                let new_effects: GritResult<Vec<Effect<MarzanoQueryContext>>> = bindings
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
                        let binding = binding.last().ok_or_else(|| {
                            GritPatternError::new("cannot extend with empty binding")
                        })?;
                        snippets.push_back(ResolvedSnippet::Binding(binding.clone()));
                    }
                    Self::List(_) => {
                        return Err(GritPatternError::new(
                            "cannot extend ResolvedPattern::Snippet with List",
                        ))
                    }
                    Self::File(_) => {
                        return Err(GritPatternError::new(
                            "cannot extend ResolvedPattern::Snippet with File",
                        ))
                    }
                    Self::Files(_) => {
                        return Err(GritPatternError::new(
                            "cannot extend ResolvedPattern::Snippet with Files",
                        ))
                    }
                    Self::Map(_) => {
                        return Err(GritPatternError::new(
                            "cannot extend ResolvedPattern::Snippet with Map",
                        ))
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
            Self::File(_) => Err(GritPatternError::new("cannot extend ResolvedPattern::File")),
            Self::Files(_) => Err(GritPatternError::new(
                "cannot extend ResolvedPattern::Files",
            )),
            Self::Map(_) => Err(GritPatternError::new("cannot extend ResolvedPattern::Map")),
            Self::Constant(Constant::Integer(i)) => {
                if let Self::Constant(Constant::Integer(j)) = with {
                    *i += j;
                    Ok(())
                } else {
                    Err(GritPatternError::new(
                        "can only extend Constant::Integer with another Constant::Integer",
                    ))
                }
            }
            Self::Constant(Constant::Float(x)) => {
                if let Self::Constant(Constant::Float(y)) = with {
                    *x += y;
                    Ok(())
                } else {
                    Err(GritPatternError::new(
                        "can only extend Constant::Float with another Constant::Float",
                    ))
                }
            }
            Self::Constant(_) => Err(GritPatternError::new(
                "cannot extend ResolvedPattern::Constant",
            )),
        }
    }

    fn position(&self, language: &TargetLanguage) -> Option<Range> {
        if let MarzanoResolvedPattern::Binding(binding) = self {
            if let Some(binding) = binding.last() {
                return binding.position(language);
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

    fn from_file_pointer(file: FilePtr) -> Self {
        Self::File(MarzanoFile::Ptr(file))
    }

    fn from_files(files: Self) -> Self {
        Self::Files(Box::new(files))
    }

    fn from_list_parts(parts: impl Iterator<Item = Self>) -> Self {
        Self::List(parts.collect())
    }

    fn from_string(string: String) -> Self {
        Self::Snippets(vector![ResolvedSnippet::Text(string.into())])
    }

    fn from_resolved_snippet(snippet: ResolvedSnippet<'a, MarzanoQueryContext>) -> Self {
        Self::Snippets(vector![snippet])
    }

    fn get_bindings(&self) -> Option<impl Iterator<Item = MarzanoBinding<'a>>> {
        if let Self::Binding(bindings) = self {
            Some(bindings.iter().cloned())
        } else {
            None
        }
    }

    fn get_file(&self) -> Option<&MarzanoFile<'a>> {
        if let Self::File(file) = self {
            Some(file)
        } else {
            None
        }
    }

    fn get_file_pointers(&self) -> Option<Vec<FilePtr>> {
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

    fn get_files(&self) -> Option<&Self> {
        if let Self::Files(files) = self {
            Some(files)
        } else {
            None
        }
    }

    fn get_last_binding(&self) -> Option<&MarzanoBinding<'a>> {
        if let Self::Binding(bindings) = self {
            bindings.last()
        } else {
            None
        }
    }

    fn get_list_item_at(&self, index: isize) -> Option<&Self> {
        if let Self::List(items) = self {
            to_unsigned(index, items.len()).and_then(|index| items.get(index))
        } else {
            None
        }
    }

    fn get_list_item_at_mut(&mut self, index: isize) -> Option<&mut Self> {
        if let Self::List(items) = self {
            to_unsigned(index, items.len()).and_then(|index| items.get_mut(index))
        } else {
            None
        }
    }

    fn get_list_items(&self) -> Option<impl Iterator<Item = &Self>> {
        if let Self::List(items) = self {
            Some(items.iter())
        } else {
            None
        }
    }

    fn get_list_binding_items(&self) -> Option<impl Iterator<Item = Self> + Clone> {
        self.get_last_binding()
            .and_then(Binding::list_items)
            .map(|items| items.map(MarzanoResolvedPattern::from_node_binding))
    }

    fn get_map(&self) -> Option<&BTreeMap<String, Self>> {
        if let Self::Map(map) = self {
            Some(map)
        } else {
            None
        }
    }

    fn get_map_mut(&mut self) -> Option<&mut BTreeMap<String, Self>> {
        if let Self::Map(map) = self {
            Some(map)
        } else {
            None
        }
    }

    fn get_snippets(
        &self,
    ) -> Option<impl Iterator<Item = ResolvedSnippet<'a, MarzanoQueryContext>>> {
        if let Self::Snippets(snippets) = self {
            Some(snippets.iter().cloned())
        } else {
            None
        }
    }

    fn is_binding(&self) -> bool {
        matches!(self, Self::Binding(_))
    }

    fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }

    fn push_binding(&mut self, binding: MarzanoBinding<'a>) -> GritResult<()> {
        let Self::Binding(bindings) = self else {
            return Err(GritPatternError::new("can only push to bindings"));
        };

        bindings.push_back(binding);
        Ok(())
    }

    fn set_list_item_at_mut(&mut self, index: isize, value: Self) -> GritResult<bool> {
        let Self::List(items) = self else {
            return Err(GritPatternError::new("can only set items on a list"));
        };

        let Some(index) = to_unsigned(index, items.len()) else {
            return Ok(false);
        };

        items.insert(index, value);
        Ok(true)
    }

    fn from_dynamic_snippet(
        snippet: &'a DynamicSnippet,
        state: &mut State<'a, MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<Self> {
        let mut parts = Vec::new();
        for part in &snippet.parts {
            match part {
                DynamicSnippetPart::String(string) => {
                    parts.push(ResolvedSnippet::Text(string.into()));
                }
                DynamicSnippetPart::Variable(var) => {
                    let content = &state.bindings[var.try_scope().unwrap().into()].last().unwrap()[var.try_index().unwrap().into()];
                    let name = &content.name;
                    // feels weird not sure if clone is correct
                    let value = if let Some(value) = &content.value {
                        value.clone()
                    } else if let Some(pattern) = content.pattern {
                        Self::from_pattern(pattern, state, context, logs)?
                    } else {
                        return Err(GritPatternError::new(format!(
                            "cannot create resolved snippet from unresolved variable {}",
                            name,
                        )));
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
    ) -> GritResult<Self> {
        match pattern {
            DynamicPattern::Variable(var) => {
                let content = &state.bindings[var.try_scope().unwrap().into()].last().unwrap()[var.try_index().unwrap().into()];
                let name = &content.name;
                // feels weird not sure if clone is correct
                if let Some(value) = &content.value {
                    Ok(value.clone())
                } else if let Some(pattern) = content.pattern {
                    Self::from_pattern(pattern, state, context, logs)
                } else {
                    return Err(GritPatternError::new(format!(
                        "cannot create resolved snippet from unresolved variable {}",
                        name
                    )));
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
    ) -> GritResult<Self> {
        match accessor.get(state, context.language())? {
            Some(PatternOrResolved::Pattern(pattern)) => {
                Self::from_pattern(pattern, state, context, logs)
            }
            Some(PatternOrResolved::ResolvedBinding(resolved)) => Ok(resolved),
            Some(PatternOrResolved::Resolved(resolved)) => Ok(resolved.clone()),
            None => Ok(Self::from_constant_binding(&Constant::Undefined)),
        }
    }

    fn from_list_index(
        index: &'a ListIndex<MarzanoQueryContext>,
        state: &mut State<'a, MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<Self> {
        match index.get(state, context.language())? {
            Some(PatternOrResolved::Pattern(pattern)) => {
                Self::from_pattern(pattern, state, context, logs)
            }
            Some(PatternOrResolved::ResolvedBinding(resolved)) => Ok(resolved),
            Some(PatternOrResolved::Resolved(resolved)) => Ok(resolved.clone()),
            None => Ok(Self::from_constant_binding(&Constant::Undefined)),
        }
    }

    fn from_pattern(
        pattern: &'a Pattern<MarzanoQueryContext>,
        state: &mut State<'a, MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<Self> {
        match pattern {
            Pattern::Dynamic(pattern) => Self::from_dynamic_pattern(pattern, state, context, logs),
            Pattern::CodeSnippet(MarzanoCodeSnippet {
                dynamic_snippet: Some(pattern),
                ..
            }) => Self::from_dynamic_pattern(pattern, state, context, logs),
            Pattern::CallBuiltIn(built_in) => built_in.call(state, context, logs),
            Pattern::CallFunction(func) => func.call(state, context, logs),
            Pattern::CallForeignFunction(func) => func.call(state, context, logs),
            Pattern::CallbackPattern(callback) => Err(GritPatternError::new(format!(
                "cannot make resolved pattern from callback pattern {}",
                callback.name()
            ))),
            Pattern::StringConstant(string) => Ok(Self::Snippets(vector![ResolvedSnippet::Text(
                (&string.text).into(),
            )])),
            Pattern::IntConstant(int) => Ok(Self::Constant(Constant::Integer(int.value))),
            Pattern::FloatConstant(double) => Ok(Self::Constant(Constant::Float(double.value))),
            Pattern::BooleanConstant(bool) => Ok(Self::Constant(Constant::Boolean(bool.value))),
            Pattern::Variable(var) => {
                let content = &state.bindings[var.try_scope().unwrap().into()].last().unwrap()[var.try_index().unwrap().into()];
                let name = &content.name;
                // feels weird not sure if clone is correct
                if let Some(value) = &content.value {
                    Ok(value.clone())
                } else if let Some(pattern) = content.pattern {
                    Self::from_pattern(pattern, state, context, logs)
                } else {
                    return Err(GritPatternError::new(format!(
                        "cannot create resolved snippet from unresolved variable {}",
                        name
                    )));
                }
            }
            Pattern::List(list) => list
                .patterns
                .iter()
                .map(|pattern| Self::from_pattern(pattern, state, context, logs))
                .collect::<GritResult<Vector<_>>>()
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
                .collect::<GritResult<BTreeMap<_, _>>>()
                .map(Self::Map),
            Pattern::Accessor(accessor) => Self::from_accessor(accessor, state, context, logs),
            Pattern::File(file_pattern) => {
                let name = &file_pattern.name;
                let body = &file_pattern.body;
                let name = Self::from_pattern(name, state, context, logs)?;
                let name = name.text(&state.files, context.language)?;
                let name = Self::Constant(Constant::String(name.to_string()));
                let body = Self::from_pattern(body, state, context, logs)?;
                // todo: replace GENERATED_SOURCE with a computed source once linearization and
                // on-the-fly rewrites are in place
                Ok(Self::File(MarzanoFile::Resolved(Box::new(ResolvedFile {
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
            | Pattern::Sequential(_) => Err(GritPatternError::new(format!(
                "cannot make resolved pattern from arbitrary pattern {}",
                pattern.name()
            ))),
        }
    }

    fn linearized_text(
        &self,
        language: &TargetLanguage,
        effects: &[Effect<'a, MarzanoQueryContext>],
        files: &FileRegistry<'a, MarzanoQueryContext>,
        memo: &mut HashMap<CodeRange, Option<String>>,
        should_pad_snippet: bool,
        logs: &mut AnalysisLogs,
    ) -> GritResult<Cow<'a, str>> {
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
                        .collect::<GritResult<Vec<_>>>()?
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
                .collect::<GritResult<Vec<_>>>()?
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
                    .collect::<GritResult<Vec<_>>>()?
                    .iter()
                    .map(|(key, value)| format!("\"{}\": {}", key, value))
                    .collect::<Vec<_>>()
                    .join(", ")
                + "}")
                .into()),
            // might have to handle differently for ResolvedPattern::List containing indent followed by binding
            Self::Binding(binding) => Ok(binding
                .last()
                .ok_or_else(|| {
                    GritPatternError::new("cannot grab text of resolved_pattern with no binding")
                })?
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
            Self::Files(_files) => Err(GritPatternError::new(
                "cannot linearize files pattern, not implemented yet",
            )),
            // unsure if this is correct, taken from text
            Self::Constant(c) => Ok(c.to_string().into()),
        }
    }

    fn float(
        &self,
        state: &FileRegistry<'a, MarzanoQueryContext>,
        language: &TargetLanguage,
    ) -> GritResult<f64> {
        match self {
            Self::Constant(c) => match c {
                Constant::Float(d) => Ok(*d),
                Constant::Integer(i) => Ok(*i as f64),
                Constant::String(s) => Ok(s.parse::<f64>()?),
                Constant::Boolean(_) | Constant::Undefined => Err(GritPatternError::new("Cannot convert constant to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")),
            },
            Self::Snippets(s) => {
                let text = s
                    .iter()
                    .map(|snippet| snippet.text(state, language))
                    .collect::<GritResult<Vec<_>>>()?
                    .join("");
                text.parse::<f64>().map_err(|_| {
                    GritPatternError::new("Failed to convert snippet to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")
                })
            }
            Self::Binding(binding) => {
                let text = binding
                    .last()
                    .ok_or_else(|| GritPatternError::new("cannot grab text of resolved_pattern with no binding"))?
                    .text(language)?;
                text.parse::<f64>().map_err(|_| {
                    GritPatternError::new("Failed to convert binding to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")
                })
            }
            Self::List(_) | Self::Map(_) | Self::File(_) | Self::Files(_) => Err(GritPatternError::new("Cannot convert type to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")),
        }
    }

    fn matches_undefined(&self) -> bool {
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

    fn matches_false_or_undefined(&self) -> bool {
        // should this match a binding to the constant `false` as well?
        matches!(self, Self::Constant(Constant::Boolean(false))) || self.matches_undefined()
    }

    // should we instead return an Option?
    fn text(
        &self,
        state: &FileRegistry<'a, MarzanoQueryContext>,
        language: &TargetLanguage,
    ) -> GritResult<Cow<'a, str>> {
        match self {
            Self::Snippets(snippets) => Ok(snippets
                .iter()
                .map(|snippet| snippet.text(state, language))
                .collect::<GritResult<Vec<_>>>()?
                .join("")
                .into()),
            Self::List(list) => Ok(list
                .iter()
                .map(|pattern| pattern.text(state, language))
                .collect::<GritResult<Vec<_>>>()?
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
                .ok_or_else(|| {
                    GritPatternError::new("cannot grab text of resolved_pattern with no binding")
                })?
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

    fn normalize_insert(
        &mut self,
        binding: &MarzanoBinding<'a>,
        is_first: bool,
        language: &TargetLanguage,
    ) -> GritResult<()> {
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

    fn is_truthy(
        &self,
        state: &mut State<'a, MarzanoQueryContext>,
        language: &TargetLanguage,
    ) -> GritResult<bool> {
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

#[derive(Debug, Clone, PartialEq)]
pub enum MarzanoFile<'a> {
    Resolved(Box<ResolvedFile<'a, MarzanoQueryContext>>),
    Ptr(FilePtr),
}

impl<'a> File<'a, MarzanoQueryContext> for MarzanoFile<'a> {
    fn name(&self, files: &FileRegistry<'a, MarzanoQueryContext>) -> MarzanoResolvedPattern<'a> {
        match self {
            Self::Resolved(resolved) => resolved.name.clone(),
            Self::Ptr(ptr) => MarzanoResolvedPattern::from_path_binding(files.get_file_name(*ptr)),
        }
    }

    fn absolute_path(
        &self,
        files: &FileRegistry<'a, MarzanoQueryContext>,
        language: &TargetLanguage,
    ) -> GritResult<MarzanoResolvedPattern<'a>> {
        match self {
            Self::Resolved(resolved) => {
                let name = resolved.name.text(files, language)?;
                let absolute_path = absolutize(Path::new(name.as_ref()))?;
                Ok(ResolvedPattern::from_constant(Constant::String(
                    absolute_path.to_string_lossy().to_string(),
                )))
            }
            Self::Ptr(ptr) => Ok(ResolvedPattern::from_path_binding(
                files.get_absolute_path(*ptr)?,
            )),
        }
    }

    fn body(&self, files: &FileRegistry<'a, MarzanoQueryContext>) -> MarzanoResolvedPattern<'a> {
        match self {
            Self::Resolved(resolved) => resolved.body.clone(),
            Self::Ptr(ptr) => {
                let file = &files.get_file_owner(*ptr);
                let root = file.tree.root_node();
                let range = root.byte_range();
                ResolvedPattern::from_range_binding(range, &file.tree.source)
            }
        }
    }

    fn binding(&self, files: &FileRegistry<'a, MarzanoQueryContext>) -> MarzanoResolvedPattern<'a> {
        match self {
            Self::Resolved(resolved) => resolved.body.clone(),
            Self::Ptr(ptr) => {
                let file = &files.get_file_owner(*ptr);
                ResolvedPattern::from_node_binding(file.tree.root_node())
            }
        }
    }
}

fn extract_file_pointer(file: &MarzanoFile) -> Option<FilePtr> {
    match file {
        MarzanoFile::Resolved(_) => None,
        MarzanoFile::Ptr(ptr) => Some(*ptr),
    }
}

fn handle_files(files_list: &MarzanoResolvedPattern) -> Option<Vec<FilePtr>> {
    if let MarzanoResolvedPattern::List(files) = files_list {
        files
            .iter()
            .map(|r| {
                if let MarzanoResolvedPattern::File(MarzanoFile::Ptr(ptr)) = r {
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
