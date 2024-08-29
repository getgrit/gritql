use super::auto_wrap::wrap_pattern_in_before_and_after_each_file;
use super::compiler::NodeCompilationContext;
use super::pattern_compiler::PatternCompiler;
use super::NodeCompiler;
use crate::problem::MarzanoQueryContext;
use anyhow::Result;
use grit_pattern_matcher::pattern::{Pattern, Step};
use grit_util::AnalysisLogBuilder;
use marzano_util::node_with_source::NodeWithSource;

const SEQUENTIAL_WARNING: &str = "Warning: sequential matches at the top of the file. If a pattern matched outside of a sequential, but no longer matches, it is likely because naked patterns are automatically wrapped with `contains bubble <pattern>`";

pub(crate) struct StepCompiler;

impl NodeCompiler for StepCompiler {
    type TargetPattern = Step<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let pattern = PatternCompiler::from_node(node, context)?;
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
            | Pattern::AstNode(_)
            | Pattern::List(_)
            | Pattern::ListIndex(_)
            | Pattern::Map(_)
            | Pattern::Accessor(_)
            | Pattern::Regex(_)
            | Pattern::Limit(_)
            | Pattern::CallBuiltIn(_)
            | Pattern::CallFunction(_)
            | Pattern::CallbackPattern(_)
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
            | Pattern::Like(_)
            | Pattern::CallbackPattern(_) => {
                let range = node.range();
                let log = AnalysisLogBuilder::default()
                    .level(441_u16)
                    .file(context.compilation.file)
                    .source(node.source)
                    .position(range.start)
                    .range(range)
                    .message(SEQUENTIAL_WARNING)
                    .build()?;
                context.logs.push(log);
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
                        let range = node.range();
                        let log = AnalysisLogBuilder::default()
                            .level(441_u16)
                            .file(context.compilation.file)
                            .source(node.source)
                            .position(range.start)
                            .range(range)
                            .message(SEQUENTIAL_WARNING)
                            .build()?;
                        context.logs.push(log);
                        break;
                    }
                }
            }
        }
        let pattern = wrap_pattern_in_before_and_after_each_file(
            pattern,
            context.compilation.pattern_definition_info,
        )?;

        Ok(Step::new(pattern))
    }
}
