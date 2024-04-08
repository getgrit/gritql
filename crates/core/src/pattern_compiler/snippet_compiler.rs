use super::{
    back_tick_compiler::{BackTickCompiler, RawBackTickCompiler},
    pattern_compiler::PatternCompiler,
    NodeCompiler,
};
use crate::pattern::{
    code_snippet::CodeSnippet,
    constants::{DEFAULT_FILE_NAME, GLOBAL_VARS_SCOPE_INDEX},
    dynamic_snippet::{DynamicPattern, DynamicSnippet, DynamicSnippetPart},
    patterns::Pattern,
    variable::{register_variable, Variable},
};
use crate::{pattern_compiler::compiler::NodeCompilationContext, split_snippet::split_snippet};
use anyhow::{anyhow, bail, Result};
use grit_util::AstNode;
use marzano_language::{
    language::{nodes_from_indices, Language, SortId},
    target_language::TargetLanguage,
};
use marzano_util::{
    node_with_source::NodeWithSource,
    position::{Position, Range},
};

pub(crate) struct CodeSnippetCompiler;

impl NodeCompiler for CodeSnippetCompiler {
    type TargetPattern = Pattern;

    fn from_node_with_rhs(
        node: NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let snippet = node
            .child_by_field_name("source")
            .ok_or_else(|| anyhow!("missing content of codeSnippet"))?;
        match snippet.node.kind().as_ref() {
            "backtickSnippet" => BackTickCompiler::from_node(snippet, context),
            "rawBacktickSnippet" => RawBackTickCompiler::from_node(snippet, context),
            "languageSpecificSnippet" => {
                LanguageSpecificSnippetCompiler::from_node(snippet, context)
            }
            _ => bail!("invalid code snippet kind: {}", snippet.node.kind()),
        }
    }
}

pub(crate) struct LanguageSpecificSnippetCompiler;

impl NodeCompiler for LanguageSpecificSnippetCompiler {
    type TargetPattern = Pattern;

    fn from_node_with_rhs(
        node: NodeWithSource,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let lang_node = node
            .child_by_field_name("language")
            .ok_or_else(|| anyhow!("missing language of languageSpecificSnippet"))?;
        let lang_name = lang_node.text().trim().to_string();
        let _snippet_lang = TargetLanguage::from_string(&lang_name, None)
            .ok_or_else(|| anyhow!("invalid language: {lang_name}"))?;
        let snippet_node = node
            .child_by_field_name("snippet")
            .ok_or_else(|| anyhow!("missing snippet of languageSpecificSnippet"))?;
        let source = snippet_node.text().to_string();
        let mut range: Range = node.range().into();
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
    let mut metavariables = split_snippet(source, context.compilation.lang);
    metavariables.reverse();
    let mut parts = Vec::new();
    let mut last = 0;
    let mut last_pos = source_range.start;
    for (byte_range, var) in metavariables {
        parts.push(DynamicSnippetPart::String(
            source[last as usize..byte_range.start as usize].to_string(),
        ));
        let start_pos = Position::from_byte_index(source, Some((last_pos, last)), byte_range.start);
        // todo: does this handle utf8 correctly?
        last_pos = Position::new(start_pos.line, start_pos.column + var.len() as u32);
        let range = Range::new(
            start_pos,
            last_pos,
            source_range.start_byte + byte_range.start,
            source_range.start_byte + byte_range.start + var.len() as u32,
        );
        if let Some(var) = context.vars.get(&var.to_string()) {
            context.vars_array[context.scope_index][*var]
                .locations
                .insert(range);
            parts.push(DynamicSnippetPart::Variable(Variable::new(
                context.scope_index,
                *var,
            )));
        } else if let Some(var) = context.global_vars.get(&var.to_string()) {
            if context.compilation.file == DEFAULT_FILE_NAME {
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
            bail!(
                "Could not find variable {} in this context, for snippet {}",
                var,
                source
            );
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
) -> Result<Pattern> {
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
        let snippet_patterns: Vec<(SortId, Pattern)> = snippet_nodes
            .into_iter()
            .map(|node| {
                Ok((
                    node.node.kind_id(),
                    PatternCompiler::from_snippet_node(node, range, context, is_rhs)?,
                ))
            })
            .collect::<Result<Vec<(SortId, Pattern)>>>()?;
        let dynamic_snippet = dynamic_snippet_from_source(source, range, context)
            .map_or(None, |s| Some(DynamicPattern::Snippet(s)));
        Ok(Pattern::CodeSnippet(CodeSnippet::new(
            snippet_patterns,
            dynamic_snippet,
            source,
        )))
    }
}
