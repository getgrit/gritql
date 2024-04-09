use super::{
    ast_node_compiler::AstNodeCompiler, compiler::NodeCompilationContext,
    node_compiler::NodeCompiler, pattern_compiler::PatternCompiler,
};
use crate::pattern::{
    built_in_functions::CallBuiltIn,
    call::{Call, PrCall},
    file_pattern::FilePattern,
    functions::{CallForeignFunction, CallFunction},
    patterns::Pattern,
};
use anyhow::{anyhow, bail, Result};
use grit_util::AstNode;
use itertools::Itertools;
use marzano_language::language::Language;
use marzano_util::{node_with_source::NodeWithSource, position::Range};
use std::collections::BTreeMap;

pub(crate) struct CallCompiler;

impl NodeCompiler for CallCompiler {
    type TargetPattern = Pattern;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let sort = node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of nodeLike"))?;
        let kind = sort.text().trim();
        let sort = context
            .compilation
            .lang
            .get_ts_language()
            .id_for_node_kind(kind, true);
        let expected_params = if let Some(built_in) = context
            .compilation
            .built_ins
            .get_built_ins()
            .iter()
            .find(|b| b.name == kind)
        {
            Some(built_in.params.iter().map(|s| s.to_string()).collect_vec())
        } else if let Some(info) = context.compilation.function_definition_info.get(kind) {
            Some(collect_params(&info.parameters))
        } else if let Some(info) = context
            .compilation
            .foreign_function_definition_info
            .get(kind)
        {
            Some(collect_params(&info.parameters))
        } else {
            context
                .compilation
                .pattern_definition_info
                .get(kind)
                .map(|info| collect_params(&info.parameters))
        };
        let named_args_count = node.named_children_by_field_name("named_args").count();
        let named_args = node.named_children_by_field_name("named_args");
        let named_args =
            node_to_args_pairs(named_args, context.compilation.lang, kind, &expected_params)?;

        // tree-sitter returns 0 for sorts/kinds it doesn't know about
        if sort != 0 {
            return Ok(Pattern::ASTNode(Box::new(AstNodeCompiler::from_args(
                named_args, sort, context,
            )?)));
        }
        let mut args = named_args_to_hash_map(named_args, context)?;
        if args.len() != named_args_count {
            return Err(anyhow!("duplicate named args for invocation of {kind}"));
        }
        if kind == "file" {
            for arg in &args {
                if arg.0 != "$name" && arg.0 != "$body" {
                    bail!("file pattern can only have $name and $body as named args");
                }
            }
            let name = args
                .remove_entry("$name")
                .map(|p| p.1)
                .unwrap_or(Pattern::Underscore);
            let body = args
                .remove_entry("$body")
                .map(|p| p.1)
                .unwrap_or(Pattern::Top);
            Ok(Pattern::File(Box::new(FilePattern::new(name, body))))
        } else if let Some(index) = context
            .compilation
            .built_ins
            .get_built_ins()
            .iter()
            .position(|b| b.name == kind)
        {
            if !is_rhs {
                return Err(anyhow!(format!(
                    "built-in {} can only be used on the right hand side of a rewrite",
                    kind
                )));
            }
            Ok(Pattern::CallBuiltIn(Box::new(CallBuiltIn::from_args(
                args,
                context.compilation.built_ins,
                index,
                context.compilation.lang,
            )?)))
        } else if let Some(info) = context.compilation.function_definition_info.get(kind) {
            let args = match_args_to_params(
                kind,
                args,
                &collect_params(&info.parameters),
                context.compilation.lang,
            )?;
            Ok(Pattern::CallFunction(Box::new(CallFunction::new(
                info.index, args,
            ))))
        } else if let Some(info) = context
            .compilation
            .foreign_function_definition_info
            .get(kind)
        {
            let args = match_args_to_params(
                kind,
                args,
                &collect_params(&info.parameters),
                context.compilation.lang,
            )?;
            Ok(Pattern::CallForeignFunction(Box::new(
                CallForeignFunction::new(info.index, args),
            )))
        } else {
            let info = context
                .compilation
                .pattern_definition_info
                .get(kind)
                .ok_or_else(|| {
                    anyhow!("pattern definition not found: {kind}. Try running grit init.")
                })?;
            let args = match_args_to_params(
                kind,
                args,
                &collect_params(&info.parameters),
                context.compilation.lang,
            )?;
            Ok(Pattern::Call(Box::new(Call::new(
                (info.index).to_owned(),
                args,
            ))))
        }
    }
}

pub(crate) struct PrCallCompiler;

impl NodeCompiler for PrCallCompiler {
    type TargetPattern = PrCall;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let name = node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing pattern, predicate, or sort name"))?;
        let name = name.text().trim();
        let named_args_count = node.named_children_by_field_name("named_args").count();
        let info = if let Some(info) = context.compilation.predicate_definition_info.get(name) {
            info
        } else if let Some(info) = context.compilation.function_definition_info.get(name) {
            info
        } else {
            bail!("predicate or function definition not found: {name}. Try running grit init.");
        };
        let params = collect_params(&info.parameters);
        let expected_params = Some(params.clone());
        let named_args = node.named_children_by_field_name("named_args");
        let named_args =
            node_to_args_pairs(named_args, context.compilation.lang, name, &expected_params)?;
        let args = named_args_to_hash_map(named_args, context)?;
        if args.len() != named_args_count {
            return Err(anyhow!("duplicate named args for invocation of {name}"));
        }

        let args = match_args_to_params(name, args, &params, context.compilation.lang)?;
        Ok(PrCall::new(info.index, args))
    }
}

fn collect_params(parameters: &[(String, Range)]) -> Vec<String> {
    parameters.iter().map(|p| p.0.clone()).collect::<Vec<_>>()
}

fn match_args_to_params(
    name: &str,
    mut args: BTreeMap<String, Pattern>,
    params: &[String],
    language: &impl Language,
) -> Result<Vec<Option<Pattern>>> {
    for (arg, _) in args.iter() {
        if !params.contains(arg) {
            bail!(
                format!("attempting to call pattern {name}, with invalid parameter {arg}. Valid parameters are: ") +
                &params
                    .iter()
                    .map(|p| p.strip_prefix(language.metavariable_prefix()).unwrap_or(p))
                    .join(", ")
            )
        }
    }
    Ok(params.iter().map(|param| args.remove(param)).collect())
}

fn named_args_to_hash_map(
    named_args: Vec<(String, NodeWithSource)>,
    context: &mut NodeCompilationContext,
) -> Result<BTreeMap<String, Pattern>> {
    named_args
        .into_iter()
        .map(|(name, node)| {
            let name = context.compilation.lang.metavariable_prefix().to_owned() + &name;
            let pattern = PatternCompiler::from_node_with_rhs(&node, context, true)?;
            Ok((name, pattern))
        })
        .collect()
}

fn node_to_args_pairs<'a>(
    named_args: impl Iterator<Item = NodeWithSource<'a>>,
    lang: &impl Language,
    kind: &str,
    expected_params: &'a Option<Vec<String>>,
) -> Result<Vec<(String, NodeWithSource<'a>)>> {
    named_args
        .enumerate()
        .map(|(i, node)| {
            if let Some(var) = node.child_by_field_name("variable") {
                let name = var.text().trim();
                let name = match name
                    .strip_prefix(lang.metavariable_prefix()) {
                        Some(stripped) => if expected_params.as_ref().is_some_and(|e| !e.contains(&name.to_string()) && !e.contains(&stripped.to_string())) {
                            None
                        } else {
                            Some(stripped)
                        },
                        None => None,
                    }
                    .or_else(|| match expected_params {
                        Some(params) => params.get(i).map(|p| p.strip_prefix(lang.metavariable_prefix()).unwrap_or(p)),
                        None => None,
                    })
                    .ok_or_else(|| if let Some(exp) = expected_params {
                        anyhow!("Too many params for {kind}: expected maximum {}", exp.len())
                    } else {
                        anyhow!("Variable {name} in params is missing a prefix. Try prepending an attribute name: for example, instead of `ensure_import_from(js\"'package'\")`, try `ensure_import_from(source=js\"'package'\")`")
                    })?;
                Ok((name.to_owned(), var))
            } else {
                let name = node
                    .child_by_field_name("name")
                    .ok_or_else(|| anyhow!("missing name of named arg"))?;
                let name = name.text().trim();
                let pattern = node
                    .child_by_field_name("pattern")
                    .ok_or_else(|| anyhow!("missing pattern of named arg"))?;
                Ok((name.to_owned(), pattern))
            }
        })
        .collect()
}
