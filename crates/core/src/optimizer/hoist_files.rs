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
    let filename_pattern = match pattern {
        Pattern::Contains(c) => c.extract_filename_pattern(),
        Pattern::Bubble(b) => b.extract_filename_pattern(),
        Pattern::Where(w) => w.extract_filename_pattern(),

        Pattern::Rewrite(rw) => extract_filename_pattern(&rw.left),

        // Once we hit a leaf node, we can't go any further
        Pattern::Variable(_) | Pattern::CodeSnippet(_) | Pattern::Range(_) => {
            Ok(Some(Pattern::Top))
        }

        Pattern::Includes(inc) => extract_filename_pattern(&inc.includes),

        // TODO: decide the rest of these
        Pattern::Within(_)
        | Pattern::After(_)
        | Pattern::Before(_)
        | Pattern::Some(_)
        | Pattern::Every(_)
        | Pattern::Add(_)
        | Pattern::Subtract(_)
        | Pattern::Multiply(_)
        | Pattern::Divide(_)
        | Pattern::Modulo(_)
        | Pattern::Dots
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
        | Pattern::Dynamic(_) => Ok(None),
    };

    filename_pattern
}

impl<Q: QueryContext> FilenamePatternExtractor<Q> for Bubble<Q> {
    fn extract_filename_pattern(&self) -> Result<Option<Pattern<Q>>> {
        extract_filename_pattern(&self.pattern_def.pattern)
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

fn extract_patterns_from_predicates<Q: QueryContext>(
    predicates: &[Predicate<Q>],
) -> Result<Vec<Pattern<Q>>> {
    let mut patterns = vec![];
    for p in predicates {
        let pattern = p.extract_filename_pattern()?;
        if let Some(pattern) = pattern {
            patterns.push(pattern);
        } else {
            return Ok(vec![]);
        }
    }
    Ok(patterns)
}

impl<Q: QueryContext> FilenamePatternExtractor<Q> for Predicate<Q> {
    fn extract_filename_pattern(&self) -> Result<Option<Pattern<Q>>> {
        match self {
            Predicate::And(and) => {
                let patterns = extract_patterns_from_predicates(&and.predicates)?;
                Ok(Some(Pattern::And(Box::new(And::new(patterns)))))
            }
            Predicate::Or(or) => {
                let patterns = extract_patterns_from_predicates(&or.predicates)?;
                Ok(Some(Pattern::Or(Box::new(Or::new(patterns)))))
            }
            Predicate::Any(a) => {
                let patterns = extract_patterns_from_predicates(&a.predicates)?;
                Ok(Some(Pattern::Any(Box::new(Any::new(patterns)))))
            }
            Predicate::Match(m) => {
                match m.val {
                    grit_pattern_matcher::pattern::Container::Variable(var) => {
                        if var.is_file_name() {
                            match &m.pattern {
                                Some(pattern) => {
                                    // This is the key line of this entire file
                                    return Ok(Some(pattern.clone()));
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

            Predicate::Maybe(_) | Predicate::True => Ok(Some(Pattern::Top)),
            Predicate::False => Ok(None),
            Predicate::Rewrite(rw) => extract_filename_pattern(&rw.left),
            Predicate::Log(_) => Ok(Some(Pattern::Top)),

            // These are more complicated, implement carefully
            Predicate::Call(_) | Predicate::Not(_) | Predicate::If(_) | Predicate::Equal(_) => {
                Ok(None)
            }
        }
    }
}
