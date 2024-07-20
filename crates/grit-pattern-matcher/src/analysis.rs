use crate::{
    context::QueryContext,
    pattern::{Pattern, PatternOrPredicate},
};

/// Determine if a provided pattern has a rewrite anywhere inside of it
pub fn has_rewrite<Q: QueryContext>(current_pattern: &Pattern<Q>) -> bool {
    for pattern in current_pattern.iter() {
        if matches!(pattern, PatternOrPredicate::Pattern(Pattern::Rewrite(_))) {
            return true;
        }
    }
    false
}
