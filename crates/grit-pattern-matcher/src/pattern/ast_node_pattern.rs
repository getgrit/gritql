use super::{
    iter_pattern::PatternOrPredicate,
    patterns::{Matcher, PatternName},
};
use crate::context::{QueryContext, StaticDefinitions};

/// Type of pattern that matches against an individual (non-leaf) AST node.
pub trait AstNodePattern<Q: QueryContext>:
    Clone + std::fmt::Debug + Matcher<Q> + PatternName + Sized
{
    /// Does this AST include trivia?
    /// Trivia is useful for being able to re-print an AST, but not all parsers support collecting it.
    const INCLUDES_TRIVIA: bool;

    fn children<'a>(
        &'a self,
        definitions: &'a StaticDefinitions<Q>,
    ) -> Vec<PatternOrPredicate<'a, Q>>;

    fn matches_kind_of(&self, node: &Q::Node<'_>) -> bool;
}

/// Type of pattern that matches against an individual AST leaf node.
pub trait AstLeafNodePattern<Q: QueryContext>:
    Clone + std::fmt::Debug + Matcher<Q> + PatternName + Sized
{
    /// Provides a *possible* text value for the leaf node.
    /// This is not mandatory, but enables some advanced functionality.
    fn text(&self) -> Option<&str> {
        None
    }
}
