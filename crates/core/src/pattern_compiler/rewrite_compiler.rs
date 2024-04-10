use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::pattern::{
    code_snippet::CodeSnippet, dynamic_snippet::DynamicPattern, patterns::Pattern, rewrite::Rewrite,
};
use anyhow::{anyhow, Result};
use grit_util::AstNode;
use marzano_util::{analysis_logs::AnalysisLogBuilder, node_with_source::NodeWithSource};

pub(crate) struct RewriteCompiler;

impl NodeCompiler for RewriteCompiler {
    type TargetPattern = Rewrite;

    // do we want to add support for annotations?
    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let left = node
            .child_by_field_name("left")
            .ok_or_else(|| anyhow!("missing lhs of rewrite"))?;
        let right = node
            .child_by_field_name("right")
            .ok_or_else(|| anyhow!("missing rhs of rewrite"))?;
        let annotation = node.child_by_field_name("annotation");
        let left = PatternCompiler::from_node(&left, context)?;
        let right = PatternCompiler::from_node_with_rhs(&right, context, true)?;

        match (&left, &right) {
            (
                Pattern::CodeSnippet(CodeSnippet {
                    source: left_source,
                    ..
                }),
                Pattern::CodeSnippet(CodeSnippet {
                    source: right_source,
                    ..
                }),
            ) if left_source == right_source => {
                let log = AnalysisLogBuilder::default()
                .level(441_u16)
                .file(context.compilation.file)
                .source(node.source)
                .position(node.node.start_position())
                .range(node.range())
                .message(
                    format!("Warning: This is rewriting `{}` into the identical string `{}`, will have no effect.", left_source, right_source)
                )
                .build()?;
                context.logs.push(log);
            }
            (_, _) => {}
        }
        let right = match right {
            Pattern::Dynamic(r) => r,
            Pattern::CodeSnippet(CodeSnippet {
                dynamic_snippet: Some(r),
                ..
            }) => r,
            Pattern::Variable(v) => DynamicPattern::Variable(v),
            Pattern::Accessor(a) => DynamicPattern::Accessor(a),
            Pattern::ListIndex(a) => DynamicPattern::ListIndex(a),
            Pattern::CallBuiltIn(c) => DynamicPattern::CallBuiltIn(*c),
            Pattern::CallFunction(c) => DynamicPattern::CallFunction(*c),
            Pattern::CallForeignFunction(c) => DynamicPattern::CallForeignFunction(*c),
            Pattern::ASTNode(_)
                | Pattern::List(_)
                | Pattern::Map(_)
                | Pattern::Call(_)
                | Pattern::Regex(_)
                | Pattern::File(_)
                | Pattern::Files(_)
                | Pattern::Bubble(_)
                | Pattern::Limit(_)
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
                | Pattern::Like(_)
                | Pattern::Dots
                | Pattern::Sequential(_) => Err(anyhow!(
                "right hand side of rewrite must be a code snippet or function call, but found: {:?}",
                right
            ))?,
        };

        let annotation = annotation.map(|n| n.text().trim().to_string());
        Ok(Rewrite::new(left, right, annotation))
    }
}
