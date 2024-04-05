use super::{
    constants::{GLOBAL_VARS_SCOPE_INDEX, NEW_FILES_INDEX},
    patterns::{Matcher, Pattern},
    resolved_pattern::{File, ResolvedPattern},
    state::{FilePtr, State},
    FileOwner, InputRanges, MatchRanges,
};
use crate::{
    context::Context,
    orphan::{get_orphaned_ranges, remove_orphaned_ranges},
    text_unparser::apply_effects,
};
use anyhow::{anyhow, bail, Result};
use im::vector;
use marzano_language::language::Language;
use marzano_util::analysis_logs::AnalysisLogs;
use std::path::PathBuf;
use tree_sitter::Parser;

#[derive(Debug, Clone)]
pub struct Step {
    pub(crate) pattern: Pattern,
}

fn extract_file_pointer(file: &File) -> Option<FilePtr> {
    match file {
        File::Resolved(_) => None,
        File::Ptr(ptr) => Some(*ptr),
    }
}

fn handle_files(files_list: &ResolvedPattern) -> Option<Vec<FilePtr>> {
    if let ResolvedPattern::List(files) = files_list {
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

fn extract_file_pointers(binding: &ResolvedPattern) -> Option<Vec<FilePtr>> {
    match binding {
        ResolvedPattern::Binding(_) => None,
        ResolvedPattern::Snippets(_) => None,
        ResolvedPattern::List(_) => handle_files(binding),
        ResolvedPattern::Map(_) => None,
        ResolvedPattern::File(file) => extract_file_pointer(file).map(|f| vec![f]),
        ResolvedPattern::Files(files) => handle_files(files),
        ResolvedPattern::Constant(_) => None,
    }
}

impl Matcher for Step {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let mut parser = Parser::new()?;
        parser.set_language(context.language().get_ts_language())?;

        let files = if let Some(files) = extract_file_pointers(binding) {
            files
                .iter()
                .map(|f| state.files.latest_revision(f))
                .collect::<Vec<FilePtr>>()
        } else {
            return Ok(false);
        };

        let binding = if files.len() == 1 {
            ResolvedPattern::File(File::Ptr(*files.last().unwrap()))
        } else {
            ResolvedPattern::Files(Box::new(ResolvedPattern::List(
                files
                    .iter()
                    .map(|f| ResolvedPattern::File(File::Ptr(*f)))
                    .collect(),
            )))
        };
        if !self.pattern.execute(&binding, state, context, logs)? {
            return Ok(false);
        }

        // todo, for multifile we need to split up the matches by file.
        let (variables, ranges, suppressed) =
            state.bindings_history_to_ranges(context.language(), context.name());

        let input_ranges = InputRanges {
            ranges,
            variables,
            suppressed,
        };
        for file_ptr in files {
            let file = state.files.get_file(file_ptr);
            let mut match_log = file.matches.borrow_mut();

            let filename_path = &file.name;

            let mut new_filename = filename_path.clone();

            let src = &file.source;

            if match_log.input_matches.is_none() {
                match_log.input_matches = Some(input_ranges.clone());
            }

            if state
                .effects
                .iter()
                .find(|e| {
                    e.binding.source() == Some(src)
                        || e.binding.as_filename() == Some(filename_path)
                })
                .cloned()
                .is_some()
            {
                let (new_src, new_ranges) = apply_effects(
                    src,
                    state.effects.clone(),
                    &state.files,
                    &file.name,
                    &mut new_filename,
                    context.language(),
                    context.name(),
                    logs,
                )?;
                if let Some(new_ranges) = new_ranges {
                    let tree = parser.parse(new_src.as_bytes(), None).unwrap().unwrap();
                    let orphans = get_orphaned_ranges(&tree, &new_src, context.language());
                    let (_cleaned_tree, cleaned_src) =
                        remove_orphaned_ranges(&mut parser, orphans, &new_src)?;
                    let new_src = if let Some(src) = cleaned_src {
                        src
                    } else {
                        new_src
                    };

                    let ranges =
                        MatchRanges::new(new_ranges.into_iter().map(|r| r.into()).collect());
                    let owned_file = FileOwner::new(
                        new_filename.clone(),
                        new_src,
                        Some(ranges),
                        true,
                        context.language(),
                        logs,
                    )?
                    .ok_or_else(|| {
                        anyhow!(
                            "failed to construct new file for file {}",
                            new_filename.to_string_lossy()
                        )
                    })?;
                    context.files().push(owned_file);
                    state
                        .files
                        .push_revision(&file_ptr, context.files().last().unwrap())
                }
            };
        }

        if let Some(ResolvedPattern::List(new_files_vector)) =
            &state.bindings[GLOBAL_VARS_SCOPE_INDEX].last().unwrap()[NEW_FILES_INDEX].value
        {
            for f in new_files_vector {
                if let ResolvedPattern::File(file) = f {
                    let name: PathBuf = file
                        .name(&state.files)
                        .text(&state.files)
                        .unwrap()
                        .as_ref()
                        .into();
                    let body = file.body(&state.files).text(&state.files).unwrap().into();
                    let owned_file =
                        FileOwner::new(name.clone(), body, None, true, context.language(), logs)?
                            .ok_or_else(|| {
                            anyhow!(
                                "failed to construct new file for file {}",
                                name.to_string_lossy()
                            )
                        })?;
                    context.files().push(owned_file);
                    let _ = state.files.push_new_file(context.files().last().unwrap());
                } else {
                    bail!("Expected a list of files")
                }
            }
        } else {
            bail!("Expected a list of files")
        }

        state.effects = vector![];
        let the_new_files =
            state.bindings[GLOBAL_VARS_SCOPE_INDEX].back_mut().unwrap()[NEW_FILES_INDEX].as_mut();
        the_new_files.value = Some(ResolvedPattern::List(vector!()));
        Ok(true)
    }
}
