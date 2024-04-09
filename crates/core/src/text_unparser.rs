use crate::binding::linearize_binding;
use crate::pattern::resolved_pattern::CodeRange;
use crate::pattern::state::FileRegistry;
use crate::pattern::Effect;
use anyhow::Result;
use im::Vector;
use marzano_language::target_language::TargetLanguage;
use marzano_util::analysis_logs::AnalysisLogs;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Range;
use std::path::{Path, PathBuf};

/**
 * Applies the given effects to the given code, using the bindings to resolve metavariables in the snippets.
 *
 * Bindings is a mapping from variable names to replacement string -- which is obtained from any of the nodes in the bindings vector.
 */

#[allow(clippy::too_many_arguments)]
pub(crate) fn apply_effects<'a>(
    code: &'a str,
    effects: Vector<Effect<'a>>,
    files: &FileRegistry<'a>,
    the_filename: &Path,
    new_filename: &mut PathBuf,
    language: &TargetLanguage,
    current_name: Option<&str>,
    logs: &mut AnalysisLogs,
) -> Result<(String, Option<Vec<Range<usize>>>)> {
    let effects: Vec<_> = effects
        .into_iter()
        .filter(|effect| !effect.binding.is_suppressed(language, current_name))
        .collect();
    if effects.is_empty() {
        return Ok((code.to_string(), None));
    }
    let mut memo: HashMap<CodeRange, Option<String>> = HashMap::new();
    let (from_inline, ranges) = linearize_binding(
        language,
        &effects,
        files,
        &mut memo,
        code,
        CodeRange::new(0, code.len() as u32, code),
        language.should_pad_snippet().then_some(0),
        logs,
    )?;
    for effect in effects.iter() {
        if let Some(filename) = effect.binding.as_filename() {
            if std::ptr::eq(filename, the_filename) {
                let snippet = effect
                    .pattern
                    .linearized_text(language, &effects, files, &mut memo, false, logs)?;
                *new_filename = PathBuf::from(snippet.to_string());
            }
        }
    }
    Ok((from_inline.to_string(), Some(ranges)))
}

/**
 * This is a naive implementation of distributing indentation to a string, for cases where we don't have more context to do it well.
 */
pub fn naive_distribute_indentation(
    original_string: Cow<str>,
    distributed_indent: Option<usize>,
) -> Cow<str> {
    if let Some(distributed_indent) = distributed_indent {
        original_string
            .lines()
            .enumerate()
            .map(|(i, line)| {
                if i == 0 {
                    line.to_string()
                } else {
                    format!("{}{}", " ".repeat(distributed_indent), line)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            .into()
    } else {
        original_string
    }
}
