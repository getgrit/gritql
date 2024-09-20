use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler, variable_compiler::VariableCompiler,
};
use crate::{problem::MarzanoQueryContext, split_snippet::split_snippet};
use anyhow::{anyhow, Result};
use grit_pattern_matcher::pattern::{Container, Match, Pattern, Predicate, Where};
use grit_util::{traverse, AnalysisLogBuilder, AstNode, Language, Order};
use marzano_util::{cursor_wrapper::CursorWrapper, node_with_source::NodeWithSource};

pub(crate) struct AsCompiler;

impl NodeCompiler for AsCompiler {
    // todo make `as` its own pattern
    type TargetPattern = Where<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let pattern = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern of patternWhere"))?;

        let variable = node
            .child_by_field_name("variable")
            .ok_or_else(|| anyhow!("missing variable of patternWhere"))?;

        let name = variable.text()?;
        let name = name.trim();

        // this just searches the subtree for a variables that share the name.
        // could possible lead to some false positives, but more precise solutions
        // require much greater changes.
        if pattern_repeated_variable(&pattern, name, context.compilation.lang)? {
            let range = node.range();
            let log = AnalysisLogBuilder::default()
                .level(441_u16)
                .file(context.compilation.file)
                .source(node.source)
                .position(range.start)
                .range(range)
                .message(format!(
                    "Warning: it is usually incorrect to redefine a variable {name} using as"
                ))
                .build()?;
            context.logs.push(log);
        }

        let pattern = PatternCompiler::from_node(&pattern, context)?;
        let variable = VariableCompiler::from_node(&variable, context)?;
        Ok(Where::new(
            Pattern::Variable(variable.clone()),
            Predicate::Match(Box::new(Match::new(
                Container::Variable(variable.clone()),
                Some(pattern),
            ))),
        ))
    }
}

fn pattern_repeated_variable(
    pattern: &NodeWithSource,
    name: &str,
    lang: &impl Language,
) -> Result<bool> {
    let cursor = pattern.node.walk();
    let cursor = traverse(CursorWrapper::new(cursor, pattern.source), Order::Pre);
    Ok(cursor
        .filter(|n| n.node.kind() == "variable" || n.node.kind() == "codeSnippet")
        .map(|n| {
            let s = n.text()?.trim().to_string();
            if n.node.kind() == "variable" {
                Ok(s == name)
            } else {
                Ok(is_variables_in_snippet(name, &s, lang))
            }
        })
        .collect::<Result<Vec<bool>>>()?
        .into_iter()
        .any(|b| b))
}

fn is_variables_in_snippet(name: &str, snippet: &str, lang: &impl Language) -> bool {
    let variables = split_snippet(snippet, lang);
    variables.iter().any(|v| v.1 == name)
}
