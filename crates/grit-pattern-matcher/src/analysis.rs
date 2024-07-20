use crate::{
    context::QueryContext,
    pattern::{Pattern, PatternDefinition, PatternOrPredicate, Predicate},
};

/// Determine if a provided pattern has a rewrite anywhere inside of it
///
/// Note this does not yet walk inside predicates and function calls
pub fn has_rewrite<Q: QueryContext>(
    current_pattern: &Pattern<Q>,
    definitions: &[PatternDefinition<Q>],
) -> bool {
    for pattern in current_pattern.iter(definitions) {
        if matches!(pattern, PatternOrPredicate::Pattern(Pattern::Rewrite(_))) {
            return true;
        }
        if matches!(
            pattern,
            PatternOrPredicate::Predicate(Predicate::Rewrite(_))
        ) {
            return true;
        }
        if matches!(
            pattern,
            PatternOrPredicate::Predicate(Predicate::Accumulate(_))
        ) {
            return true;
        }
        // match pattern {
        //     PatternOrPredicate::Pattern(p) => {
        //         println!("Check {}", p.name());
        //     }
        //     PatternOrPredicate::Predicate(p) => {
        //         println!("Check {}", p.name());
        //     }
        // }
    }
    false
}
