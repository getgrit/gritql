use super::{
    back_tick_compiler::{BackTickCompiler, RawBackTickCompiler},
    pattern_compiler::PatternCompiler,
    NodeCompiler,
};
use crate::{
    marzano_code_snippet::MarzanoCodeSnippet, problem::MarzanoQueryContext,
    variables::register_variable,
};
use crate::{pattern_compiler::compiler::NodeCompilationContext, split_snippet::split_snippet};
use anyhow::{anyhow, bail, Result};
use grit_pattern_matcher::{
    constants::GLOBAL_VARS_SCOPE_INDEX,
    pattern::{DynamicPattern, DynamicSnippet, DynamicSnippetPart, Pattern, Variable},
};
use grit_util::{AstNode, Language, Position, Range};
use marzano_language::{
    language::{nodes_from_indices, MarzanoLanguage, SortId},
    target_language::TargetLanguage,
};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct CodeSnippetCompiler;

impl NodeCompiler for CodeSnippetCompiler {
    type TargetPattern = Pattern<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let snippet = node
            .child_by_field_name("source")
            .ok_or_else(|| anyhow!("missing content of codeSnippet"))?;
        match snippet.node.kind().as_ref() {
            "backtickSnippet" => BackTickCompiler::from_node_with_rhs(&snippet, context, is_rhs),
            "rawBacktickSnippet" => {
                RawBackTickCompiler::from_node_with_rhs(&snippet, context, is_rhs)
            }
            "languageSpecificSnippet" => {
                LanguageSpecificSnippetCompiler::from_node_with_rhs(&snippet, context, is_rhs)
            }
            _ => bail!("invalid code snippet kind: {}", snippet.node.kind()),
        }
    }
}

pub(crate) struct LanguageSpecificSnippetCompiler;

impl NodeCompiler for LanguageSpecificSnippetCompiler {
    type TargetPattern = Pattern<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let lang_node = node
            .child_by_field_name("language")
            .ok_or_else(|| anyhow!("missing language of languageSpecificSnippet"))?;
        let lang_name = lang_node.text()?.trim().to_string();
        let _snippet_lang = TargetLanguage::from_string(&lang_name, None)
            .ok_or_else(|| anyhow!("invalid language: {lang_name}"))?;
        let snippet_node = node
            .child_by_field_name("snippet")
            .ok_or_else(|| anyhow!("missing snippet of languageSpecificSnippet"))?;
        let source = snippet_node.text()?.to_string();
        let mut range = node.range();
        range.adjust_columns(1, -1);
        let content = source
            .strip_prefix('"')
            .ok_or_else(|| anyhow!("Unable to extract content from raw snippet: {source}"))?
            .strip_suffix('"')
            .ok_or_else(|| anyhow!("Unable to extract content from raw snippet: {source}"))?;

        parse_snippet_content(content, range, context, is_rhs)
    }
}

pub(crate) fn dynamic_snippet_from_source(
    raw_source: &str,
    source_range: Range,
    context: &mut NodeCompilationContext,
) -> Result<DynamicSnippet> {
    let source_string = raw_source
        .replace("\\n", "\n")
        .replace("\\$", "$")
        .replace("\\^", "^")
        .replace("\\`", "`")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\");
    let source = source_string.as_str();
    let metavariables = split_snippet(source, context.compilation.lang);
    let mut parts = Vec::with_capacity(2 * metavariables.len() + 1);
    let mut last = 0;
    let mut last_pos = source_range.start;
    // Reverse the iterator so we go over the variables in ascending order.
    for (byte_range, var) in metavariables.into_iter().rev() {
        parts.push(DynamicSnippetPart::String(
            source[last as usize..byte_range.start as usize].to_string(),
        ));
        let start_pos = position_from_previous(last_pos, source, last, byte_range.start);
        // todo: does this handle utf8 correctly?
        last_pos = Position::new(start_pos.line, start_pos.column + var.len() as u32);
        let range = Range::new(
            start_pos,
            last_pos,
            source_range.start_byte + byte_range.start,
            source_range.start_byte + byte_range.start + var.len() as u32,
        );
        if let Some(var) = context.vars.get(var.as_ref()) {
            context.vars_array[context.scope_index][*var]
                .locations
                .insert(range);
            parts.push(DynamicSnippetPart::Variable(Variable::new(
                context.scope_index,
                *var,
            )));
        } else if let Some(var) = context.global_vars.get(var.as_ref()) {
            if context.compilation.file.is_none() {
                context.vars_array[GLOBAL_VARS_SCOPE_INDEX][*var]
                    .locations
                    .insert(range);
            }
            parts.push(DynamicSnippetPart::Variable(Variable::new(
                GLOBAL_VARS_SCOPE_INDEX,
                *var,
            )));
        } else if var.starts_with("$GLOBAL_") {
            let variable = register_variable(&var, range, context)?;
            parts.push(DynamicSnippetPart::Variable(variable));
        } else {
            bail!("Could not find variable {var} in this context, for snippet {source}");
        }
        last = byte_range.end;
    }
    parts.push(DynamicSnippetPart::String(
        source[last as usize..].to_string(),
    ));
    Ok(DynamicSnippet { parts })
}

pub(crate) fn parse_snippet_content(
    source: &str,
    range: Range,
    context: &mut NodeCompilationContext,
    is_rhs: bool,
) -> Result<Pattern<MarzanoQueryContext>> {
    // we check for CURLY_VAR_REGEX in the content, and if found
    // compile into a DynamicPattern, rather than a CodeSnippet.
    // This is because the syntax should only ever be necessary
    // when treating a metavariable as a string to substitute
    // rather than an AST node to match on. eg. in the following
    // `const ${name}Handler = useCallback(async () => $body, []);`
    // $name does not correspond to a node, but rather prepends a
    // string to "Handler", which will together combine into an
    // identifier.
    if context
        .compilation
        .lang
        .metavariable_bracket_regex()
        .is_match(source)
    {
        if is_rhs {
            Ok(Pattern::Dynamic(
                dynamic_snippet_from_source(source, range, context).map(DynamicPattern::Snippet)?,
            ))
        } else {
            bail!("bracketed metavariables are only allowed on the rhs of a snippet");
        }
    } else {
        if context
            .compilation
            .lang
            .exact_variable_regex()
            .is_match(source.trim())
        {
            match source.trim() {
                "$_" => return Ok(Pattern::Underscore),
                "^_" => return Ok(Pattern::Underscore),
                name => {
                    let var = register_variable(name, range, context)?;
                    return Ok(Pattern::Variable(var));
                }
            }
        }
        let snippet_trees = context.compilation.lang.parse_snippet_contexts(source);
        let snippet_nodes = nodes_from_indices(&snippet_trees);
        if snippet_nodes.is_empty() {
            // not checking if is_rhs. So could potentially
            // be harder to find bugs where we expect the pattern
            // to parse. unfortunately got rid of check to support
            // passing non-node snippets as args.
            return Ok(Pattern::Dynamic(
                dynamic_snippet_from_source(source, range, context).map(DynamicPattern::Snippet)?,
            ));
        }
        let snippet_patterns: Vec<(SortId, Pattern<MarzanoQueryContext>)> = snippet_nodes
            .into_iter()
            .map(|node| {
                Ok((
                    node.node.kind_id(),
                    PatternCompiler::from_snippet_node(node, range, context, is_rhs)?,
                ))
            })
            .collect::<Result<Vec<(SortId, Pattern<MarzanoQueryContext>)>>>()?;
        let dynamic_snippet = dynamic_snippet_from_source(source, range, context)
            .map_or(None, |s| Some(DynamicPattern::Snippet(s)));
        Ok(Pattern::CodeSnippet(MarzanoCodeSnippet::new(
            snippet_patterns,
            dynamic_snippet,
            source,
        )))
    }
}

fn position_from_previous(
    prev_position: Position,
    source: &str,
    start_index: u32,
    end_index: u32,
) -> Position {
    let mut pos = Position::from_byte_index(
        &source[start_index as usize..],
        (end_index - start_index) as usize,
    );
    pos.add(prev_position);
    pos
}
