use super::{
    iter_pattern::PatternOrPredicate,
    patterns::{Matcher, PatternName},
};
use crate::context::QueryContext;

/// Type of pattern that matches against an individual (non-leaf) AST node.
pub trait AstNodePattern<Q: QueryContext>:
    Clone + std::fmt::Debug + Matcher<Q> + PatternName + Sized
{
    fn children(&self) -> Vec<PatternOrPredicate<Q>>;

    fn matches_kind_of(&self, node: &Q::Node<'_>) -> bool;
}

/// Type of pattern that matches against an individual AST leaf node.
pub trait AstLeafNodePattern<Q: QueryContext>:
    Clone + std::fmt::Debug + Matcher<Q> + PatternName + Sized
{
    /// Provides a *possible* text value for the leaf node.
    fn text(&self) -> Option<&str>;
}
