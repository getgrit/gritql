use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{
    accumulate::Accumulate, code_snippet::CodeSnippet, dynamic_snippet::DynamicPattern,
    patterns::Pattern, variable::VariableSourceLocations,
};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct AccumulateCompiler;

impl NodeCompiler for AccumulateCompiler {
    type TargetPattern = Accumulate;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let left_node = node
            .child_by_field_name("left")
            .ok_or_else(|| anyhow!("missing variable of patternAccumulateString"))?;
        let left = Pattern::from_node(
            &left_node,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        let right_node = node
            .child_by_field_name("right")
            .ok_or_else(|| anyhow!("missing pattern of patternAccumulateString"))?;
        let right = Pattern::from_node(
            &right_node,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            true,
            logs,
        )?;
        let dynamic_right = match right.clone() {
            Pattern::Dynamic(r) => Some(r),
            Pattern::CodeSnippet(CodeSnippet {
                dynamic_snippet: Some(r),
                ..
            }) => Some(r),
            Pattern::Variable(v) => Some(DynamicPattern::Variable(v)),
            Pattern::ASTNode(_)
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
