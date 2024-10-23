use anyhow::Result;
use grit_pattern_matcher::{
    context::QueryContext,
    pattern::{And, Any, Bubble, Contains, Includes, Or, Pattern, Predicate, Where},
};

trait FilenamePatternExtractor<Q: QueryContext> {
    fn extract_filename_pattern(&self) -> Result<Option<Pattern<Q>>>;
}

/// Given a pattern, construct a new pattern that reflects any filename predicates found
/// If analysis cannot be done reliably, returns None
pub fn extract_filename_pattern<Q: QueryContext>(
    pattern: &Pattern<Q>,
) -> Result<Option<Pattern<Q>>> {
    match pattern {
        // Once we hit a leaf node that is *not* matched against the filename, we can't go any further
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
        Pattern::Contains(c) => c.extract_filename_pattern(),
        Pattern::Bubble(b) => b.extract_filename_pattern(),
        Pattern::Where(w) => w.extract_filename_pattern(),
        Pattern::Rewrite(rw) => extract_filename_pattern(&rw.left),
        Pattern::Includes(inc) => extract_filename_pattern(&inc.includes),
        Pattern::Every(every) => extract_filename_pattern(&every.pattern),
        Pattern::Within(within) => extract_filename_pattern(&within.pattern),
        Pattern::After(a) => extract_filename_pattern(&a.after),
        Pattern::Before(b) => extract_filename_pattern(&b.before),

        // Mirror existing logic
        Pattern::Maybe(_) => Ok(Some(Pattern::Top)),
        Pattern::And(target) => {
            let Some(patterns) = extract_filename_patterns_from_patterns(&target.patterns)? else {
                return Ok(None);
            };
            Ok(Some(Pattern::And(Box::new(And::new(patterns)))))
        }
        Pattern::Or(target) => {
            let Some(patterns) = extract_filename_patterns_from_patterns(&target.patterns)? else {
                return Ok(None);
            };
            Ok(Some(Pattern::Or(Box::new(Or::new(patterns)))))
        }
        Pattern::Any(target) => {
            let Some(patterns) = extract_filename_patterns_from_patterns(&target.patterns)? else {
                return Ok(None);
            };
            Ok(Some(Pattern::Any(Box::new(Any::new(patterns)))))
        }
        Pattern::Some(some) => extract_filename_pattern(&some.pattern),

        Pattern::Log(_) => Ok(Some(Pattern::Top)),

        Pattern::Add(add) => {
            let Some(lhs) = extract_filename_pattern(&add.lhs)? else {
                return Ok(None);
            };
            let Some(rhs) = extract_filename_pattern(&add.rhs)? else {
                return Ok(None);
            };
            Ok(Some(Pattern::And(Box::new(And::new(vec![lhs, rhs])))))
        }
        Pattern::Subtract(sub) => {
            let Some(lhs) = extract_filename_pattern(&sub.lhs)? else {
                return Ok(None);
            };
            let Some(rhs) = extract_filename_pattern(&sub.rhs)? else {
                return Ok(None);
            };
            Ok(Some(Pattern::And(Box::new(And::new(vec![lhs, rhs])))))
        }
        Pattern::Multiply(target) => {
            let Some(lhs) = extract_filename_pattern(&target.lhs)? else {
                return Ok(None);
            };
            let Some(rhs) = extract_filename_pattern(&target.rhs)? else {
                return Ok(None);
            };
            Ok(Some(Pattern::And(Box::new(And::new(vec![lhs, rhs])))))
        }
        Pattern::Divide(target) => {
            let Some(lhs) = extract_filename_pattern(&target.lhs)? else {
                return Ok(None);
            };
            let Some(rhs) = extract_filename_pattern(&target.rhs)? else {
                return Ok(None);
            };
            Ok(Some(Pattern::And(Box::new(And::new(vec![lhs, rhs])))))
        }
        Pattern::Modulo(target) => {
            let Some(lhs) = extract_filename_pattern(&target.lhs)? else {
                return Ok(None);
            };
            let Some(rhs) = extract_filename_pattern(&target.rhs)? else {
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

impl<Q: QueryContext> FilenamePatternExtractor<Q> for Bubble<Q> {
    fn extract_filename_pattern(&self) -> Result<Option<Pattern<Q>>> {
        extract_filename_pattern(self.pattern_def.pattern())
    }
}

impl<Q: QueryContext> FilenamePatternExtractor<Q> for Contains<Q> {
    fn extract_filename_pattern(&self) -> Result<Option<Pattern<Q>>> {
        extract_filename_pattern(&self.contains)
    }
}

impl<Q: QueryContext> FilenamePatternExtractor<Q> for Includes<Q> {
    fn extract_filename_pattern(&self) -> Result<Option<Pattern<Q>>> {
        extract_filename_pattern(&self.includes)
    }
}

impl<Q: QueryContext> FilenamePatternExtractor<Q> for Where<Q> {
    fn extract_filename_pattern(&self) -> Result<Option<Pattern<Q>>> {
        let pattern = extract_filename_pattern(&self.pattern)?.unwrap_or(Pattern::Top);
        let predicate_pattern = self
            .side_condition
            .extract_filename_pattern()?
            .unwrap_or(Pattern::Top);
        Ok(Some(Pattern::And(Box::new(And::new(vec![
            pattern,
            predicate_pattern,
        ])))))
    }
}

/// Given a list of patterns, extract the filename patterns from each of them
fn extract_filename_patterns_from_patterns<Q: QueryContext>(
    predicates: &[Pattern<Q>],
) -> Result<Option<Vec<Pattern<Q>>>> {
    let mut patterns = vec![];
    for p in predicates {
        let pattern = extract_filename_pattern(p)?;
        if let Some(pattern) = pattern {
            patterns.push(pattern);
        } else {
            return Ok(None);
        }
    }
    Ok(Some(patterns))
}

/// Given a list of predicates, extract the filename patterns from each of them
fn extract_patterns_from_predicates<Q: QueryContext>(
    predicates: &[Predicate<Q>],
) -> Result<Option<Vec<Pattern<Q>>>> {
    let mut patterns = vec![];
    for p in predicates {
        let pattern = p.extract_filename_pattern()?;
        if let Some(pattern) = pattern {
            patterns.push(pattern);
        } else {
            return Ok(None);
        }
    }
    Ok(Some(patterns))
}

impl<Q: QueryContext> FilenamePatternExtractor<Q> for Predicate<Q> {
    fn extract_filename_pattern(&self) -> Result<Option<Pattern<Q>>> {
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
                        if var.is_file_name() {
                            match &m.pattern {
                                Some(pattern) => {
                                    // This is the key line of this entire file
                                    if is_safe_to_hoist(pattern)? {
                                        return Ok(Some(pattern.clone()));
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
                    Some(pattern) => extract_filename_pattern(pattern),
                    // TODO: is this right? Why do we ever have an empty pattern?
                    None => Ok(None),
                }
            }
            Predicate::Accumulate(_) | Predicate::Assignment(_) | Predicate::Return(_) => {
                Ok(Some(Pattern::Top))
            }

            Predicate::Rewrite(rw) => extract_filename_pattern(&rw.left),
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
                let Some(condition) = target.if_.extract_filename_pattern()? else {
                    return Ok(None);
                };
                let Some(then) = target.then.extract_filename_pattern()? else {
                    return Ok(None);
                };
                let Some(else_) = target.else_.extract_filename_pattern()? else {
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

// Check if a filename pattern is safe to hoist.
// This is not a great implementation, but it's a start.
// I think a better approach will actually be to introduce a Pattern::FailOpen idea where if any errors are encountered when resolving
// a pattern, we can just assume it's true. This will allow us to hoist more patterns without worrying about unbound variables.
fn is_safe_to_hoist<Q: QueryContext>(pattern: &Pattern<Q>) -> Result<bool> {
    match pattern {
        Pattern::Includes(inc) => is_safe_to_hoist(&inc.includes),
        Pattern::StringConstant(_) => Ok(true),
        // This is conservative, but it's a start
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
        | Pattern::Contains(_)
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
        | Pattern::Like(_) => Ok(false),
    }
}

mod test {
    use std::collections::BTreeMap;

    use marzano_language::target_language::TargetLanguage;

    use crate::{
        pattern_compiler::src_to_problem_libs,
        test_utils::{run_on_test_files, SyntheticFile},
    };

    #[test]
    fn test_basic_file_contains() {
        let pattern_src = r#"
        file(body=contains bubble `console.log($_)`)
        "#;
        let libs = BTreeMap::new();

        let matching_src = r#"
        console.log("Hello, world!");
        "#;

        let pattern = src_to_problem_libs(
            pattern_src.to_string(),
            &libs,
            TargetLanguage::default(),
            None,
            None,
            None,
            None,
        )
        .unwrap()
        .problem;

        // All together now
        let test_files = vec![
            SyntheticFile::new("target.js".to_owned(), matching_src.to_owned(), true),
            SyntheticFile::new(
                "do_not_read.js".to_owned(),
                "// this is not a matching file".to_owned(),
                false,
            ),
        ];
        let results = run_on_test_files(&pattern, &test_files);

        // Confirm we have 2 DoneFiles and 1 match
        assert_eq!(results.len(), 3);
        assert!(results.iter().any(|r| r.is_match()));
    }
}
