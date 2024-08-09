use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    snippet_compiler::parse_snippet_content,
};
use crate::problem::MarzanoQueryContext;
use anyhow::{anyhow, bail, Result};
use grit_pattern_matcher::pattern::Pattern;
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct BackTickCompiler;

impl NodeCompiler for BackTickCompiler {
    type TargetPattern = Pattern<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let source = node.text()?.to_string();
        let mut range = node.range();
        range.adjust_columns(1, -1);
        let content = source
            .strip_prefix('`')
            .ok_or_else(|| GritPatternError::new("Unable to extract content from snippet: {source}"))?
            .strip_suffix('`')
            .ok_or_else(|| GritPatternError::new("Unable to extract content from snippet: {source}"))?;
        parse_snippet_content(content, range.into(), context, is_rhs)
    }
}

pub(crate) struct RawBackTickCompiler;

impl NodeCompiler for RawBackTickCompiler {
    type TargetPattern = Pattern<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        if !is_rhs {
            return Err(GritPatternError::new("raw snippets are only allowed on the right hand side of a rule"));
        }
        let source = node.text()?.to_string();
        let mut range = node.range();
        // adjust range by "raw`" and "`"
        range.adjust_columns(4, -1);
        let content = source
            .strip_prefix("raw`")
            .ok_or_else(|| GritPatternError::new(format!("Unable to extract content from raw snippet: {}", source)))?
            .strip_suffix('`')
            .ok_or_else(|| GritPatternError::new(format!("Unable to extract content from raw snippet: {}", source)))?;
        parse_snippet_content(content, range.into(), context, is_rhs)
    }
}
