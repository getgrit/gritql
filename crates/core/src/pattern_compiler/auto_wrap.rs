use crate::{optimizer::hoist_files::extract_filename_pattern, problem::MarzanoQueryContext};

use super::compiler::{DefinitionInfo, SnippetCompilationContext};
use anyhow::Result;
use grit_pattern_matcher::{
    constants::{GRIT_RANGE_VAR},
    pattern::{
        And, Bubble, Call, Container, Contains, FilePattern, Includes, Limit, Match, Maybe,
        Pattern, PatternDefinition, PrAnd, PrOr, Predicate, Range as PRange, Rewrite, Step,
        StringConstant, Variable, Where,
    },
};
use grit_util::FileRange;
use log::debug;

pub(crate) fn auto_wrap_pattern(
    pattern: Pattern<MarzanoQueryContext>,
    pattern_definitions: &mut [PatternDefinition<MarzanoQueryContext>],
    is_not_multifile: bool,
    file_ranges: Option<Vec<FileRange>>,
    context: &mut dyn SnippetCompilationContext,
    injected_limit: Option<usize>,
) -> Result<Pattern<MarzanoQueryContext>> {
    let is_sequential = is_sequential(&pattern, pattern_definitions);
    let should_wrap_in_sequential = !is_sequential;
    let should_wrap_in_contains = should_autowrap(&pattern, pattern_definitions);
    let should_wrap_in_file = should_wrap_in_file(&pattern, pattern_definitions);
    let (pattern, extracted_limit) = if should_wrap_in_contains && should_wrap_in_file {
        extract_limit_pattern(pattern, pattern_definitions)
    } else {
        (pattern, None)
    };
    let pattern = if is_not_multifile {
        let pattern = if let Some(ranges) = file_ranges {
            if should_wrap_in_sequential {
                wrap_pattern_in_range(GRIT_RANGE_VAR, pattern, ranges, context)?
            } else {
                pattern
            }
        } else {
            pattern
        };
        let first_wrap = if should_wrap_in_contains {
            wrap_pattern_in_contains(pattern, context)?
        } else {
            pattern
        };
        let second_wrap = if should_wrap_in_file {
            wrap_pattern_in_file(first_wrap)?
        } else {
            first_wrap
        };
        let third_wrap = if let Some(limit) = injected_limit {
            // Strip the limit if there is one
            let (pattern, _) = extract_limit_pattern(second_wrap, pattern_definitions);
            Pattern::Limit(Box::new(Limit::new(pattern, limit)))
        } else if let Some(limit) = extracted_limit {
            Pattern::Limit(Box::new(Limit::new(second_wrap, limit)))
        } else {
            second_wrap
        };
        wrap_pattern_in_before_and_after_each_file(third_wrap, context)?
    } else {
        pattern
    };
    if should_wrap_in_sequential {
        Ok(Pattern::Sequential(vec![Step { pattern }].into()))
    } else {
        Ok(pattern)
    }
}

pub fn is_sequential(
    pattern: &Pattern<MarzanoQueryContext>,
    pattern_definitions: &[PatternDefinition<MarzanoQueryContext>],
) -> bool {
    match pattern {
        Pattern::Sequential(_) => true,
        Pattern::Where(w) => is_sequential(&w.pattern, pattern_definitions),
        Pattern::Maybe(m) => is_sequential(&m.pattern, pattern_definitions),
        Pattern::Rewrite(r) => is_sequential(&r.left, pattern_definitions),
        Pattern::Bubble(b) => is_sequential(b.pattern_def.pattern(), pattern_definitions),
        Pattern::Limit(l) => is_sequential(&l.pattern, pattern_definitions),
        Pattern::Call(call) => is_sequential(
            pattern_definitions[call.index].pattern(),
            pattern_definitions,
        ),
        Pattern::AstNode(_)
        | Pattern::List(_)
        | Pattern::ListIndex(_)
        | Pattern::Map(_)
        | Pattern::Accessor(_)
        | Pattern::Regex(_)
        | Pattern::File(_)
        | Pattern::Files(_)
        | Pattern::CallBuiltIn(_)
        | Pattern::CallFunction(_)
        | Pattern::CallForeignFunction(_)
        | Pattern::CallbackPattern(_)
        | Pattern::Assignment(_)
        | Pattern::Accumulate(_)
        | Pattern::And(_)
        | Pattern::Or(_)
        | Pattern::Any(_)
        | Pattern::Not(_)
        | Pattern::If(_)
        | Pattern::Undefined
        | Pattern::Top
        | Pattern::Bottom
        | Pattern::Underscore
        | Pattern::StringConstant(_)
        | Pattern::AstLeafNode(_)
        | Pattern::IntConstant(_)
        | Pattern::FloatConstant(_)
        | Pattern::BooleanConstant(_)
        | Pattern::Dynamic(_)
        | Pattern::CodeSnippet(_)
        | Pattern::Variable(_)
        | Pattern::Log(_)
        | Pattern::Range(_)
        | Pattern::Contains(_)
        | Pattern::Includes(_)
        | Pattern::Within(_)
        | Pattern::After(_)
        | Pattern::Before(_)
        | Pattern::Some(_)
        | Pattern::Every(_)
        | Pattern::Add(_)
        | Pattern::Subtract(_)
        | Pattern::Multiply(_)
        | Pattern::Divide(_)
        | Pattern::Modulo(_)
        | Pattern::Like(_)
        | Pattern::Dots => false,
    }
}

pub(crate) fn should_autowrap(
    pattern: &Pattern<MarzanoQueryContext>,
    pattern_definitions: &[PatternDefinition<MarzanoQueryContext>],
) -> bool {
    match pattern {
        Pattern::Contains(_) => false,
        Pattern::File(_) => false,
        Pattern::Sequential(_) => false,
        Pattern::Where(w) => should_autowrap(&w.pattern, pattern_definitions),
        Pattern::Maybe(m) => should_autowrap(&m.pattern, pattern_definitions),
        Pattern::Rewrite(r) => should_autowrap(&r.left, pattern_definitions),
        Pattern::Bubble(b) => should_autowrap(b.pattern_def.pattern(), pattern_definitions),
        Pattern::Limit(l) => should_autowrap(&l.pattern, pattern_definitions),
        Pattern::Call(call) => should_autowrap(
            pattern_definitions[call.index].pattern(),
            pattern_definitions,
        ),
        Pattern::AstNode(_)
        | Pattern::List(_)
        | Pattern::ListIndex(_)
        | Pattern::Map(_)
        | Pattern::Accessor(_)
        | Pattern::Regex(_)
        | Pattern::Files(_)
        | Pattern::CallBuiltIn(_)
        | Pattern::CallFunction(_)
        | Pattern::CallForeignFunction(_)
        | Pattern::CallbackPattern(_)
        | Pattern::Assignment(_)
        | Pattern::Accumulate(_)
        | Pattern::And(_)
        | Pattern::Or(_)
        | Pattern::Any(_)
        | Pattern::Not(_)
        | Pattern::If(_)
        | Pattern::Undefined
        | Pattern::Top
        | Pattern::Bottom
        | Pattern::Underscore
        | Pattern::StringConstant(_)
        | Pattern::AstLeafNode(_)
        | Pattern::IntConstant(_)
        | Pattern::FloatConstant(_)
        | Pattern::BooleanConstant(_)
        | Pattern::Dynamic(_)
        | Pattern::CodeSnippet(_)
        | Pattern::Variable(_)
        | Pattern::Log(_)
        | Pattern::Range(_)
        | Pattern::Includes(_)
        | Pattern::Within(_)
        | Pattern::After(_)
        | Pattern::Before(_)
        | Pattern::Some(_)
        | Pattern::Every(_)
        | Pattern::Add(_)
        | Pattern::Subtract(_)
        | Pattern::Multiply(_)
        | Pattern::Divide(_)
        | Pattern::Modulo(_)
        | Pattern::Like(_)
        | Pattern::Dots => true,
    }
}

fn extract_limit_pattern(
    pattern: Pattern<MarzanoQueryContext>,
    pattern_definitions: &mut [PatternDefinition<MarzanoQueryContext>],
) -> (Pattern<MarzanoQueryContext>, Option<usize>) {
    match pattern {
        Pattern::Limit(limit) => (limit.pattern, Some(limit.limit)),
        Pattern::Where(w) => {
            let extracted = extract_limit_pattern(w.pattern, pattern_definitions);
            let pattern = Pattern::Where(Box::new(Where::new(extracted.0, w.side_condition)));
            (pattern, extracted.1)
        }
        Pattern::Maybe(m) => {
            let extracted = extract_limit_pattern(m.pattern, pattern_definitions);
            let pattern = Pattern::Maybe(Box::new(Maybe::new(extracted.0)));
            (pattern, extracted.1)
        }
        Pattern::Rewrite(r) => {
            let extracted = extract_limit_pattern(r.left, pattern_definitions);
            let pattern =
                Pattern::Rewrite(Box::new(Rewrite::new(extracted.0, r.right, r.annotation)));
            (pattern, extracted.1)
        }
        Pattern::Bubble(b) => {
            let extracted =
                extract_limit_pattern(b.pattern_def.pattern().clone(), pattern_definitions);
            let pattern = Pattern::Bubble(Box::new(Bubble::new(
                PatternDefinition::new(
                    b.pattern_def.name.clone(),
                    b.pattern_def.try_scope().unwrap(),
                    b.pattern_def.params().clone(),
                    extracted.0,
                ),
                b.args.into_iter().flatten().collect(),
            )));
            (pattern, extracted.1)
        }
        Pattern::Call(call) => {
            let (new_pattern, extracted_limit) = extract_limit_pattern(
                pattern_definitions[call.index].pattern().clone(),
                pattern_definitions,
            );
            pattern_definitions[call.index].replace_pattern(new_pattern);
            (Pattern::Call(call), extracted_limit)
        }
        Pattern::AstNode(_)
        | Pattern::File(_)
        | Pattern::Contains(_)
        | Pattern::Sequential(_)
        | Pattern::List(_)
        | Pattern::ListIndex(_)
        | Pattern::Map(_)
        | Pattern::Accessor(_)
        | Pattern::Regex(_)
        | Pattern::Files(_)
        | Pattern::CallBuiltIn(_)
        | Pattern::CallFunction(_)
        | Pattern::CallForeignFunction(_)
        | Pattern::Assignment(_)
        | Pattern::Accumulate(_)
        | Pattern::And(_)
        | Pattern::Or(_)
        | Pattern::Any(_)
        | Pattern::Not(_)
        | Pattern::If(_)
        | Pattern::Undefined
        | Pattern::Top
        | Pattern::Bottom
        | Pattern::Underscore
        | Pattern::StringConstant(_)
        | Pattern::AstLeafNode(_)
        | Pattern::IntConstant(_)
        | Pattern::FloatConstant(_)
        | Pattern::BooleanConstant(_)
        | Pattern::Dynamic(_)
        | Pattern::CodeSnippet(_)
        | Pattern::Variable(_)
        | Pattern::Log(_)
        | Pattern::Range(_)
        | Pattern::Includes(_)
        | Pattern::Within(_)
        | Pattern::After(_)
        | Pattern::Before(_)
        | Pattern::Some(_)
        | Pattern::Every(_)
        | Pattern::Add(_)
        | Pattern::Subtract(_)
        | Pattern::Multiply(_)
        | Pattern::Divide(_)
        | Pattern::Modulo(_)
        | Pattern::Like(_)
        | Pattern::Dots => (pattern, None),
        Pattern::CallbackPattern(_) => (pattern, None),
    }
}

pub fn should_wrap_in_file(
    pattern: &Pattern<MarzanoQueryContext>,
    pattern_definitions: &[PatternDefinition<MarzanoQueryContext>],
) -> bool {
    match pattern {
        Pattern::File(_) => false,
        Pattern::Files(_) => false,
        Pattern::Sequential(_) => false,
        Pattern::Where(w) => should_wrap_in_file(&w.pattern, pattern_definitions),
        Pattern::Maybe(m) => should_wrap_in_file(&m.pattern, pattern_definitions),
        Pattern::Rewrite(r) => should_wrap_in_file(&r.left, pattern_definitions),
        Pattern::Bubble(b) => should_wrap_in_file(b.pattern_def.pattern(), pattern_definitions),
        Pattern::Limit(l) => should_wrap_in_file(&l.pattern, pattern_definitions),
        Pattern::Call(call) => should_wrap_in_file(
            pattern_definitions[call.index].pattern(),
            pattern_definitions,
        ),
        Pattern::And(a) => a
            .patterns
            .iter()
            .all(|c| should_wrap_in_file(c, pattern_definitions)),
        Pattern::Or(o) => o
            .patterns
            .iter()
            .all(|c| should_wrap_in_file(c, pattern_definitions)),
        Pattern::Any(a) => a
            .patterns
            .iter()
            .all(|c| should_wrap_in_file(c, pattern_definitions)),
        Pattern::Not(n) => should_wrap_in_file(&n.pattern, pattern_definitions),
        Pattern::AstNode(_)
        | Pattern::Contains(_)
        | Pattern::List(_)
        | Pattern::ListIndex(_)
        | Pattern::Map(_)
        | Pattern::Accessor(_)
        | Pattern::Regex(_)
        | Pattern::CallBuiltIn(_)
        | Pattern::CallFunction(_)
        | Pattern::CallForeignFunction(_)
        | Pattern::CallbackPattern(_)
        | Pattern::Assignment(_)
        | Pattern::Accumulate(_)
        | Pattern::If(_)
        | Pattern::Undefined
        | Pattern::Top
        | Pattern::Bottom
        | Pattern::Underscore
        | Pattern::StringConstant(_)
        | Pattern::AstLeafNode(_)
        | Pattern::IntConstant(_)
        | Pattern::FloatConstant(_)
        | Pattern::BooleanConstant(_)
        | Pattern::Dynamic(_)
        | Pattern::CodeSnippet(_)
        | Pattern::Variable(_)
        | Pattern::Log(_)
        | Pattern::Range(_)
        | Pattern::Includes(_)
        | Pattern::Within(_)
        | Pattern::After(_)
        | Pattern::Before(_)
        | Pattern::Some(_)
        | Pattern::Every(_)
        | Pattern::Add(_)
        | Pattern::Subtract(_)
        | Pattern::Multiply(_)
        | Pattern::Divide(_)
        | Pattern::Modulo(_)
        | Pattern::Like(_)
        | Pattern::Dots => true,
    }
}

fn wrap_pattern_in_range(
    var_name: &str,
    pattern: Pattern<MarzanoQueryContext>,
    ranges: Vec<FileRange>,
    context: &mut dyn SnippetCompilationContext,
) -> Result<Pattern<MarzanoQueryContext>> {
    let var = context.register_variable(var_name, None)?;
    let mut predicates = Vec::new();
    for file_range in ranges {
        let range = file_range.range.clone();
        let range = PRange::from(range);
        let range_match = Predicate::Match(Box::new(Match::new(
            Container::Variable(var.clone()),
            Some(Pattern::Range(range)),
        )));
        let file_match = Predicate::Match(Box::new(Match::new(
            Container::Variable(Variable::file_name()),
            Some(Pattern::Includes(Box::new(Includes::new(
                Pattern::StringConstant(StringConstant::new(
                    file_range.file_path.to_string_lossy().to_string(),
                )),
            )))),
        )));
        predicates.push(Predicate::And(Box::new(PrAnd::new(vec![
            file_match,
            range_match,
        ]))));
    }
    let pattern = Pattern::Where(Box::new(Where::new(
        pattern,
        Predicate::Or(Box::new(PrOr::new(predicates))),
    )));
    let pattern = Pattern::Where(Box::new(Where::new(
        Pattern::Variable(var.clone()),
        Predicate::Match(Box::new(Match::new(
            Container::Variable(var.clone()),
            Some(pattern),
        ))),
    )));
    Ok(pattern)
}

fn wrap_pattern_in_contains(
    pattern: Pattern<MarzanoQueryContext>,
    context: &mut dyn SnippetCompilationContext,
) -> Result<Pattern<MarzanoQueryContext>> {
    let pattern = if let Ok(var) = context.register_match_variable() {
        Pattern::Where(Box::new(Where::new(
            Pattern::Variable(var.clone()),
            Predicate::Match(Box::new(Match::new(
                Container::Variable(var.clone()),
                Some(pattern),
            ))),
        )))
    } else {
        pattern
    };

    let pattern_definition = context.register_ephemeral_pattern(pattern)?;
    let bubble = Pattern::Bubble(Box::new(Bubble::new(pattern_definition, vec![])));
    Ok(Pattern::Contains(Box::new(Contains::new(bubble, None))))
}

/// Wraps the pattern in a file pattern, so it can match directly against files
/// This also handles optimizing the pattern to avoid unnecessary file loading by hoisting $filename matches
///
/// For example:
/// ```grit
/// `console.log($foo)` where {
///   $filename <: includes "foo.js"
/// }
/// ```
///
/// Will become:
/// ```grit
/// file(name=includes "foo.js") {
///  `console.log($foo)` where {
///   $filename <: includes "foo.js"
///   }
/// }
/// ```
fn wrap_pattern_in_file(
    pattern: Pattern<MarzanoQueryContext>,
) -> Result<Pattern<MarzanoQueryContext>> {
    let filename_pattern = extract_filename_pattern(&pattern)?.unwrap_or_else(|| {
        debug!("Optimization skipped: no filename pattern found, wrapping in top pattern");
        Pattern::Top
    });

    let pattern = Pattern::File(Box::new(FilePattern::new(filename_pattern, pattern)));
    Ok(pattern)
}

pub(crate) fn wrap_pattern_in_before_and_after_each_file(
    pattern: Pattern<MarzanoQueryContext>,
    context: &mut dyn SnippetCompilationContext,
) -> Result<Pattern<MarzanoQueryContext>> {
    let before_each_file = "before_each_file";
    let after_each_file = "after_each_file";
    let mut all_steps = vec![];
    if let Some(DefinitionInfo {
        index,
        parameters: _,
    }) = context.get_pattern_definition(before_each_file)
    {
        all_steps.push(Pattern::Call(Box::new(Call::new(*index, vec![]))));
    }

    all_steps.push(pattern);
    if let Some(DefinitionInfo {
        index,
        parameters: _,
    }) = context.get_pattern_definition(after_each_file)
    {
        all_steps.push(Pattern::Call(Box::new(Call::new(*index, vec![]))));
    }

    let final_pattern = if all_steps.len() > 1 {
        Pattern::And(Box::new(And::new(all_steps)))
    } else {
        all_steps.pop().unwrap()
    };

    Ok(final_pattern)
}
