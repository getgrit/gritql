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
use grit_util::{
    error::{GritPatternError, GritResult},
    AnalysisLogs, CodeRange, Range, VariableMatch,
};
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
    owners: Vector<Vector<&'a FileOwner<Q::Tree<'a>>>>,
}

impl<'a, Q: QueryContext> FileRegistry<'a, Q> {
    pub fn get_file_owner(&self, pointer: FilePtr) -> &'a FileOwner<Q::Tree<'a>> {
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

    pub fn get_absolute_path(&self, pointer: FilePtr) -> GritResult<&'a PathBuf> {
        let file_index = pointer.file as usize;
        let version_index = pointer.version as usize;
        if let Some(owners) = self.owners.get(file_index) {
            if let Some(owner) = owners.get(version_index) {
                return Ok(&owner.absolute_path);
            }
        }
        Err(GritPatternError::new(
            "Absolute file path accessed before file was loaded.",
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
    pub fn load_file(&mut self, pointer: &FilePtr, file: &'a FileOwner<Q::Tree<'a>>) {
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

    pub fn files(&self) -> &Vector<Vector<&'a FileOwner<Q::Tree<'a>>>> {
        &self.owners
    }

    pub fn push_revision(&mut self, pointer: &FilePtr, file: &'a FileOwner<Q::Tree<'a>>) {
        self.version_count[pointer.file as usize] += 1;
        self.owners[pointer.file as usize].push_back(file)
    }

    pub fn push_new_file(&mut self, file: &'a FileOwner<Q::Tree<'a>>) -> FilePtr {
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
    current_scope: usize,
}

fn get_top_level_effect_ranges<'a, Q: QueryContext>(
    effects: &[Effect<'a, Q>],
    memo: &HashMap<CodeRange, Option<String>>,
    range: &CodeRange,
    language: &Q::Language<'a>,
    logs: &mut AnalysisLogs,
) -> GritResult<Vec<EffectRange<'a, Q>>> {
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
                .ok_or_else(|| GritPatternError::new("binding has no range"))?;
            let end_byte = byte_range.end as u32;
            let start_byte = byte_range.start as u32;
            Ok(EffectRange {
                range: start_byte..end_byte,
                effect: effect.clone(),
            })
        })
        .collect::<GritResult<Vec<EffectRange<Q>>>>()?;
    if !earliest_deadline_sort(&mut effects) {
        return Err(GritPatternError::new("effects have overlapping ranges"));
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
) -> GritResult<Vec<Effect<'a, Q>>> {
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

pub struct ScopeTracker {
    previous_scope: usize,
}

impl<'a, Q: QueryContext> State<'a, Q> {
    pub fn new(bindings: VarRegistry<'a, Q>, registry: FileRegistry<'a, Q>) -> Self {
        Self {
            rng: rand::rngs::StdRng::seed_from_u64(32),
            current_scope: 0,
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

    /// Enter a scope by copying the current scope and adding the new variables
    /// When you are done with a scope, you *must* call exit_scope
    ///
    /// # Parameters
    ///
    /// * `scope` - The scope to enter
    /// * `args` - The arguments to the scope
    pub(crate) fn enter_scope(
        &mut self,
        scope: usize,
        args: &'a [Option<Pattern<Q>>],
    ) -> ScopeTracker {
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

        let old_scope_index = self.current_scope;
        self.current_scope = scope;

        ScopeTracker {
            previous_scope: old_scope_index,
        }
    }

    pub(crate) fn exit_scope(&mut self, tracker: ScopeTracker) {
        self.current_scope = tracker.previous_scope;
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
        &self.bindings[var.try_scope().unwrap().into()]
            .last()
            .unwrap()[var.try_index().unwrap().into()]
        .name
    }

    /// Attempt to find a variable by name in any scope
    /// This is inefficient and should only be used when we haven't pre-allocated a Variable reference
    ///
    /// If you have a Variable reference, use `trace_var` instead to find the latest binding
    pub fn find_var(&self, name: &str) -> Option<Variable> {
        for (scope_index, scope) in self.bindings.iter().enumerate().rev() {
            for (index, content) in scope.last().unwrap().iter().enumerate() {
                if content.name == name {
                    return Some(Variable::new(scope_index, index));
                }
            }
        }
        None
    }

    pub fn register_var(&mut self, name: &str) -> (usize, usize) {
        let scope = self.current_scope;
        let the_scope = self.bindings[self.current_scope].back_mut().unwrap();
        let index = the_scope.len();

        the_scope.push_back(Box::new(VariableContent::new(name.to_string())));
        (scope, index)
    }

    /// Attempt to find a variable by name in the current scope
    pub fn find_var_in_scope(&mut self, name: &str) -> Option<Variable> {
        for (index, content) in self.bindings[self.current_scope]
            .last()
            .unwrap()
            .iter()
            .enumerate()
        {
            if content.name == name {
                return Some(Variable::new(self.current_scope, index));
            }
        }
        None
    }

    /// Trace a variable to the root binding
    pub fn trace_var(&self, var: &Variable) -> Variable {
        if let Some(Pattern::Variable(v)) = &self.bindings[var.try_scope().unwrap().into()]
            .last()
            .unwrap()[var.try_index().unwrap().into()]
        .pattern
        {
            self.trace_var(v)
        } else {
            var.clone()
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
