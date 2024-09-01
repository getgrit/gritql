use grit_pattern_matcher::{
    context::QueryContext,
    pattern::{Matcher, State},
};
use grit_util::AnalysisLogs;
use std::fmt;

use crate::{
    marzano_context::MarzanoContext, marzano_resolved_pattern::MarzanoResolvedPattern,
    problem::MarzanoQueryContext,
};

pub(crate) struct LazyTraversal<'a, 'b> {
    binding: &'b MarzanoResolvedPattern<'a>,
    context: &'a MarzanoContext<'a>,
    state: &'a mut State<'a, MarzanoQueryContext>,
    logs: &'a mut AnalysisLogs,
}

impl<'a, 'b> LazyTraversal<'a, 'b> {
    pub(crate) fn new(
        root: &'b MarzanoResolvedPattern<'a>,
        context: &'a MarzanoContext<'a>,
        state: &'a mut State<'a, MarzanoQueryContext>,
        logs: &'a mut AnalysisLogs,
    ) -> Self {
        Self {
            binding: root,
            state,
            context,
            logs,
        }
    }

    // pub(crate) fn matches(&self, pattern: &MarzanoResolvedPattern<'a>) -> bool {
    //     self.root.matches(pattern)
    // }
}

impl<'a, 'b> fmt::Debug for LazyTraversal<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LazyTraversal")
            .field("binding", &self.binding)
            .field("state", &self.state)
            .finish()
    }
}

// impl<'a, 'b, Q: QueryContext> Matcher<Q> for LazyTraversal<'a, 'b> {
//     fn execute<'a>(
//         &'a self,
//         binding: &Q::ResolvedPattern<'a>,
//         state: &mut State<'a, Q>,
//         context: &'a Q::ExecContext<'a>,
//         _logs: &mut AnalysisLogs,
//     ) -> GritResult<bool> {
//         let text = binding.text(&state.files, context.language())?;
//         if text == self.text {
//             Ok(true)
//         } else {
//             Ok(false)
//         }
//     }
// }
