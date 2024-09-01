use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    variable::Variable,
    State,
};
use crate::{
    binding::Binding,
    context::{ExecContext, QueryContext},
};
use core::fmt::Debug;
use grit_util::{
    error::{GritPatternError, GritResult},
    AnalysisLogs,
};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct RegexPattern<Q: QueryContext> {
    pub regex: RegexLike<Q>,
    pub variables: Vec<Variable>,
}

#[derive(Debug, Clone)]
pub enum RegexLike<Q: QueryContext> {
    Regex(String),
    Pattern(Box<Pattern<Q>>),
}

impl<Q: QueryContext> RegexPattern<Q> {
    pub fn new(regex: RegexLike<Q>, variables: Vec<Variable>) -> Self {
        Self { regex, variables }
    }

    pub(crate) fn execute_matching<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
        must_match_entire_string: bool,
    ) -> GritResult<bool> {
        let text = binding.text(&state.files, context.language())?;
        let resolved_regex_text = match &self.regex {
            RegexLike::Regex(regex) => match must_match_entire_string {
                true => format!("^{}$", regex),
                false => regex.to_string(),
            },
            RegexLike::Pattern(ref pattern) => {
                let resolved = Q::ResolvedPattern::from_pattern(pattern, state, context, logs)?;
                let text = resolved.text(&state.files, context.language())?;
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
            return Err(GritPatternError::new(format!(
                "regex pattern matched {} variables, but expected {}",
                captures.len() - 1,
                self.variables.len()
            )));
        }
        // why not zip?
        for (i, variable) in self.variables.iter().enumerate() {
            let value = captures
                .get(i + 1)
                .ok_or_else(|| GritPatternError::new("missing capture group"))?;

            let range = value.range();
            let value = value.as_str();

            // we should really be making the resolved pattern first, and using
            // variable execute, instead of reimplementing here.
            let variable_content =
                &mut state.bindings[variable.scope].back_mut().unwrap()[variable.index];

            if let Some(previous_value) = &variable_content.value {
                if previous_value
                    .text(&state.files, context.language())
                    .unwrap()
                    != value
                {
                    return Ok(false);
                } else {
                    continue;
                }
            } else {
                let res = if let Some((Some(mut byte_range), Some(source))) = binding
                    .get_last_binding()
                    .map(|binding| (binding.range(context.language()), binding.source()))
                {
                    byte_range.end = byte_range.start + range.end;
                    byte_range.start += range.start;
                    Q::ResolvedPattern::from_range_binding(byte_range, source)
                } else {
                    Q::ResolvedPattern::from_string(value.to_string())
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

impl<Q: QueryContext> PatternName for RegexPattern<Q> {
    fn name(&self) -> &'static str {
        "REGEX"
    }
}

impl<Q: QueryContext> Matcher<Q> for RegexPattern<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        self.execute_matching(binding, state, context, logs, true)
    }
}
