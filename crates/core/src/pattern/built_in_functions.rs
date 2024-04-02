use crate::{binding::Constant, context::Context};
use itertools::Itertools;
use marzano_util::analysis_logs::AnalysisLogs;
use rand::prelude::SliceRandom;
use rand::Rng;

// todo we can probably use a macro to generate a function that takes a vec and
// and calls the input function with the vec args unpacked.
use super::{
    functions::GritCall,
    paths::resolve,
    patterns::{Name, Pattern},
    resolved_pattern::{
        patterns_to_resolved, JoinFn, LazyBuiltIn, ResolvedPattern, ResolvedSnippet,
    },
    variable::get_absolute_file_name,
    MarzanoContext, State,
};
use anyhow::{anyhow, bail, Result};
use im::vector;
use im::Vector;
use marzano_language::language::Language;

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct CallBuiltIn {
    pub(crate) index: usize,
    pub(crate) args: Vec<Option<Pattern>>,
}

impl CallBuiltIn {
    pub fn new(index: usize, args: Vec<Option<Pattern>>) -> Self {
        Self { index, args }
    }

    pub(crate) fn from_args(
        mut args: BTreeMap<String, Pattern>,
        built_ins: &BuiltIns,
        index: usize,
        lang: &impl Language,
    ) -> Result<Self> {
        let params = &built_ins.0[index].params;
        let mut pattern_params = Vec::with_capacity(args.len());
        for param in params.iter() {
            match args.remove(&(lang.metavariable_prefix().to_owned() + param)) {
                Some(p) => pattern_params.push(Some(p)),
                None => pattern_params.push(None),
            }
        }
        Ok(Self::new(index, pattern_params))
    }
}

impl GritCall for CallBuiltIn {
    fn call<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>> {
        context.call_built_in(self, context, state, logs)
    }
}

impl Name for CallBuiltIn {
    fn name(&self) -> &'static str {
        "CALL_BUILT_IN"
    }
}

// I think we may want to support both functions that return a borrowed value, and an owned value
// eg. capitalize returns an owned string_constant pattern, but unique would return a borrowed
// value.

type F = dyn for<'a> Fn(
        &'a [Option<Pattern>],
        &'a MarzanoContext<'a>,
        &mut State<'a>,
        &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>>
    + Send
    + Sync;

pub struct BuiltInFunction {
    pub name: &'static str,
    pub params: Vec<&'static str>,
    pub(crate) func: Box<F>,
}

impl BuiltInFunction {
    fn call<'a>(
        &self,
        args: &'a [Option<Pattern>],
        context: &'a MarzanoContext<'a>,
        state: &mut State<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>> {
        (self.func)(args, context, state, logs)
    }

    pub fn new(name: &'static str, params: Vec<&'static str>, func: Box<F>) -> Self {
        Self { name, params, func }
    }
}

impl std::fmt::Debug for BuiltInFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BuiltInFunction")
            .field("name", &self.name)
            .field("params", &self.params)
            .finish()
    }
}

#[derive(Debug)]
pub struct BuiltIns(Vec<BuiltInFunction>);

impl BuiltIns {
    pub(crate) fn call<'a>(
        &self,
        call: &'a CallBuiltIn,
        context: &'a MarzanoContext<'a>,
        state: &mut State<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>> {
        self.0[call.index].call(&call.args, context, state, logs)
    }

    pub fn extend_builtins(&mut self, other: BuiltIns) -> Result<()> {
        let self_name = self.0.iter().map(|b| &b.name).collect_vec();
        let other_name = other.0.iter().map(|b| &b.name).collect_vec();
        let repeats = self_name
            .iter()
            .filter(|n| other_name.contains(n))
            .collect_vec();
        if !repeats.is_empty() {
            let repeated_names = repeats
                .iter()
                .fold("".to_string(), |a, n| format!("{}{}, ", a, n));
            let repeated_names = repeated_names.strip_suffix(", ").unwrap();
            Err(anyhow!(
                "failed to extend builtins as collections had repeated definitions for: {}",
                repeated_names
            ))
        } else {
            self.0.extend(other.0);
            Ok(())
        }
    }

    pub(crate) fn get_built_ins(&self) -> &[BuiltInFunction] {
        &self.0
    }

    pub fn get_built_in_functions() -> BuiltIns {
        vec![
            BuiltInFunction::new("resolve", vec!["path"], Box::new(resolve_path_fn)),
            BuiltInFunction::new("capitalize", vec!["string"], Box::new(capitalize_fn)),
            BuiltInFunction::new("lowercase", vec!["string"], Box::new(lowercase_fn)),
            BuiltInFunction::new("uppercase", vec!["string"], Box::new(uppercase_fn)),
            BuiltInFunction::new("text", vec!["string"], Box::new(text_fn)),
            BuiltInFunction::new("trim", vec!["string", "trim_chars"], Box::new(trim_fn)),
            BuiltInFunction::new("join", vec!["list", "separator"], Box::new(join_fn)),
            BuiltInFunction::new("distinct", vec!["list"], Box::new(distinct_fn)),
            BuiltInFunction::new("length", vec!["target"], Box::new(length_fn)),
            BuiltInFunction::new("shuffle", vec!["list"], Box::new(shuffle_fn)),
            BuiltInFunction::new("random", vec!["floor", "ceiling"], Box::new(random_fn)),
            BuiltInFunction::new("split", vec!["string", "separator"], Box::new(split_fn)),
        ]
        .into()
    }
}

impl From<Vec<BuiltInFunction>> for BuiltIns {
    fn from(built_ins: Vec<BuiltInFunction>) -> Self {
        Self(built_ins)
    }
}

/// Turn an arbitrary path into a resolved and normalized absolute path
fn resolve_path_fn<'a>(
    args: &'a [Option<Pattern>],
    context: &'a MarzanoContext<'a>,
    state: &mut State<'a>,
    logs: &mut AnalysisLogs,
) -> Result<ResolvedPattern<'a>> {
    let args = patterns_to_resolved(args, state, context, logs)?;

    let current_file = get_absolute_file_name(state)?;
    let target_path = match &args[0] {
        Some(resolved_pattern) => resolved_pattern.text(&state.files)?,
        None => return Err(anyhow!("No path argument provided for resolve function")),
    };

    let resolved_path = resolve(target_path, current_file.into())?;

    Ok(ResolvedPattern::from_string(resolved_path))
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn capitalize_fn<'a>(
    args: &'a [Option<Pattern>],
    context: &'a MarzanoContext<'a>,
    state: &mut State<'a>,
    logs: &mut AnalysisLogs,
) -> Result<ResolvedPattern<'a>> {
    let args = patterns_to_resolved(args, state, context, logs)?;

    let s = match &args[0] {
        Some(resolved_pattern) => resolved_pattern.text(&state.files)?,
        None => return Err(anyhow!("No argument provided for capitalize function")),
    };
    Ok(ResolvedPattern::from_string(capitalize(&s)))
}

fn lowercase_fn<'a>(
    args: &'a [Option<Pattern>],
    context: &'a MarzanoContext<'a>,
    state: &mut State<'a>,
    logs: &mut AnalysisLogs,
) -> Result<ResolvedPattern<'a>> {
    let args = patterns_to_resolved(args, state, context, logs)?;

    let s = match &args[0] {
        Some(resolved_pattern) => resolved_pattern.text(&state.files)?,
        None => return Err(anyhow!("lowercase takes 1 argument")),
    };
    Ok(ResolvedPattern::from_string(s.to_lowercase()))
}

fn uppercase_fn<'a>(
    args: &'a [Option<Pattern>],
    context: &'a MarzanoContext<'a>,
    state: &mut State<'a>,
    logs: &mut AnalysisLogs,
) -> Result<ResolvedPattern<'a>> {
    let args = patterns_to_resolved(args, state, context, logs)?;

    let s = match &args[0] {
        Some(resolved_pattern) => resolved_pattern.text(&state.files)?,
        None => return Err(anyhow!("uppercase takes 1 argument")),
    };
    Ok(ResolvedPattern::from_string(s.to_uppercase()))
}

fn text_fn<'a>(
    args: &'a [Option<Pattern>],
    context: &'a MarzanoContext<'a>,
    state: &mut State<'a>,
    logs: &mut AnalysisLogs,
) -> Result<ResolvedPattern<'a>> {
    let args = patterns_to_resolved(args, state, context, logs)?;

    let s = match args.first() {
        Some(Some(resolved_pattern)) => resolved_pattern.text(&state.files)?,
        _ => return Err(anyhow!("text takes 1 argument")),
    };
    Ok(ResolvedPattern::from_string(s.to_string()))
}

fn trim_fn<'a>(
    args: &'a [Option<Pattern>],
    context: &'a MarzanoContext<'a>,
    state: &mut State<'a>,
    logs: &mut AnalysisLogs,
) -> Result<ResolvedPattern<'a>> {
    let args = patterns_to_resolved(args, state, context, logs)?;

    let trim_chars = match &args[1] {
        Some(resolved_pattern) => resolved_pattern.text(&state.files)?,
        None => return Err(anyhow!("trim takes 2 arguments: string and trim_chars")),
    };

    let s = match &args[0] {
        Some(resolved_pattern) => resolved_pattern.text(&state.files)?,
        None => return Err(anyhow!("trim takes 2 arguments: string and trim_chars")),
    };

    let trim_chars = trim_chars.chars().collect::<Vec<char>>();
    let trim_chars = trim_chars.as_slice();
    let s = s.trim_matches(trim_chars).to_string();
    Ok(ResolvedPattern::from_string(s))
}

fn split_fn<'a>(
    args: &'a [Option<Pattern>],
    context: &'a MarzanoContext<'a>,
    state: &mut State<'a>,
    logs: &mut AnalysisLogs,
) -> Result<ResolvedPattern<'a>> {
    let args = patterns_to_resolved(args, state, context, logs)?;

    let string = if let Some(string) = &args[0] {
        string.text(&state.files)?
    } else {
        bail!("split requires parameter string")
    };
    let separator = if let Some(separator) = &args[1] {
        separator.text(&state.files)?
    } else {
        bail!("split requires parameter separator")
    };
    let parts: Vector<ResolvedPattern> = string
        .split(&separator.as_ref())
        .map(|s| ResolvedPattern::Snippets(vector![ResolvedSnippet::Text(s.to_string().into())]))
        .collect();
    Ok(ResolvedPattern::List(parts))
}

fn random_fn<'a>(
    args: &'a [Option<Pattern>],
    context: &'a MarzanoContext<'a>,
    state: &mut State<'a>,
    logs: &mut AnalysisLogs,
) -> Result<ResolvedPattern<'a>> {
    let args = patterns_to_resolved(args, state, context, logs)?;

    match args.as_slice() {
        [Some(start), Some(end)] => {
            let start = start.text(&state.files)?;
            let end = end.text(&state.files)?;
            let start = start.parse::<i64>().unwrap();
            let end = end.parse::<i64>().unwrap();
            // Inclusive range
            let value = state.get_rng().gen_range(start..=end);
            Ok(ResolvedPattern::Constant(Constant::Integer(value)))
        }
        [Some(_), None] => {
            bail!("If you provide a start argument to random(), you must provide an end argument")
        }
        [None, Some(_)] => {
            bail!("If you provide an end argument to random(), you must provide a start argument")
        }
        [None, None] => {
            let value = state.get_rng().gen::<f64>();
            Ok(ResolvedPattern::Constant(Constant::Float(value)))
        }
        _ => bail!("random() takes 0 or 2 arguments"),
    }
}

fn join_fn<'a>(
    args: &'a [Option<Pattern>],
    context: &'a MarzanoContext<'a>,
    state: &mut State<'a>,
    logs: &mut AnalysisLogs,
) -> Result<ResolvedPattern<'a>> {
    let args = patterns_to_resolved(args, state, context, logs)?;

    let separator = &args[1];
    let separator = match separator {
        Some(resolved_pattern) => resolved_pattern.text(&state.files)?,
        None => return Err(anyhow!("trim takes 2 arguments: list and separator")),
    };

    let list = &args[0];
    let join = match list {
        Some(ResolvedPattern::List(list)) => {
            JoinFn::from_resolved(list.to_owned(), separator.to_string())
        }
        Some(ResolvedPattern::Binding(binding)) => binding
            .last()
            .and_then(|b| JoinFn::from_list_binding(b, separator.to_string()))
            .ok_or_else(|| anyhow!("join takes a list as the first argument"))?,
        _ => bail!("join takes a list as the first argument"),
    };
    let snippet = ResolvedSnippet::LazyFn(Box::new(LazyBuiltIn::Join(join)));
    Ok(ResolvedPattern::from_resolved_snippet(snippet))
}

fn distinct_fn<'a>(
    args: &'a [Option<Pattern>],
    context: &'a MarzanoContext<'a>,
    state: &mut State<'a>,
    logs: &mut AnalysisLogs,
) -> Result<ResolvedPattern<'a>> {
    let args = patterns_to_resolved(args, state, context, logs)?;

    let list = args.into_iter().next().unwrap();
    match list {
        Some(ResolvedPattern::List(list)) => {
            let mut unique_list = Vector::new();
            for item in list {
                if !unique_list.contains(&item) {
                    unique_list.push_back(item);
                }
            }
            Ok(ResolvedPattern::List(unique_list))
        }
        Some(ResolvedPattern::Binding(binding)) => match binding.last() {
            Some(b) => {
                if let Some(list_items) = b.list_items() {
                    let mut unique_list = Vector::new();
                    for item in list_items {
                        let resolved = ResolvedPattern::from_node(item);
                        if !unique_list.contains(&resolved) {
                            unique_list.push_back(resolved);
                        }
                    }
                    Ok(ResolvedPattern::List(unique_list))
                } else {
                    bail!("distinct takes a list as the first argument")
                }
            }
            None => Ok(ResolvedPattern::Binding(binding)),
        },
        _ => Err(anyhow!("distinct takes a list as the first argument")),
    }
}

// Shuffle a list
fn shuffle_fn<'a>(
    args: &'a [Option<Pattern>],
    context: &'a MarzanoContext<'a>,
    state: &mut State<'a>,
    logs: &mut AnalysisLogs,
) -> Result<ResolvedPattern<'a>> {
    let args = patterns_to_resolved(args, state, context, logs)?;

    let list = args
        .into_iter()
        .next()
        .ok_or(anyhow!("shuffle requires one argument"))?
        .ok_or(anyhow!(
            "shuffle requires a non-null list as the first argument"
        ))?;
    match list {
        ResolvedPattern::List(list) => {
            let mut shuffled_list = list.iter().cloned().collect::<Vec<_>>();
            shuffled_list.shuffle(state.get_rng());

            Ok(ResolvedPattern::List(shuffled_list.into()))
        }
        ResolvedPattern::Binding(binding) => match binding.last() {
            Some(b) => {
                if let Some(list_items) = b.list_items() {
                    let mut list: Vec<_> = list_items.collect();
                    list.shuffle(state.get_rng());
                    let list: Vector<_> =
                        list.into_iter().map(ResolvedPattern::from_node).collect();
                    Ok(ResolvedPattern::List(list))
                } else {
                    Err(anyhow!("shuffle takes a list as the first argument"))
                }
            }
            None => Err(anyhow!("shuffle argument must be bound")),
        },
        ResolvedPattern::Snippets(_)
        | ResolvedPattern::Map(_)
        | ResolvedPattern::File(_)
        | ResolvedPattern::Files(_)
        | ResolvedPattern::Constant(_) => {
            Err(anyhow!("shuffle takes a list as the first argument"))
        }
    }
}

fn length_fn<'a>(
    args: &'a [Option<Pattern>],
    context: &'a MarzanoContext<'a>,
    state: &mut State<'a>,
    logs: &mut AnalysisLogs,
) -> Result<ResolvedPattern<'a>> {
    let args = patterns_to_resolved(args, state, context, logs)?;

    let list = args.into_iter().next().unwrap();
    match &list {
        Some(ResolvedPattern::List(list)) => {
            let length = list.len();
            Ok(ResolvedPattern::Constant(Constant::Integer(length as i64)))
        }
        Some(ResolvedPattern::Binding(binding)) => match binding.last() {
            Some(resolved_pattern) => {
                let length = if let Some(list_items) = resolved_pattern.list_items() {
                    list_items.count()
                } else {
                    resolved_pattern.text().len()
                };
                Ok(ResolvedPattern::Constant(Constant::Integer(length as i64)))
            }
            None => Err(anyhow!("length argument must be a list or string")),
        },
        Some(resolved_pattern) => {
            if let Ok(text) = resolved_pattern.text(&state.files) {
                let length = text.len();
                Ok(ResolvedPattern::Constant(Constant::Integer(length as i64)))
            } else {
                Err(anyhow!("length argument must be a list or string"))
            }
        }
        None => Err(anyhow!("length argument must be a list or string")),
    }
}
