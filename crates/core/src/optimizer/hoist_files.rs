use anyhow::Result;
use grit_pattern_matcher::{context::QueryContext, pattern::Pattern};

/// Given a pattern, construct a new pattern that reflects any filename predicates found
/// If analysis cannot be done reliably, returns None
pub fn extract_filename_pattern<Q: QueryContext>(
    pattern: &Pattern<Q>,
) -> Result<Option<Pattern<Q>>> {
    print!("extract_filename_pattern: {:?}", pattern);
    Ok(None)
}
