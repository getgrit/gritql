use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::problem::MarzanoQueryContext;
use anyhow::{anyhow, Result};
use grit_pattern_matcher::pattern::Includes;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct IncludesCompiler;

impl NodeCompiler for IncludesCompiler {
    type TargetPattern = Includes<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let includes = node
            .child_by_field_name("includes")
            .ok_or_else(|| anyhow!("missing includes of patternIncludes"))?;
        let includes = PatternCompiler::from_node(&includes, context)?;
        Ok(Includes::new(includes))
    }
}
