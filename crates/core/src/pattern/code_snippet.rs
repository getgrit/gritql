use super::{
    dynamic_snippet::{DynamicPattern, DynamicSnippet},
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    variable::{register_variable, VariableSourceLocations},
    State,
};
use crate::{context::Context, resolve};
use anyhow::{anyhow, bail, Result};
use core::fmt::Debug;
use marzano_language::{
    language::{nodes_from_indices, Language, SortId},
    target_language::TargetLanguage,
};
use marzano_util::{analysis_logs::AnalysisLogs, position::Range};
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct CodeSnippet {
    pub(crate) patterns: Vec<(SortId, Pattern)>,
    pub(crate) source: String,
    pub(crate) dynamic_snippet: Option<DynamicPattern>,
}

impl CodeSnippet {
    pub fn new(
        patterns: Vec<(SortId, Pattern)>,
        dynamic_snippet: Option<DynamicPattern>,
        source: &str,
    ) -> Self {
        Self {
            patterns,
            source: source.to_string(),
            dynamic_snippet,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_node(
        node: &Node,
        file: &str,
        src: &str,
        vars: &mut BTreeMap<String, usize>,
        global_vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        lang: &TargetLanguage,
        is_rhs: bool,
    ) -> Result<Pattern> {
        let snippet = node
            .child_by_field_name("source")
            .ok_or_else(|| anyhow!("missing content of codeSnippet"))?;
        match snippet.kind().as_ref() {
            "backtickSnippet" => from_back_tick_node(
                snippet,
                file,
                src,
                vars,
                global_vars,
                vars_array,
                scope_index,
                lang,
                is_rhs,
            ),
            "rawBacktickSnippet" => from_raw_back_tick_node(
                snippet,
                src,
                vars,
                global_vars,
                vars_array,
                scope_index,
                lang,
                is_rhs,
            ),
            "languageSpecificSnippet" => language_specific_snippet(
                snippet,
                file,
                src,
                vars,
                global_vars,
                vars_array,
                scope_index,
                lang,
                is_rhs,
            ),
            _ => bail!("invalid code snippet kind: {}", snippet.kind()),
        }
    }
}

impl Name for CodeSnippet {
    fn name(&self) -> &'static str {
        "CODESNIPPET"
    }
}

impl Matcher for CodeSnippet {
    // wrong, but whatever for now
    fn execute<'a>(
        &'a self,
        resolved_pattern: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let binding = match resolved_pattern {
            ResolvedPattern::Binding(binding) => resolve!(binding.last()),
            resolved @ ResolvedPattern::Snippets(_)
            | resolved @ ResolvedPattern::List(_)
            | resolved @ ResolvedPattern::Map(_)
            | resolved @ ResolvedPattern::File(_)
            | resolved @ ResolvedPattern::Files(_)
            | resolved @ ResolvedPattern::Constant(_) => {
                return Ok(resolved.text(&state.files)?.trim() == self.source)
            }
        };

        let Some(node) = binding.singleton() else {
            return Ok(false);
        };

        if let Some((_, pattern)) = self
            .patterns
            .iter()
            .find(|(id, _)| *id == node.node.kind_id())
        {
            pattern.execute(resolved_pattern, state, context, logs)
        } else {
            Ok(false)
        }
    }
}

// we check for CURLY_VAR_REGEX in the content, and if found
// compile into a DynamicPattern, rather than a CodeSnippet.
// This is because the syntax should only ever be necessary
// when treating a metavariable as a string to substitute
// rather than an AST node to match on. eg. in the following
// `const ${name}Handler = useCallback(async () => $body, []);`
// $name does not correspond to a node, but rather prepends a
// string to "Handler", which will together combine into an
// identifier.

#[allow(clippy::too_many_arguments)]
fn process_snippet_content(
    source: &str,
    file: &str,
    range: Range,
    vars: &mut BTreeMap<String, usize>,
    global_vars: &mut BTreeMap<String, usize>,
    vars_array: &mut Vec<Vec<VariableSourceLocations>>,
    scope_index: usize,
    lang: &TargetLanguage,
    is_rhs: bool,
) -> Result<Pattern> {
    if lang.metavariable_bracket_regex().is_match(source) {
        if is_rhs {
            Ok(Pattern::Dynamic(
                DynamicSnippet::from(
                    source,
                    file,
                    range,
                    vars,
                    global_vars,
                    vars_array,
                    scope_index,
                    lang,
                )
                .map(DynamicPattern::Snippet)?,
            ))
        } else {
            bail!("bracketed metavariables are only allowed on the rhs of a snippet");
        }
    } else {
        if lang.exact_variable_regex().is_match(source.trim()) {
            match source.trim() {
                "$_" => return Ok(Pattern::Underscore),
                "^_" => return Ok(Pattern::Underscore),
                name => {
                    let var = register_variable(
                        name,
                        file,
                        range,
                        vars,
                        global_vars,
                        vars_array,
                        scope_index,
                    )?;
                    return Ok(Pattern::Variable(var));
                }
            }
        }
        let snippet_trees = lang.parse_snippet_contexts(source);
        let snippet_nodes = nodes_from_indices(&snippet_trees);
        if snippet_nodes.is_empty() {
            // not checking if is_rhs. So could potentially
            // be harder to find bugs where we expect the pattern
            // to parse. unfortunately got rid of check to support
            // passing non-node snippets as args.
            return Ok(Pattern::Dynamic(
                DynamicSnippet::from(
                    source,
                    file,
                    range,
                    vars,
                    global_vars,
                    vars_array,
                    scope_index,
                    lang,
                )
                .map(DynamicPattern::Snippet)?,
            ));
        }
        let snippet_patterns: Vec<(SortId, Pattern)> = snippet_nodes
            .into_iter()
            .map(|node| {
                Ok((
                    node.node.kind_id(),
                    Pattern::from_snippet_node(
                        node,
                        range,
                        file,
                        lang,
                        vars,
                        global_vars,
                        vars_array,
                        scope_index,
                        is_rhs,
                    )?,
                ))
            })
            .collect::<Result<Vec<(SortId, Pattern)>>>()?;
        let dynamic_snippet = DynamicSnippet::from(
            source,
            file,
            range,
            vars,
            global_vars,
            vars_array,
            scope_index,
            lang,
        )
        .map_or(None, |s| Some(DynamicPattern::Snippet(s)));
        Ok(Pattern::CodeSnippet(CodeSnippet::new(
            snippet_patterns,
            dynamic_snippet,
            source,
        )))
    }
}

#[allow(clippy::too_many_arguments)]
pub fn from_back_tick_node(
    node: Node,
    file: &str,
    src: &str,
    vars: &mut BTreeMap<String, usize>,
    global_vars: &mut BTreeMap<String, usize>,
    vars_array: &mut Vec<Vec<VariableSourceLocations>>,
    scope_index: usize,
    lang: &TargetLanguage,
    is_rhs: bool,
) -> Result<Pattern> {
    let source = node.utf8_text(src.as_bytes())?.to_string();
    let mut range: Range = node.range().into();
    range.adjust_columns(1, -1);
    let content = source
        .strip_prefix('`')
        .ok_or_else(|| anyhow!("Unable to extract content from snippet: {}", source))?
        .strip_suffix('`')
        .ok_or_else(|| anyhow!("Unable to extract content from snippet: {}", source))?;
    process_snippet_content(
        content,
        file,
        range,
        vars,
        global_vars,
        vars_array,
        scope_index,
        lang,
        is_rhs,
    )
}

#[allow(clippy::too_many_arguments)]
fn from_raw_back_tick_node(
    node: Node,
    src: &str,
    vars: &mut BTreeMap<String, usize>,
    global_vars: &mut BTreeMap<String, usize>,
    vars_array: &mut Vec<Vec<VariableSourceLocations>>,
    scope_index: usize,
    lang: &TargetLanguage,
    is_rhs: bool,
) -> Result<Pattern> {
    if !is_rhs {
        bail!("raw snippets are only allowed on the right hand side of a rule");
    }
    let source = node.utf8_text(src.as_bytes())?.to_string();
    let mut range: Range = node.range().into();
    // adjust range by "raw`" and "`"
    range.adjust_columns(4, -1);
    let content = source
        .strip_prefix("raw`")
        .ok_or_else(|| anyhow!("Unable to extract content from raw snippet: {}", source))?
        .strip_suffix('`')
        .ok_or_else(|| anyhow!("Unable to extract content from raw snippet: {}", source))?;
    process_snippet_content(
        content,
        "",
        range,
        vars,
        global_vars,
        vars_array,
        scope_index,
        lang,
        is_rhs,
    )
}

#[allow(clippy::too_many_arguments)]
fn language_specific_snippet(
    node: Node,
    file: &str,
    src: &str,
    vars: &mut BTreeMap<String, usize>,
    global_vars: &mut BTreeMap<String, usize>,
    vars_array: &mut Vec<Vec<VariableSourceLocations>>,
    scope_index: usize,
    lang: &TargetLanguage,
    is_rhs: bool,
) -> Result<Pattern> {
    let lang_node = node
        .child_by_field_name("language")
        .ok_or_else(|| anyhow!("missing language of languageSpecificSnippet"))?;
    let lang_name = lang_node.utf8_text(src.as_bytes())?.trim().to_string();
    let _snippet_lang = TargetLanguage::from_string(&lang_name, None)
        .ok_or_else(|| anyhow!("invalid language: {}", lang_name))?;
    let snippet_node = node
        .child_by_field_name("snippet")
        .ok_or_else(|| anyhow!("missing snippet of languageSpecificSnippet"))?;
    let source = snippet_node.utf8_text(src.as_bytes())?.to_string();
    let mut range: Range = node.range().into();
    range.adjust_columns(1, -1);
    let content = source
        .strip_prefix('"')
        .ok_or_else(|| anyhow!("Unable to extract content from raw snippet: {}", source))?
        .strip_suffix('"')
        .ok_or_else(|| anyhow!("Unable to extract content from raw snippet: {}", source))?;

    process_snippet_content(
        content,
        file,
        range,
        vars,
        global_vars,
        vars_array,
        scope_index,
        lang,
        is_rhs,
    )
}

#[cfg(test)]
mod tests {
    use marzano_language::target_language::PatternLanguage;
    use marzano_util::position::Position;

    use super::*;
    use std::{collections::BTreeMap, vec};

    use crate::pattern::patterns::Pattern;

    #[test]
    fn parse_condition_snippet() {
        let src = "if ($cond) { $cond_true }";
        let lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
        let mut vars = BTreeMap::new();
        let mut global_vars = BTreeMap::new();
        let mut vars_array = vec![vec![]];
        let snippet_trees = lang.parse_snippet_contexts(src);
        let snippet_nodes = nodes_from_indices(&snippet_trees);
        let snippet_patterns: Vec<Pattern> = snippet_nodes
            .into_iter()
            .map(|node| {
                Pattern::from_snippet_node(
                    node,
                    Range {
                        start: Position { line: 1, column: 1 },
                        end: Position { line: 2, column: 1 },
                        start_byte: 0,
                        end_byte: 100,
                    },
                    "",
                    &lang,
                    &mut vars,
                    &mut global_vars,
                    &mut vars_array,
                    0,
                    false,
                )
            })
            .collect::<Result<Vec<Pattern>>>()
            .unwrap();
        assert_eq!(snippet_patterns.len(), 2);
    }
}
