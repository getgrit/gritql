use super::NodeCompiler;
use super::{auto_wrap::wrap_pattern_in_before_and_after_each_file, compiler::CompilationContext};
use crate::pattern::{patterns::Pattern, step::Step, variable::VariableSourceLocations};
use anyhow::Result;
use marzano_util::analysis_logs::{AnalysisLogBuilder, AnalysisLogs};
use marzano_util::position::Range;
use std::collections::BTreeMap;
use tree_sitter::Node;

const SEQUENTIAL_WARNING: &str = "Warning: sequential matches at the top of the file. If a pattern matched outside of a sequential, but no longer matches, it is likely because naked patterns are automatically wrapped with `contains bubble <pattern>`";

pub(crate) struct StepCompiler;

impl NodeCompiler for StepCompiler {
    type TargetPattern = Step;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let pattern = Pattern::from_node(
            node,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        match pattern {
            Pattern::File(_)
            | Pattern::Files(_)
            | Pattern::Contains(_)
            | Pattern::Includes(_)
            | Pattern::Maybe(_)
            | Pattern::Call(_)
            | Pattern::Where(_)
            | Pattern::Bubble(_) => {}
            Pattern::And(_)
            | Pattern::Or(_)
            | Pattern::ASTNode(_)
            | Pattern::List(_)
            | Pattern::ListIndex(_)
            | Pattern::Map(_)
            | Pattern::Accessor(_)
            | Pattern::Regex(_)
            | Pattern::Limit(_)
            | Pattern::CallBuiltIn(_)
            | Pattern::CallFunction(_)
            | Pattern::CallForeignFunction(_)
            | Pattern::Assignment(_)
            | Pattern::Accumulate(_)
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
            | Pattern::Dynamic(_)
            | Pattern::CodeSnippet(_)
            | Pattern::Variable(_)
            | Pattern::Rewrite(_)
            | Pattern::Log(_)
            | Pattern::Range(_)
            | Pattern::Within(_)
            | Pattern::After(_)
            | Pattern::Before(_)
            | Pattern::Some(_)
            | Pattern::Every(_)
            | Pattern::Add(_)
            | Pattern::Subtract(_)
            | Pattern::Multiply(_)
            | Pattern::Divide(_)
            | Pattern::Modulo(_)
            | Pattern::Dots
            | Pattern::Like(_) => {
                let range: Range = node.range().into();
                let log = AnalysisLogBuilder::default()
                    .level(441_u16)
                    .file(context.file)
                    .source(context.src)
                    .position(range.start)
                    .range(range)
                    .message(SEQUENTIAL_WARNING)
                    .build()?;
                logs.push(log);
            }
            Pattern::Sequential(ref s) => {
                for step in s.iter() {
                    if !matches!(
                        step.pattern,
                        Pattern::File(_)
                            | Pattern::Files(_)
                            | Pattern::Contains(_)
                            | Pattern::Includes(_)
                            | Pattern::Maybe(_)
                            | Pattern::Call(_)
                            | Pattern::Where(_)
                    ) {
                        let range: Range = node.range().into();
                        let log = AnalysisLogBuilder::default()
                            .level(441_u16)
                            .file(context.file)
                            .source(context.src)
                            .position(range.start)
                            .range(range)
                            .message(SEQUENTIAL_WARNING)
                            .build()?;
                        logs.push(log);
                        break;
                    }
                }
            }
        }
        let pattern =
            wrap_pattern_in_before_and_after_each_file(pattern, context.pattern_definition_info)?;

        Ok(Step::new(pattern))
    }
}
