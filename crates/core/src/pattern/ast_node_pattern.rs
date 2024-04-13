use super::{
    iter_pattern::PatternOrPredicate,
    patterns::{Matcher, PatternName},
};
use crate::context::ProblemContext;

/// Type of pattern that matches against an individual AST node.
pub trait AstNodePattern<P: ProblemContext>:
    Clone + std::fmt::Debug + Matcher<P> + PatternName + Sized
{
    fn children(&self) -> Vec<PatternOrPredicate<P>>;

    fn matches_kind_of(&self, node: &P::Node<'_>) -> bool;
}
