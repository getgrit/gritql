use crate::{inline_snippets::ReplacementInfo, marzano_binding::linearize_binding};
use anyhow::Result;
use grit_pattern_matcher::{
    binding::Binding,
    context::{ExecContext, QueryContext},
    effects::Effect,
    pattern::{FileRegistry, ResolvedPattern},
};
use grit_util::{AnalysisLogs, AstNode, CodeRange, Language};
use im::Vector;
use std::collections::HashMap;
use std::ops::Range;
use std::path::{Path, PathBuf};

/**
 * Applies the given effects to the given code, using the bindings to resolve metavariables in the snippets.
 *
 * Bindings is a mapping from variable names to replacement string -- which is obtained from any of the nodes in the bindings vector.
 */

/// The outcome of applying an effect to a code snippet or file
/// new_source, replacement_ranges in original source, replacement_infos for input
type EffectOutcome = (
    String,
    Option<Vec<Range<usize>>>,
    Option<Vec<ReplacementInfo>>,
);

#[allow(clippy::too_many_arguments)]
pub(crate) fn apply_effects<'a, Q: QueryContext>(
    code: Q::Node<'a>,
    effects: Vector<Effect<'a, Q>>,
    files: &FileRegistry<'a, Q>,
    the_filename: &Path,
    new_filename: &mut PathBuf,
    context: &'a Q::ExecContext<'a>,
    logs: &mut AnalysisLogs,
) -> Result<EffectOutcome> {
    let language = context.language();
    let current_name = context.name();

    let effects: Vec<_> = effects
        .into_iter()
        .filter(|effect| !effect.binding.is_suppressed(language, current_name))
        .collect();
    if effects.is_empty() {
        return Ok((code.full_source().to_owned(), None, None));
    }
    let mut memo: HashMap<CodeRange, Option<String>> = HashMap::new();
    let (from_inline, output_ranges, effect_ranges) = linearize_binding(
        language,
        &effects,
        files,
        &mut memo,
        code.clone(),
        CodeRange::new(0, code.full_source().len() as u32, code.full_source()),
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
    Ok((
        from_inline.to_string(),
        Some(output_ranges),
        Some(effect_ranges),
    ))
}
