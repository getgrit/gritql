use super::{
    ast_node_compiler::AstNodeCompiler, compiler::NodeCompilationContext,
    node_compiler::NodeCompiler, pattern_compiler::PatternCompiler,
};
use crate::{built_in_functions::BuiltIns, problem::MarzanoQueryContext};
use anyhow::{anyhow, bail, Result};
use grit_pattern_matcher::pattern::{
    Call, CallBuiltIn, CallForeignFunction, CallFunction, FilePattern, Pattern, PrCall, Predicate,
};
use grit_util::{AstNode, ByteRange, Language};
use itertools::Itertools;
use marzano_language::language::MarzanoLanguage;
use marzano_util::node_with_source::NodeWithSource;
use std::collections::BTreeMap;

pub(crate) struct CallCompiler;

impl NodeCompiler for CallCompiler {
    type TargetPattern = Pattern<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Pattern<MarzanoQueryContext>> {
        let sort = node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of nodeLike"))?;
        let kind = sort.text()?;
        let kind = kind.trim();
        let lang = context.compilation.lang;
        let sort = lang.get_ts_language().id_for_node_kind(kind, true);
        let expected_params = if let Some(built_in) = context
            .compilation
            .built_ins
            .get_built_ins()
            .iter()
            .find(|b| b.name == kind)
        {
            if !built_in.position.is_pattern() {
                bail!("function {kind} cannot be called as a pattern");
            }

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
        let named_args = node_to_args_pairs(named_args, lang, kind, &expected_params)?;

        // tree-sitter returns 0 for sorts/kinds it doesn't know about
        if sort != 0 {
            return Ok(Pattern::AstNode(Box::new(AstNodeCompiler::from_args(
                named_args, sort, context, is_rhs,
            )?)));
        }
        let mut args = named_args_to_hash_map(named_args, context)?;
        if args.len() != named_args_count {
            return Err(anyhow!("duplicate named args for invocation of {}", kind));
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
            Ok(Pattern::CallBuiltIn(Box::new(BuiltIns::call_from_args(
                args,
                context.compilation.built_ins,
                index,
                lang,
                kind,
            )?)))
        } else if let Some(info) = context.compilation.function_definition_info.get(kind) {
            let args = match_args_to_params(kind, args, &collect_params(&info.parameters), lang)?;
            Ok(Pattern::CallFunction(Box::new(CallFunction::new(
                info.index, args,
            ))))
        } else if let Some(info) = context
            .compilation
            .foreign_function_definition_info
            .get(kind)
        {
            let args = match_args_to_params(kind, args, &collect_params(&info.parameters), lang)?;
            Ok(Pattern::CallForeignFunction(Box::new(
                CallForeignFunction::new(info.index, args),
            )))
        } else {
            let info = context
                .compilation
                .pattern_definition_info
                .get(kind)
                .ok_or_else(|| {
                    anyhow!(
                        "pattern definition not found: {}. Try running grit init.",
                        kind
                    )
                })?;
            let args = match_args_to_params(kind, args, &collect_params(&info.parameters), lang)?;
            Ok(Pattern::Call(Box::new(Call::new(info.index, args))))
        }
    }
}

pub(crate) struct PrCallCompiler;

impl NodeCompiler for PrCallCompiler {
    type TargetPattern = Predicate<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let name = node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing pattern, predicate, or sort name"))?;
        let name = name.text()?;
        let name = name.trim();

        if let Some((index, built_in)) = context
            .compilation
            .built_ins
            .get_built_ins()
            .iter()
            .enumerate()
            .find(|(_, built_in)| built_in.name == name)
        {
            if !built_in.position.is_predicate() {
                bail!("function {name} cannot be called as a predicate");
            }

            let params = built_in.params.iter().map(|s| s.to_string()).collect_vec();
            let args = args_from_params(node, context, name, params)?;
            return Ok(Predicate::CallBuiltIn(Box::new(CallBuiltIn::new(
                index, name, args,
            ))));
        }

        let info = if let Some(info) = context.compilation.predicate_definition_info.get(name) {
            info
        } else if let Some(info) = context.compilation.function_definition_info.get(name) {
            info
        } else {
            bail!("predicate or function definition not found: {name}. Try running grit init.");
        };

        let args = args_from_params(node, context, name, collect_params(&info.parameters))?;
        Ok(Predicate::Call(Box::new(PrCall::new(info.index, args))))
    }
}

fn args_from_params(
    node: &NodeWithSource<'_>,
    context: &mut NodeCompilationContext,
    name: &str,
    params: Vec<String>,
) -> Result<Vec<Option<Pattern<MarzanoQueryContext>>>> {
    let expected_params = Some(params.clone());
    let named_args = node.named_children_by_field_name("named_args");
    let named_args =
        node_to_args_pairs(named_args, context.compilation.lang, name, &expected_params)?;
    let args = named_args_to_hash_map(named_args, context)?;
    let named_args_count = node.named_children_by_field_name("named_args").count();
    if args.len() != named_args_count {
        bail!("duplicate named args for invocation of {name}");
    }

    match_args_to_params(name, args, &params, context.compilation.lang)
}

fn collect_params(parameters: &[(String, ByteRange)]) -> Vec<String> {
    parameters.iter().map(|p| p.0.clone()).collect()
}

fn match_args_to_params(
    name: &str,
    mut args: BTreeMap<String, Pattern<MarzanoQueryContext>>,
    params: &[String],
    language: &impl Language,
) -> Result<Vec<Option<Pattern<MarzanoQueryContext>>>> {
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
) -> Result<BTreeMap<String, Pattern<MarzanoQueryContext>>> {
    let mut args = BTreeMap::new();
    for (name, node) in named_args {
        let pattern = PatternCompiler::from_node_with_rhs(&node, context, true)?;
        args.insert(
            context.compilation.lang.metavariable_prefix().to_owned() + &name,
            pattern,
        );
    }
    Ok(args)
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
                let name = var.text()?;
                let name = name.trim();
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
                        anyhow!("Too many params for {}: expected maximum {}", kind, exp.len())
                    } else {
                        anyhow!("Variable {} in params is missing a prefix. Try prepending an attribute name: for example, instead of `ensure_import_from(js\"'package'\")`, try `ensure_import_from(source=js\"'package'\")`", name)
                    })?;
                Ok((name.to_owned(), var))
            } else {
                let name = node
                    .child_by_field_name("name")
                    .ok_or_else(|| anyhow!("missing name of named arg"))?;
                let name = name.text()?;
                let name = name.trim();
                let pattern = node
                    .child_by_field_name("pattern")
                    .ok_or_else(|| anyhow!("missing pattern of named arg"))?;
                Ok((name.to_owned(), pattern))
            }
        })
        .collect()
}
