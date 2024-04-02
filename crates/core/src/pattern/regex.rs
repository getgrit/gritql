use super::{
    code_snippet::from_back_tick_node,
    compiler::CompilationContext,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    variable::{Variable, VariableSourceLocations},
    State,
};
use crate::context::Context;
use anyhow::{anyhow, bail, Result};
use core::fmt::Debug;
use marzano_language::{language::Language, target_language::TargetLanguage};
use marzano_util::analysis_logs::{AnalysisLogBuilder, AnalysisLogs};
use marzano_util::position::Range;
use regex::Regex;
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct RegexPattern {
    pub regex: RegexLike,
    pub variables: Vec<Variable>,
}

#[derive(Debug, Clone)]
pub enum RegexLike {
    Regex(String),
    Pattern(Box<Pattern>),
}

impl RegexPattern {
    pub fn new(regex: RegexLike, variables: Vec<Variable>) -> Self {
        Self { regex, variables }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        global_vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        lang: &TargetLanguage,
        is_rhs: bool,
        logs: &mut AnalysisLogs,
    ) -> Result<Pattern> {
        if is_rhs {
            bail!("regex patterns are not allowed on the right-hand side of a rule")
        }
        let regex_node = node
            .child_by_field_name("regex")
            .ok_or_else(|| anyhow!("malformed regex, check the parser"))?;

        let regex = if regex_node.kind() == "regex" {
            let regex = regex_node
                .utf8_text(context.src.as_bytes())?
                .trim()
                .to_string();
            let regex = regex
                .strip_prefix("r\"")
                .ok_or_else(|| anyhow!("invalid regex prefix"))?
                .strip_suffix('\"')
                .ok_or_else(|| anyhow!("invalid regex postfix"))?;

            RegexLike::Regex(regex.to_string())
        } else {
            let back_tick_node = regex_node
                .child_by_field_name("snippet")
                .ok_or_else(|| anyhow!("malformed regex, check the parser"))?;
            let regex = regex_node
                .utf8_text(context.src.as_bytes())?
                .trim()
                .to_string();
            if !lang.metavariable_regex().is_match(&regex) {
                let range: Range = regex_node.range().into();
                let alternative = format!(
                    "r\"{}\"",
                    regex
                        .strip_prefix("r`")
                        .ok_or_else(|| anyhow!("invalid regex prefix"))?
                        .strip_suffix('`')
                        .ok_or_else(|| anyhow!("invalid regex postfix"))?
                );
                let log = AnalysisLogBuilder::default()
                .level(441_u16)
                .file(context.file)
                .source(context.src)
                .position(range.start)
                .range(range)
                .message(
                    format!("Warning: unnecessary use of metavariable snippet syntax without metavariables. Replace {regex} with {alternative}"))
                .build()?;
                logs.push(log);
            }
            let pattern = from_back_tick_node(
                back_tick_node,
                context.file,
                context.src,
                vars,
                global_vars,
                vars_array,
                scope_index,
                lang,
                is_rhs,
            )?;
            RegexLike::Pattern(Box::new(pattern))
        };

        let mut cursor = node.walk();
        let variables = node
            .children_by_field_name("variables", &mut cursor)
            .filter(|n| n.is_named())
            .map(|n| {
                Variable::from_node(
                    &n,
                    context.file,
                    context.src,
                    vars,
                    global_vars,
                    vars_array,
                    scope_index,
                )
                .unwrap()
            });

        let variables: Vec<_> = variables.collect();

        Ok(Pattern::Regex(Box::new(RegexPattern::new(
            regex, variables,
        ))))
    }

    pub(crate) fn execute_matching<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
        must_match_entire_string: bool,
    ) -> Result<bool> {
        let text = binding.text(&state.files)?;
        let resolved_regex_text = match &self.regex {
            RegexLike::Regex(regex) => match must_match_entire_string {
                true => format!("^{}$", regex),
                false => regex.to_string(),
            },
            RegexLike::Pattern(ref pattern) => {
                let resolved = ResolvedPattern::from_pattern(pattern, state, context, logs)?;
                let text = resolved.text(&state.files)?;
                match must_match_entire_string {
                    true => format!("^{}$", text),
                    false => text.to_string(),
                }
            }
        };
        let final_regex = Regex::new(&resolved_regex_text)?;
        let captures = match final_regex.captures(&text) {
            Some(captures) => captures,
            None => return Ok(false),
        };

        // todo: make sure the entire string is matched

        if captures.len() != self.variables.len() + 1 {
            bail!(
                "regex pattern matched {} variables, but expected {}",
                captures.len() - 1,
                self.variables.len()
            )
        }
        // why not zip?
        for (i, variable) in self.variables.iter().enumerate() {
            let value = captures
                .get(i + 1)
                .ok_or_else(|| anyhow!("missing capture group"))?;

            let range = value.range();
            let value = value.as_str();

            // we should really be making the resolved pattern first, and using
            // variable execute, instead of reimplementing here.
            let variable_content =
                &mut state.bindings[variable.scope].back_mut().unwrap()[variable.index];

            if let Some(previous_value) = &variable_content.value {
                if previous_value.text(&state.files).unwrap() != value {
                    return Ok(false);
                } else {
                    continue;
                }
            } else {
                let res = if let ResolvedPattern::Binding(binding) = binding {
                    if let Some(binding) = binding.last() {
                        if let (Some(mut position), Some(source)) =
                            (binding.position(), binding.source())
                        {
                            // this moves the byte-range out of sync with
                            // the row-col range, maybe we should just
                            // have a Range<usize> for String bindings?
                            position.end_byte = position.start_byte + range.end as u32;
                            position.start_byte += range.start as u32;
                            ResolvedPattern::from_range(position, source)
                        } else {
                            ResolvedPattern::from_string(value.to_string())
                        }
                    } else {
                        bail!("binding has no binding")
                    }
                } else {
                    ResolvedPattern::from_string(value.to_string())
                };
                if let Some(pattern) = variable_content.pattern {
                    if !pattern.execute(&res, state, context, logs)? {
                        return Ok(false);
                    }
                }
                let variable_content =
                    &mut state.bindings[variable.scope].back_mut().unwrap()[variable.index];
                variable_content.set_value(res);
            }
        }

        Ok(true)
    }
}

impl Name for RegexPattern {
    fn name(&self) -> &'static str {
        "REGEX"
    }
}

impl Matcher for RegexPattern {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        self.execute_matching(binding, state, context, logs, true)
    }
}
