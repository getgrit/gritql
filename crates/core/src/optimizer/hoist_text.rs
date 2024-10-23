use anyhow::Result;
use grit_pattern_matcher::{
    context::QueryContext,
    pattern::{
        And, Any, Bubble, Contains, Includes, Or, Pattern, Predicate, StringConstant, Where,
    },
};

use super::hoist_files::is_safe_to_hoist;
use grit_pattern_matcher::pattern::AstLeafNodePattern;
use grit_pattern_matcher::pattern::CodeSnippet;

/// This file implements an optimization pass to look for *string* identifiers that the pattern is searching for
/// If any are found, we "hoist" them to skip parsing/traversing files entirely and instead just look for the text
/// Searching inside text is more optimized than matching on the AST
///
/// Note this is moslty copied from hoist_files.rs, we can DRY it later if a 3rd optimizer is added.

trait BodyPatternExtractor<Q: QueryContext> {
    fn extract_body_pattern(&self) -> Result<Option<Pattern<Q>>>;
}

/// Extracts the *text* patterns from a pattern
/// This should produce a pattern we could use for includes
fn extract_pattern_text<Q: QueryContext>(pattern: &Pattern<Q>) -> Result<Option<Pattern<Q>>> {
    match pattern {
        Pattern::StringConstant(_) => Ok(Some(pattern.clone())),
        Pattern::CodeSnippet(snippet) => {
            let patterns: Vec<_> = snippet
                .patterns()
                .map(|p| extract_pattern_text(p))
                .collect::<Result<Vec<_>>>()?;

            if patterns.iter().any(|p| p.is_none()) {
                return Ok(None);
            }

            let patterns = patterns.into_iter().map(|p| p.unwrap()).collect();

            Ok(Some(Pattern::Or(Box::new(Or::new(patterns)))))
        }
        Pattern::AstLeafNode(node) => Ok(node
            .text()
            .map(|s| Pattern::StringConstant(StringConstant::new(s.to_string())))),
        _ => Ok(None),
    }
}

pub fn extract_body_pattern<Q: QueryContext>(pattern: &Pattern<Q>) -> Result<Option<Pattern<Q>>> {
    match pattern {
        Pattern::Variable(_)
        | Pattern::CodeSnippet(_)
        | Pattern::Range(_)
        | Pattern::Top
        | Pattern::Undefined
        | Pattern::Underscore
        | Pattern::StringConstant(_)
        | Pattern::AstLeafNode(_)
        | Pattern::IntConstant(_)
        | Pattern::Bottom => Ok(Some(Pattern::Top)),

        // Traversing downwards, collecting patterns
        Pattern::Contains(c) => c.extract_body_pattern(),
        Pattern::Bubble(b) => b.extract_body_pattern(),
        Pattern::Where(w) => w.extract_body_pattern(),
        Pattern::Rewrite(rw) => extract_body_pattern(&rw.left),
        Pattern::Includes(inc) => extract_body_pattern(&inc.includes),
        Pattern::Every(every) => extract_body_pattern(&every.pattern),
        Pattern::Within(within) => extract_body_pattern(&within.pattern),
        Pattern::After(a) => extract_body_pattern(&a.after),
        Pattern::Before(b) => extract_body_pattern(&b.before),

        // Mirror existing logic
        Pattern::Maybe(_) => Ok(Some(Pattern::Top)),
        Pattern::And(target) => {
            let Some(patterns) = extract_body_patterns_from_patterns(&target.patterns)? else {
                return Ok(None);
            };
            Ok(Some(Pattern::And(Box::new(And::new(patterns)))))
        }
        Pattern::Or(target) => {
            let Some(patterns) = extract_body_patterns_from_patterns(&target.patterns)? else {
                return Ok(None);
            };
            Ok(Some(Pattern::Or(Box::new(Or::new(patterns)))))
        }
        Pattern::Any(target) => {
            let Some(patterns) = extract_body_patterns_from_patterns(&target.patterns)? else {
                return Ok(None);
            };
            Ok(Some(Pattern::Any(Box::new(Any::new(patterns)))))
        }
        Pattern::Some(some) => extract_body_pattern(&some.pattern),

        Pattern::Log(_) => Ok(Some(Pattern::Top)),

        Pattern::Add(add) => {
            let Some(lhs) = extract_body_pattern(&add.lhs)? else {
                return Ok(None);
            };
            let Some(rhs) = extract_body_pattern(&add.rhs)? else {
                return Ok(None);
            };
            Ok(Some(Pattern::And(Box::new(And::new(vec![lhs, rhs])))))
        }
        Pattern::Subtract(sub) => {
            let Some(lhs) = extract_body_pattern(&sub.lhs)? else {
                return Ok(None);
            };
            let Some(rhs) = extract_body_pattern(&sub.rhs)? else {
                return Ok(None);
            };
            Ok(Some(Pattern::And(Box::new(And::new(vec![lhs, rhs])))))
        }
        Pattern::Multiply(target) => {
            let Some(lhs) = extract_body_pattern(&target.lhs)? else {
                return Ok(None);
            };
            let Some(rhs) = extract_body_pattern(&target.rhs)? else {
                return Ok(None);
            };
            Ok(Some(Pattern::And(Box::new(And::new(vec![lhs, rhs])))))
        }
        Pattern::Divide(target) => {
            let Some(lhs) = extract_body_pattern(&target.lhs)? else {
                return Ok(None);
            };
            let Some(rhs) = extract_body_pattern(&target.rhs)? else {
                return Ok(None);
            };
            Ok(Some(Pattern::And(Box::new(And::new(vec![lhs, rhs])))))
        }
        Pattern::Modulo(target) => {
            let Some(lhs) = extract_body_pattern(&target.lhs)? else {
                return Ok(None);
            };
            let Some(rhs) = extract_body_pattern(&target.rhs)? else {
                return Ok(None);
            };
            Ok(Some(Pattern::And(Box::new(And::new(vec![lhs, rhs])))))
        }

        // TODO: decide the rest of these
        Pattern::Dots
        | Pattern::Sequential(_)
        | Pattern::Like(_)
        | Pattern::AstNode(_)
        | Pattern::List(_)
        | Pattern::ListIndex(_)
        | Pattern::Map(_)
        | Pattern::Accessor(_)
        | Pattern::Call(_)
        | Pattern::Regex(_)
        | Pattern::File(_)
        | Pattern::Files(_)
        | Pattern::Limit(_)
        | Pattern::CallBuiltIn(_)
        | Pattern::CallFunction(_)
        | Pattern::CallForeignFunction(_)
        | Pattern::CallbackPattern(_)
        | Pattern::Assignment(_)
        | Pattern::Accumulate(_)
        | Pattern::Not(_)
        | Pattern::If(_)
        | Pattern::FloatConstant(_)
        | Pattern::BooleanConstant(_)
        | Pattern::Dynamic(_) => Ok(None),
    }
}

impl<Q: QueryContext> BodyPatternExtractor<Q> for Bubble<Q> {
    fn extract_body_pattern(&self) -> Result<Option<Pattern<Q>>> {
        extract_body_pattern(self.pattern_def.pattern())
    }
}

impl<Q: QueryContext> BodyPatternExtractor<Q> for Contains<Q> {
    fn extract_body_pattern(&self) -> Result<Option<Pattern<Q>>> {
        extract_body_pattern(&self.contains)
    }
}

impl<Q: QueryContext> BodyPatternExtractor<Q> for Includes<Q> {
    fn extract_body_pattern(&self) -> Result<Option<Pattern<Q>>> {
        extract_body_pattern(&self.includes)
    }
}

impl<Q: QueryContext> BodyPatternExtractor<Q> for Where<Q> {
    fn extract_body_pattern(&self) -> Result<Option<Pattern<Q>>> {
        let pattern = extract_body_pattern(&self.pattern)?.unwrap_or(Pattern::Top);
        let predicate_pattern = self
            .side_condition
            .extract_body_pattern()?
            .unwrap_or(Pattern::Top);
        Ok(Some(Pattern::And(Box::new(And::new(vec![
            pattern,
            predicate_pattern,
        ])))))
    }
}

fn extract_body_patterns_from_patterns<Q: QueryContext>(
    in_patterns: &[Pattern<Q>],
) -> Result<Option<Vec<Pattern<Q>>>> {
    let mut patterns = vec![];
    for p in in_patterns {
        let pattern = extract_body_pattern(p)?;
        if let Some(pattern) = pattern {
            patterns.push(pattern);
        } else {
            return Ok(None);
        }
    }
    Ok(Some(patterns))
}

fn extract_patterns_from_predicates<Q: QueryContext>(
    predicates: &[Predicate<Q>],
) -> Result<Option<Vec<Pattern<Q>>>> {
    let mut patterns = vec![];
    for p in predicates {
        let pattern = p.extract_body_pattern()?;
        if let Some(pattern) = pattern {
            patterns.push(pattern);
        } else {
            return Ok(None);
        }
    }
    Ok(Some(patterns))
}

impl<Q: QueryContext> BodyPatternExtractor<Q> for Predicate<Q> {
    fn extract_body_pattern(&self) -> Result<Option<Pattern<Q>>> {
        match self {
            Predicate::And(target) => {
                let Some(patterns) = extract_patterns_from_predicates(&target.predicates)? else {
                    return Ok(None);
                };
                Ok(Some(Pattern::And(Box::new(And::new(patterns)))))
            }
            Predicate::Or(target) => {
                let Some(patterns) = extract_patterns_from_predicates(&target.predicates)? else {
                    return Ok(None);
                };
                Ok(Some(Pattern::Or(Box::new(Or::new(patterns)))))
            }
            Predicate::Any(target) => {
                let Some(patterns) = extract_patterns_from_predicates(&target.predicates)? else {
                    return Ok(None);
                };
                Ok(Some(Pattern::Any(Box::new(Any::new(patterns)))))
            }
            Predicate::Match(m) => {
                match &m.val {
                    grit_pattern_matcher::pattern::Container::Variable(var) => {
                        if var.is_program() {
                            match &m.pattern {
                                Some(pattern) => {
                                    // This is the key line of this entire file
                                    if is_safe_to_hoist(pattern)? {
                                        let modified_pattern = match pattern {
                                            Pattern::Contains(c) => {
                                                let text = extract_pattern_text(&c.contains)?;
                                                text.map(|text| {
                                                    Pattern::Includes(Box::new(Includes::new(text)))
                                                })
                                            }
                                            Pattern::AstNode(_)
                                            | Pattern::List(_)
                                            | Pattern::ListIndex(_)
                                            | Pattern::Map(_)
                                            | Pattern::Accessor(_)
                                            | Pattern::Call(_)
                                            | Pattern::Regex(_)
                                            | Pattern::File(_)
                                            | Pattern::Files(_)
                                            | Pattern::Bubble(_)
                                            | Pattern::Limit(_)
                                            | Pattern::CallBuiltIn(_)
                                            | Pattern::CallFunction(_)
                                            | Pattern::CallForeignFunction(_)
                                            | Pattern::CallbackPattern(_)
                                            | Pattern::Assignment(_)
                                            | Pattern::Accumulate(_)
                                            | Pattern::And(_)
                                            | Pattern::Or(_)
                                            | Pattern::Maybe(_)
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
                                            | Pattern::Rewrite(_)
                                            | Pattern::Log(_)
                                            | Pattern::Range(_)
                                            | Pattern::Includes(_)
                                            | Pattern::Within(_)
                                            | Pattern::After(_)
                                            | Pattern::Before(_)
                                            | Pattern::Where(_)
                                            | Pattern::Some(_)
                                            | Pattern::Every(_)
                                            | Pattern::Add(_)
                                            | Pattern::Subtract(_)
                                            | Pattern::Multiply(_)
                                            | Pattern::Divide(_)
                                            | Pattern::Modulo(_)
                                            | Pattern::Dots
                                            | Pattern::Sequential(_)
                                            | Pattern::Like(_) => None,
                                        };
                                        return Ok(modified_pattern);
                                    } else {
                                        return Ok(None);
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                    grit_pattern_matcher::pattern::Container::Accessor(_)
                    | grit_pattern_matcher::pattern::Container::ListIndex(_)
                    | grit_pattern_matcher::pattern::Container::FunctionCall(_) => {}
                };

                match &m.pattern {
                    Some(pattern) => extract_body_pattern(pattern),
                    // TODO: is this right? Why do we ever have an empty pattern?
                    None => Ok(None),
                }
            }
            Predicate::Accumulate(_) | Predicate::Assignment(_) | Predicate::Return(_) => {
                Ok(Some(Pattern::Top))
            }

            Predicate::Rewrite(rw) => extract_body_pattern(&rw.left),
            Predicate::Log(_) => Ok(Some(Pattern::Top)),

            // If we hit a leaf predicate that is *not* a match, stop traversing - it is always true
            Predicate::True => Ok(Some(Pattern::Top)),
            Predicate::False => Ok(None),

            // We can safely ignore any maybe
            Predicate::Maybe(_) => Ok(Some(Pattern::Top)),

            // Look for predicates in the condition, left, and right
            // Either we need both the condition and the left to be true
            // OR we need the right to be true
            Predicate::If(target) => {
                let Some(condition) = target.if_.extract_body_pattern()? else {
                    return Ok(None);
                };
                let Some(then) = target.then.extract_body_pattern()? else {
                    return Ok(None);
                };
                let Some(else_) = target.else_.extract_body_pattern()? else {
                    return Ok(None);
                };
                Ok(Some(Pattern::Or(Box::new(Or::new(vec![
                    Pattern::And(Box::new(And::new(vec![condition, then]))),
                    else_,
                ])))))
            }

            // These are more complicated, implement carefully
            Predicate::Call(_) | Predicate::Not(_) | Predicate::Equal(_) => Ok(None),
        }
    }
}

mod test {
    use std::collections::BTreeMap;

    use marzano_language::target_language::TargetLanguage;

    use crate::{
        api::MatchResult,
        pattern_compiler::src_to_problem_libs,
        test_utils::{run_on_test_files, SyntheticFile},
    };

    #[test]
    fn test_basic_file_includes_no_parse() {
        let libs = BTreeMap::new();

        // All together now
        let test_files = vec![
            SyntheticFile::new(
                "target.js".to_owned(),
                r#"
        console.log("Hello, world!");
        funcify()
        "#
                .to_owned(),
                true,
            ),
            SyntheticFile::new(
                "do_not_traverse.js".to_owned(),
                r#"
                // this does not include the magic word
                funcify()
                "#
                .to_owned(),
                true,
            ),
            SyntheticFile::new(
                "do_traverse.js".to_owned(),
                r#"
                // this does mention console, but it does not match
                funcify()
                "#
                .to_owned(),
                true,
            ),
        ];

        // Test with a pattern that short circuits the contains callback
        let pattern = src_to_problem_libs(
            r#"
        `funcify` where {
            // all 3 files *do* include funcify, so a naive implementation would log for all 3
            log(message="This was reached"),
            // but the hoisting of this condition ensures we don't actually traverse all 3
            $program <: contains `console`
        }
        "#
            .to_string(),
            &libs,
            TargetLanguage::default(),
            None,
            None,
            None,
            None,
        )
        .unwrap()
        .problem;

        println!("Pattern: {:?}", pattern);

        let results = run_on_test_files(&pattern, &test_files);
        println!("{:?}", results);
        assert!(results.iter().any(|r| r.is_match()));
        assert_eq!(
            results
                .iter()
                .filter(|r| matches!(r, MatchResult::AnalysisLog(_)))
                .count(),
            2
        );
    }
}
