use anyhow::Result;
use grit_pattern_matcher::{
    context::QueryContext,
    pattern::{And, Bubble, Contains, Pattern, Predicate, Where},
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

        // TODO: is this right?
        Pattern::Variable(_) => Ok(Some(Pattern::Top)),
        Pattern::CodeSnippet(_) => Ok(Some(Pattern::Top)),

        Pattern::Rewrite(_)
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

impl<Q: QueryContext> FilenamePatternExtractor<Q> for Predicate<Q> {
    fn extract_filename_pattern(&self) -> Result<Option<Pattern<Q>>> {
        match self {
            Predicate::And(and) => {
                let mut patterns = vec![];
                for p in &and.predicates {
                    let pattern = p.extract_filename_pattern()?;
                    if let Some(pattern) = pattern {
                        patterns.push(pattern);
                    } else {
                        return Ok(None);
                    }
                }
                Ok(Some(Pattern::And(Box::new(And::new(patterns)))))
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
                    // TODO: is this right? It does not seem right
                    None => Ok(None),
                }
            }
            Predicate::Accumulate(_) | Predicate::Assignment(_) | Predicate::Return(_) => {
                Ok(Some(Pattern::Top))
            }
            Predicate::Maybe(_)
            | Predicate::Any(_)
            | Predicate::Rewrite(_)
            | Predicate::Log(_)
            | Predicate::Call(_)
            | Predicate::Not(_)
            | Predicate::If(_)
            | Predicate::True
            | Predicate::False
            | Predicate::Or(_)
            | Predicate::Equal(_) => Ok(None),
        }
    }
}
