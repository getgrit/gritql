use super::{
    constants::{GLOBAL_VARS_SCOPE_INDEX, NEW_FILES_INDEX},
    patterns::{Matcher, Pattern},
    resolved_pattern::ResolvedPattern,
    state::{FilePtr, State},
};
use crate::{
    binding::Binding,
    clean::{get_replacement_ranges, replace_cleaned_ranges},
    context::{ExecContext, QueryContext},
    pattern::resolved_pattern::File,
    problem::{FileOwner, InputRanges, MatchRanges},
    text_unparser::apply_effects,
};
use anyhow::{anyhow, bail, Result};
use im::vector;
use marzano_language::language::Language;
use marzano_util::{analysis_logs::AnalysisLogs, node_with_source::NodeWithSource};
use std::path::PathBuf;
use tree_sitter::Parser;

#[derive(Debug, Clone)]
pub struct Step<Q: QueryContext> {
    pub pattern: Pattern<Q>,
}

impl<Q: QueryContext> Step<Q> {
    pub fn new(pattern: Pattern<Q>) -> Self {
        Self { pattern }
    }
}

impl<Q: QueryContext> Matcher<Q> for Step<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let mut parser = Parser::new()?;
        parser.set_language(context.language().get_ts_language())?;

        let files = if let Some(files) = binding.get_file_pointers() {
            files
                .iter()
                .map(|f| state.files.latest_revision(f))
                .collect::<Vec<FilePtr>>()
        } else {
            return Ok(false);
        };

        let binding = if files.len() == 1 {
            ResolvedPattern::from_file_pointer(*files.last().unwrap())
        } else {
            ResolvedPattern::from_files(ResolvedPattern::from_list_parts(
                files.iter().map(|f| ResolvedPattern::from_file_pointer(*f)),
            ))
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
                    let root = NodeWithSource::new(tree.root_node(), &new_src);
                    let replacement_ranges = get_replacement_ranges(root, context.language());
                    let cleaned_src = replace_cleaned_ranges(replacement_ranges, &new_src)?;
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

        let Some(new_files) = state.bindings[GLOBAL_VARS_SCOPE_INDEX]
            .last()
            .and_then(|binding| binding[NEW_FILES_INDEX].value.as_ref())
            .and_then(ResolvedPattern::get_list_items)
        else {
            bail!("Expected a list of files")
        };

        for f in new_files {
            let Some(file) = f.get_file() else {
                bail!("Expected a list of files")
            };

            let name: PathBuf = file
                .name(&state.files)
                .text(&state.files, context.language())
                .unwrap()
                .as_ref()
                .into();
            let body = file
                .body(&state.files)
                .text(&state.files, context.language())
                .unwrap()
                .into();
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
        }

        state.effects = vector![];
        let the_new_files =
            state.bindings[GLOBAL_VARS_SCOPE_INDEX].back_mut().unwrap()[NEW_FILES_INDEX].as_mut();
        the_new_files.value = Some(ResolvedPattern::from_list_parts([].into_iter()));
        Ok(true)
    }
}
