use grit_pattern_matcher::pattern::Matcher;
use grit_pattern_matcher::pattern::Pattern;
use grit_pattern_matcher::pattern::State;
use grit_util::error::GritResult;
use grit_util::AnalysisLogs;

use crate::marzano_context::MarzanoContext;
use crate::{marzano_resolved_pattern::MarzanoResolvedPattern, problem::MarzanoQueryContext};

#[derive(Debug, Clone)]
pub(crate) struct LazyTraversal<'a, 'b> {
    root: &'b MarzanoResolvedPattern<'a>,
    // patterns: Vec<Pattern<MarzanoQueryContext>>,
}

impl<'a, 'b> LazyTraversal<'a, 'b> {
    pub(crate) fn new(root: &'b MarzanoResolvedPattern<'a>) -> Self {
        Self {
            root,
            // patterns: vec![],
        }
    }

    pub(crate) fn matches(
        &mut self,
        pattern: Pattern<MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
        state: &mut State<'a, MarzanoQueryContext>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        // THIS IS UNSAFE
        // TODO: make this safe / improve the lifetimes so pattern does not need to be static
        let borrowed_pattern: &'static Pattern<MarzanoQueryContext> =
            unsafe { std::mem::transmute(&pattern) };

        let matches = borrowed_pattern.execute(self.root, state, context, logs)?;
        Ok(matches)
    }
}
