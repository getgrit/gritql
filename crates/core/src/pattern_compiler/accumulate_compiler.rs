use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::marzano_code_snippet::MarzanoCodeSnippet;
use crate::problem::MarzanoQueryContext;
use anyhow::{anyhow, Result};
use grit_pattern_matcher::pattern::{Accumulate, DynamicPattern, Pattern};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct AccumulateCompiler;

impl NodeCompiler for AccumulateCompiler {
    type TargetPattern = Accumulate<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let left_node = node
            .child_by_field_name("left")
            .ok_or_else(|| anyhow!("missing variable of patternAccumulateString"))?;
        let left = PatternCompiler::from_node(&left_node, context)?;
        let right_node = node
            .child_by_field_name("right")
            .ok_or_else(|| anyhow!("missing pattern of patternAccumulateString"))?;
        let right = PatternCompiler::from_node_with_rhs(&right_node, context, true)?;
        let dynamic_right = match right.clone() {
            Pattern::Dynamic(r) => Some(r),
            Pattern::CodeSnippet(MarzanoCodeSnippet {
                dynamic_snippet: Some(r),
                ..
            }) => Some(r),
            Pattern::Variable(v) => Some(DynamicPattern::Variable(v)),
            Pattern::AstNode(_)
            | Pattern::List(_)
            | Pattern::ListIndex(_)
            | Pattern::Map(_)
            | Pattern::Accessor(_)
            | Pattern::Call(_)
            | Pattern::Regex(_)
            | Pattern::File(_)
            | Pattern::Files(_)
            | Pattern::Bubble(_)
            | Pattern::Limit(_)
            | Pattern::Callback(_)
            | Pattern::CallBuiltIn(_)
            | Pattern::CallFunction(_)
            | Pattern::CallForeignFunction(_)
            | Pattern::Assignment(_)
            | Pattern::Accumulate(_)
            | Pattern::And(_)
            | Pattern::Or(_)
            | Pattern::Maybe(_)
            | Pattern::Any(_)
            | Pattern::Not(_)
            | Pattern::If(_)
            | Pattern::Undefined
            | Pattern::Top
            | Pattern::Bottom
            | Pattern::Underscore
            | Pattern::StringConstant(_)
            | Pattern::AstLeafNode(_)
            | Pattern::IntConstant(_)
            | Pattern::FloatConstant(_)
            | Pattern::BooleanConstant(_)
            | Pattern::CodeSnippet(_)
            | Pattern::Rewrite(_)
            | Pattern::Log(_)
            | Pattern::Range(_)
            | Pattern::Contains(_)
            | Pattern::Includes(_)
            | Pattern::Within(_)
            | Pattern::After(_)
            | Pattern::Before(_)
            | Pattern::Where(_)
            | Pattern::Some(_)
            | Pattern::Every(_)
            | Pattern::Add(_)
            | Pattern::Subtract(_)
            | Pattern::Multiply(_)
            | Pattern::Divide(_)
            | Pattern::Modulo(_)
            | Pattern::Dots
            | Pattern::Like(_)
            | Pattern::Sequential(_) => None,
        };
        Ok(Accumulate::new(left, right, dynamic_right))
    }
}
