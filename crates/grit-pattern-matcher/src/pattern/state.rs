use super::{patterns::Pattern, variable::Variable, variable_content::VariableContent};
use crate::{
    binding::Binding,
    constants::MATCH_VAR,
    context::QueryContext,
    effects::Effect,
    file_owners::FileOwner,
    intervals::{earliest_deadline_sort, get_top_level_intervals_in_range, Interval},
    pattern::resolved_pattern::ResolvedPattern,
};
use anyhow::{anyhow, bail, Result};
use grit_util::{AnalysisLogs, CodeRange, Range, VariableMatch};
use im::{vector, Vector};
use rand::SeedableRng;
use std::ops::Range as StdRange;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone)]
pub struct EffectRange<'a, Q: QueryContext> {
    range: StdRange<u32>,
    pub effect: Effect<'a, Q>,
}

impl<Q: QueryContext> Interval for EffectRange<'_, Q> {
    fn interval(&self) -> (u32, u32) {
        (self.range.start, self.range.end)
    }
}

#[derive(Clone, Debug)]
pub struct FileRegistry<'a, Q: QueryContext> {
    /// The number of versions for each file
    version_count: Vec<u16>,
    /// Original file paths, for lazy loading
    file_paths: Vec<&'a PathBuf>,
    /// The actual FileOwner, which has the full file available
    owners: Vector<Vector<&'a FileOwner<Q::Tree>>>,
}

impl<'a, Q: QueryContext> FileRegistry<'a, Q> {
    pub fn get_file_owner(&self, pointer: FilePtr) -> &'a FileOwner<Q::Tree> {
        self.owners[pointer.file as usize][pointer.version as usize]
    }

    pub fn get_file_name(&self, pointer: FilePtr) -> &'a PathBuf {
        let file_index = pointer.file as usize;
        let version_index = pointer.version as usize;
        if let Some(owners) = self.owners.get(file_index) {
            if let Some(owner) = owners.get(version_index) {
                return &owner.name;
            }
        }
        self.file_paths
            .get(file_index)
            .expect("File path should exist for given file index.")
    }

    pub fn get_absolute_path(&self, pointer: FilePtr) -> Result<&'a PathBuf> {
        let file_index = pointer.file as usize;
        let version_index = pointer.version as usize;
        if let Some(owners) = self.owners.get(file_index) {
            if let Some(owner) = owners.get(version_index) {
                return Ok(&owner.absolute_path);
            }
        }
        Err(anyhow!(
            "Absolute file path accessed before file was loaded."
        ))
    }

    /// If only the paths are available, create a FileRegistry with empty owners
    /// This is *unsafe* if you do not later insert the appropriate owners before get_file_owner is called
    pub fn new_from_paths(file_paths: Vec<&'a PathBuf>) -> Self {
        Self {
            version_count: file_paths.iter().map(|_| 0).collect(),
            owners: file_paths.iter().map(|_| vector![]).collect(),
            file_paths,
        }
    }

    /// Confirms a file is already fully loaded
    pub fn is_loaded(&self, pointer: &FilePtr) -> bool {
        self.version_count
            .get(pointer.file as usize)
            .map_or(false, |&v| v > 0)
    }

    /// Load a file in
    pub fn load_file(&mut self, pointer: &FilePtr, file: &'a FileOwner<Q::Tree>) {
        self.push_revision(pointer, file)
    }

    /// Returns the latest revision of a given filepointer
    /// If none exists, returns the file pointer itself
    pub fn latest_revision(&self, pointer: &FilePtr) -> FilePtr {
        match self.version_count.get(pointer.file as usize) {
            Some(&version_count) => {
                if version_count == 0 {
                    *pointer
                } else {
                    FilePtr {
                        file: pointer.file,
                        version: version_count - 1,
                    }
                }
            }
            None => *pointer,
        }
    }

    pub fn files(&self) -> &Vector<Vector<&'a FileOwner<Q::Tree>>> {
        &self.owners
    }

    pub fn push_revision(&mut self, pointer: &FilePtr, file: &'a FileOwner<Q::Tree>) {
        self.version_count[pointer.file as usize] += 1;
        self.owners[pointer.file as usize].push_back(file)
    }

    pub fn push_new_file(&mut self, file: &'a FileOwner<Q::Tree>) -> FilePtr {
        self.version_count.push(1);
        self.file_paths.push(&file.name);
        self.owners.push_back(vector![file]);
        FilePtr {
            file: (self.owners.len() - 1) as u16,
            version: 0,
        }
    }
}

// todo: we don't want to clone pattern definitions when cloning State
#[derive(Clone, Debug)]
pub struct State<'a, Q: QueryContext> {
    pub bindings: VarRegistry<'a, Q>,
    pub effects: Vector<Effect<'a, Q>>,
    pub files: FileRegistry<'a, Q>,
    rng: rand::rngs::StdRng,
}

fn get_top_level_effect_ranges<'a, Q: QueryContext>(
    effects: &[Effect<'a, Q>],
    memo: &HashMap<CodeRange, Option<String>>,
    range: &CodeRange,
    language: &Q::Language<'a>,
    logs: &mut AnalysisLogs,
) -> Result<Vec<EffectRange<'a, Q>>> {
    let mut effects: Vec<EffectRange<Q>> = effects
        .iter()
        .filter(|effect| {
            let binding = &effect.binding;
            if let Some(src) = binding.source() {
                if let Some(binding_range) = binding.code_range(language) {
                    range.applies_to(src) && !matches!(memo.get(&binding_range), Some(None))
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
            let byte_range = binding
                .range(language)
                .ok_or_else(|| anyhow!("binding has no range"))?;
            let end_byte = byte_range.end as u32;
            let start_byte = byte_range.start as u32;
            Ok(EffectRange {
                range: start_byte..end_byte,
                effect: effect.clone(),
            })
        })
        .collect::<Result<Vec<EffectRange<Q>>>>()?;
    if !earliest_deadline_sort(&mut effects) {
        bail!("effects have overlapping ranges");
    }
    Ok(get_top_level_intervals_in_range(
        effects,
        range.start,
        range.end,
    ))
}

pub fn get_top_level_effects<'a, Q: QueryContext>(
    effects: &[Effect<'a, Q>],
    memo: &HashMap<CodeRange, Option<String>>,
    range: &CodeRange,
    language: &Q::Language<'a>,
    logs: &mut AnalysisLogs,
) -> Result<Vec<Effect<'a, Q>>> {
    let top_level = get_top_level_effect_ranges(effects, memo, range, language, logs)?;
    let top_level: Vec<Effect<'a, Q>> = top_level
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
    pub file: u16,
    pub version: u16,
}

impl FilePtr {
    pub fn new(file: u16, version: u16) -> Self {
        Self { file, version }
    }
}

impl<'a, Q: QueryContext> State<'a, Q> {
    pub fn new(bindings: VarRegistry<'a, Q>, registry: FileRegistry<'a, Q>) -> Self {
        Self {
            rng: rand::rngs::StdRng::seed_from_u64(32),
            bindings,
            effects: vector![],
            files: registry,
        }
    }

    pub fn get_files<'b>(&'b self) -> &'b FileRegistry<Q>
    where
        'b: 'a,
    {
        &self.files
    }

    // Grit uses a fixed seed RNG for reproducibility
    pub fn get_rng(&mut self) -> &mut rand::rngs::StdRng {
        &mut self.rng
    }

    pub(crate) fn reset_vars(&mut self, scope: usize, args: &'a [Option<Pattern<Q>>]) {
        let old_scope = self.bindings[scope].last().unwrap();
        let new_scope: Vector<Box<VariableContent<Q>>> = old_scope
            .iter()
            .enumerate()
            .map(|(index, content)| {
                let mut content = content.clone();
                let pattern = args.get(index).and_then(Option::as_ref);
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

    pub fn trace_var(&self, var: &Variable) -> Variable {
        if let Some(Pattern::Variable(v)) =
            &self.bindings[var.scope].last().unwrap()[var.index].pattern
        {
            self.trace_var(v)
        } else {
            *var
        }
    }

    pub fn bindings_history_to_ranges(
        &self,
        language: &Q::Language<'a>,
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
                    if let Some(bindings) = value.get_bindings() {
                        for binding in bindings {
                            bindings_count += 1;
                            if binding.is_suppressed(language, current_name) {
                                suppressed_count += 1;
                                continue;
                            }
                            if let Some(match_position) = binding.position(language) {
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

pub type VarRegistry<'a, P> = Vector<Vector<Vector<Box<VariableContent<'a, P>>>>>;
