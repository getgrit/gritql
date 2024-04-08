use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{
    code_snippet::CodeSnippet, dynamic_snippet::DynamicPattern, patterns::Pattern,
    rewrite::Rewrite, variable::VariableSourceLocations,
};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::{AnalysisLogBuilder, AnalysisLogs};
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct RewriteCompiler;

impl NodeCompiler for RewriteCompiler {
    type TargetPattern = Rewrite;

    // do we want to add support for annotations?
    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let left = node
            .child_by_field_name("left")
            .ok_or_else(|| anyhow!("missing lhs of rewrite"))?;
        let right = node
            .child_by_field_name("right")
            .ok_or_else(|| anyhow!("missing rhs of rewrite"))?;
        let annotation = node.child_by_field_name("annotation");
        let left = Pattern::from_node(
            &left,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        let right = Pattern::from_node(
            &right,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            true,
            logs,
        )?;

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
                .file(context.file)
                .source(context.src)
                .position(node.start_position())
                .range(node.range())
                .message(
                    format!("Warning: This is rewriting `{}` into the identical string `{}`, will have no effect.", left_source, right_source)
                )
                .build()?;
                logs.push(log);
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

        let annotation = annotation.map(|n| {
            n.utf8_text(context.src.as_bytes())
                .unwrap()
                .trim()
                .to_string()
        });
        Ok(Rewrite::new(left, right, annotation))
    }
}
