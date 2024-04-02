use rand::SeedableRng;
use std::collections::HashMap;
use std::ops::Range as StdRange;

use super::compiler::MATCH_VAR;
use super::FileOwner;
use crate::intervals::{earliest_deadline_sort, get_top_level_intervals_in_range, Interval};
use anyhow::{anyhow, Result};
use anyhow::{bail, Ok};
use im::{vector, Vector};
use marzano_language::target_language::TargetLanguage;
use marzano_util::analysis_logs::AnalysisLogs;
use marzano_util::position::Range;
use marzano_util::position::VariableMatch;

use super::{
    patterns::Pattern,
    resolved_pattern::{CodeRange, ResolvedPattern},
    variable::Variable,
    variable_content::VariableContent,
    Effect,
};

#[derive(Debug, Clone)]
pub struct EffectRange<'a> {
    range: StdRange<u32>,
    pub effect: Effect<'a>,
}

impl Interval for EffectRange<'_> {
    fn interval(&self) -> (u32, u32) {
        (self.range.start, self.range.end)
    }
}

#[derive(Clone, Debug)]
pub struct FileRegistry<'a>(Vector<Vector<&'a FileOwner>>);

impl<'a> FileRegistry<'a> {
    pub(crate) fn get_file(&self, pointer: FilePtr) -> &'a FileOwner {
        self.0[pointer.file as usize][pointer.version as usize]
    }

    pub(crate) fn new(files: Vector<Vector<&'a FileOwner>>) -> Self {
        Self(files)
    }

    // assumes at least one revision exists
    pub(crate) fn latest_revision(&self, pointer: &FilePtr) -> FilePtr {
        let latest = self.0[pointer.file as usize].len() - 1;
        FilePtr {
            file: pointer.file,
            version: latest as u16,
        }
    }

    pub(crate) fn files(&self) -> &Vector<Vector<&'a FileOwner>> {
        &self.0
    }

    pub(crate) fn push_revision(&mut self, pointer: &FilePtr, file: &'a FileOwner) {
        self.0[pointer.file as usize].push_back(file)
    }

    pub(crate) fn push_new_file(&mut self, file: &'a FileOwner) -> FilePtr {
        self.0.push_back(vector![file]);
        FilePtr {
            file: (self.0.len() - 1) as u16,
            version: 0,
        }
    }
}

// todo: we don't want to clone pattern definitions when cloning State
#[derive(Clone, Debug)]
pub struct State<'a> {
    pub bindings: VarRegistry<'a>,
    pub effects: Vector<Effect<'a>>,
    pub files: FileRegistry<'a>,
    rng: rand::rngs::StdRng,
}

fn get_top_level_effect_ranges<'a>(
    effects: &[Effect<'a>],
    memo: &HashMap<CodeRange, Option<String>>,
    range: &CodeRange,
    language: &TargetLanguage,
    logs: &mut AnalysisLogs,
) -> Result<Vec<EffectRange<'a>>> {
    let mut effects: Vec<EffectRange> = effects
        .iter()
        .filter(|effect| {
            let binding = &effect.binding;
            if let Some(src) = binding.source() {
                if let Some(position) = binding.position() {
                    range.equal_address(src)
                        && !matches!(memo.get(&CodeRange::from_range(src, position)), Some(None))
                } else {
                    let _ = binding.log_empty_field_rewrite_error(language, logs);
                    false
                }
            } else {
                false
            }
        })
        .map(|effect| {
            let binding = &effect.binding;
            let ts_range = binding
                .position()
                .ok_or_else(|| anyhow!("binding has no position"))?;
            let end_byte = ts_range.end_byte;
            let start_byte = ts_range.start_byte;
            Ok(EffectRange {
                range: start_byte..end_byte,
                effect: effect.clone(),
            })
        })
        .collect::<Result<Vec<EffectRange>>>()?;
    if !earliest_deadline_sort(&mut effects) {
        bail!("effects have overlapping ranges");
    }
    Ok(get_top_level_intervals_in_range(
        effects,
        range.start,
        range.end,
    ))
}

pub(crate) fn get_top_level_effects<'a>(
    effects: &[Effect<'a>],
    memo: &HashMap<CodeRange, Option<String>>,
    range: &CodeRange,
    language: &TargetLanguage,
    logs: &mut AnalysisLogs,
) -> Result<Vec<Effect<'a>>> {
    let top_level = get_top_level_effect_ranges(effects, memo, range, language, logs)?;
    let top_level: Vec<Effect<'a>> = top_level
        .into_iter()
        .map(|e| {
            assert!(e.range.start >= range.start);
            assert!(e.range.end <= range.end);
            e.effect
        })
        .collect();
    Ok(top_level)
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct FilePtr {
    pub(crate) file: u16,
    pub(crate) version: u16,
}

impl FilePtr {
    pub(crate) fn new(file: u16, version: u16) -> Self {
        Self { file, version }
    }
}

impl<'a> State<'a> {
    pub(crate) fn new(bindings: VarRegistry<'a>, files: Vec<&'a FileOwner>) -> Self {
        Self {
            rng: rand::rngs::StdRng::seed_from_u64(32),
            bindings,
            effects: vector![],
            files: FileRegistry::new(files.into_iter().map(|f| vector![f]).collect()),
        }
    }

    pub fn get_files<'b>(&'b self) -> &'b FileRegistry
    where
        'b: 'a,
    {
        &self.files
    }

    // Grit uses a fixed seed RNG for reproducibility
    pub fn get_rng(&mut self) -> &mut rand::rngs::StdRng {
        &mut self.rng
    }

    pub(crate) fn reset_vars(&mut self, scope: usize, args: &'a [Option<Pattern>]) {
        let old_scope = self.bindings[scope].last().unwrap();
        let new_scope: Vector<Box<VariableContent>> = old_scope
            .iter()
            .enumerate()
            .map(|(index, content)| {
                let mut content = content.clone();
                let pattern = if index < args.len() {
                    args[index].as_ref()
                } else {
                    None
                };
                if let Some(Pattern::Variable(v)) = pattern {
                    content.mirrors.push(v)
                };
                Box::new(VariableContent {
                    pattern,
                    value: None,
                    value_history: Vec::new(),
                    ..*content
                })
            })
            .collect();
        self.bindings[scope].push_back(new_scope);
    }

    // unfortunately these accessor functions are not as useful as they
    // could be due to the inability of rust to split borrows across functions
    // within a function you could mutably borrow bindings, and immutably borrow
    // src simultaneously, but you can't do that across functions.
    // see:
    // https://stackoverflow.com/questions/61699010/rust-not-allowing-mutable-borrow-when-splitting-properly
    // https://doc.rust-lang.org/nomicon/borrow-splitting.html
    // todo split State in a sensible way.
    pub fn get_name(&self, var: &Variable) -> &str {
        &self.bindings[var.scope].last().unwrap()[var.index].name
    }

    pub(crate) fn trace_var(&self, var: &Variable) -> Variable {
        if let Some(Pattern::Variable(v)) =
            &self.bindings[var.scope].last().unwrap()[var.index].pattern
        {
            self.trace_var(v)
        } else {
            *var
        }
    }

    pub(crate) fn bindings_history_to_ranges(
        &self,
        lang: &TargetLanguage,
        current_name: Option<&str>,
    ) -> (Vec<VariableMatch>, Vec<Range>, bool) {
        let mut matches = vec![];
        let mut top_level_matches = vec![];
        let mut suppressed = false;
        for (i, scope) in self.bindings.iter().enumerate() {
            for (j, content) in scope.last().unwrap().iter().enumerate() {
                let name = content.name.clone();
                let mut var_ranges = vec![];
                let mut bindings_count = 0;
                let mut suppressed_count = 0;
                for value in content.value_history.iter() {
                    if let ResolvedPattern::Binding(bindings) = value {
                        for binding in bindings.iter() {
                            bindings_count += 1;
                            if binding.is_suppressed(lang, current_name) {
                                suppressed_count += 1;
                                continue;
                            }
                            if let Some(match_position) = binding.position() {
                                // TODO, this check only needs to be done at the global scope right?
                                if name == MATCH_VAR {
                                    // apply_match = true;
                                    top_level_matches.push(match_position);
                                }
                                var_ranges.push(match_position);
                            }
                        }
                    }
                }
                if suppressed_count > 0 && suppressed_count == bindings_count {
                    suppressed = true;
                    continue;
                }
                let scoped_name = format!("{}_{}_{}", i, j, name);
                let var_match = VariableMatch::new(name, scoped_name, var_ranges);
                matches.push(var_match);
            }
        }
        suppressed = suppressed && top_level_matches.is_empty();
        (matches, top_level_matches, suppressed)
    }
}

type VarRegistry<'a> = Vector<Vector<Vector<Box<VariableContent<'a>>>>>;
