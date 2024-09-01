use crate::marzano_resolved_pattern::MarzanoResolvedPattern;

#[derive(Debug, Clone)]
pub(crate) struct LazyTraversal<'a, 'b> {
    root: &'b MarzanoResolvedPattern<'a>,
}

impl<'a, 'b> LazyTraversal<'a, 'b> {
    pub(crate) fn new(root: &'b MarzanoResolvedPattern<'a>) -> Self {
        Self { root }
    }
}
