use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler, step_compiler::StepCompiler,
};
use crate::{
    pattern::{files::Files, patterns::Pattern, sequential::Sequential, some::Some, step::Step},
    problem::MarzanoQueryContext,
};
use anyhow::Result;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct SequentialCompiler;

impl SequentialCompiler {
    pub(crate) fn from_files_node(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
    ) -> Result<Sequential<MarzanoQueryContext>> {
        node.named_children_by_field_name("files")
            .map(|n| {
                let step = StepCompiler::from_node(&n, context)?;
                let some = Pattern::Some(Box::new(Some::new(step.pattern)));
                let files = Pattern::Files(Box::new(Files::new(some)));
                Ok(Step { pattern: files })
            })
            .collect::<Result<Vec<_>>>()
            .map(Into::into)
    }
}

impl NodeCompiler for SequentialCompiler {
    type TargetPattern = Sequential<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        node.named_children_by_field_name("sequential")
            .map(|n| StepCompiler::from_node(&n, context))
            .collect::<Result<Vec<_>>>()
            .map(Into::into)
    }
}
