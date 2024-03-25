use super::{
    ast_node::ASTNode,
    built_in_functions::CallBuiltIn,
    compiler::CompilationContext,
    file_pattern::FilePattern,
    function_definition::FunctionDefinition,
    functions::{CallForeignFunction, CallFunction, Evaluator, FuncEvaluation},
    patterns::Matcher,
    patterns::{Name, Pattern},
    resolved_pattern::ResolvedPattern,
    variable::VariableSourceLocations,
    State,
};
use crate::context::Context;
use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use marzano_language::language::Language;
use marzano_util::{
    analysis_logs::AnalysisLogs, position::Range, tree_sitter_util::children_by_field_name_count,
};
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Clone, Debug)]
pub struct Call {
    pub(crate) index: usize,
    pub(crate) args: Vec<Option<Pattern>>,
}

fn node_to_args_pairs<'a>(
    named_args: impl Iterator<Item = Node<'a>>,
    src: &'a str,
    lang: &impl Language,
    kind: &str,
    expected_params: &'a Option<Vec<String>>,
) -> Result<Vec<(String, Node<'a>)>> {
    named_args
        .enumerate()
        .map(|(i, node)| {
            if let Some(var) = node.child_by_field_name("variable") {
                let name = var.utf8_text(src.as_bytes())?;
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
                let name = name.utf8_text(src.as_bytes())?;
                let name = name.trim();
                let pattern = node
                    .child_by_field_name("pattern")
                    .ok_or_else(|| anyhow!("missing pattern of named arg"))?;
                Ok((name.to_owned(), pattern))
            }
        })
        .collect()
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
                format!("attempting to call pattern {}, with invalid parameter {}. Valid parameters are: ",
                name,
                arg) + &params
                    .iter()
                    .map(|p| p.strip_prefix(language.metavariable_prefix()).unwrap_or(p))
                    .join(", ")
            )
        }
    }
    Ok(params.iter().map(|param| args.remove(param)).collect())
}

fn collect_params(parameters: &[(String, Range)]) -> Vec<String> {
    parameters.iter().map(|p| p.0.clone()).collect::<Vec<_>>()
}

impl Call {
    pub fn new(index: usize, args: Vec<Option<Pattern>>) -> Self {
        Self { index, args }
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        is_rhs: bool,
        logs: &mut AnalysisLogs,
    ) -> Result<Pattern> {
        let sort = node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of nodeLike"))?;
        let kind = sort.utf8_text(context.src.as_bytes())?;
        let kind = kind.trim();
        let sort = context.lang.get_ts_language().id_for_node_kind(kind, true);
        let expected_params = if let Some(built_in) = context
            .built_ins
            .get_built_ins()
            .iter()
            .find(|b| b.name == kind)
        {
            Some(built_in.params.iter().map(|s| s.to_string()).collect_vec())
        } else if let Some(info) = context.function_definition_info.get(kind) {
            Some(collect_params(&info.parameters))
        } else if let Some(info) = context.foreign_function_definition_info.get(kind) {
            Some(collect_params(&info.parameters))
        } else {
            context
                .pattern_definition_info
                .get(kind)
                .map(|info| collect_params(&info.parameters))
        };
        let named_args_count = children_by_field_name_count(node, "named_args");
        let mut cursor = node.walk();
        let named_args = node
            .children_by_field_name("named_args", &mut cursor)
            .filter(|n| n.is_named());
        let named_args = node_to_args_pairs(
            named_args,
            context.src,
            context.lang,
            kind,
            &expected_params,
        )?;

        // tree-sitter returns 0 for sorts/kinds it doesn't know about
        if sort != 0 {
            return Ok(Pattern::ASTNode(Box::new(ASTNode::from_args(
                named_args,
                context,
                vars,
                vars_array,
                scope_index,
                sort,
                global_vars,
                is_rhs,
                logs,
            )?)));
        }
        let mut args = named_args_to_hash_map(
            named_args,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
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
                context.built_ins,
                index,
                context.lang,
            )?)))
        } else if let Some(info) = context.function_definition_info.get(kind) {
            let args =
                match_args_to_params(kind, args, &collect_params(&info.parameters), context.lang)?;
            Ok(Pattern::CallFunction(Box::new(CallFunction::new(
                info.index, args,
            ))))
        } else if let Some(info) = context.foreign_function_definition_info.get(kind) {
            let args =
                match_args_to_params(kind, args, &collect_params(&info.parameters), context.lang)?;
            Ok(Pattern::CallForeignFunction(Box::new(
                CallForeignFunction::new(info.index, args),
            )))
        } else {
            let info = context.pattern_definition_info.get(kind).ok_or_else(|| {
                anyhow!(
                    "pattern definition not found: {}. Try running grit init.",
                    kind
                )
            })?;
            let args =
                match_args_to_params(kind, args, &collect_params(&info.parameters), context.lang)?;
            Ok(Pattern::Call(Box::new(Self::new(
                (info.index).to_owned(),
                args,
            ))))
        }
    }
}

fn named_args_to_hash_map(
    named_args: Vec<(String, Node)>,
    context: &CompilationContext,
    vars: &mut BTreeMap<String, usize>,
    vars_array: &mut Vec<Vec<VariableSourceLocations>>,
    scope_index: usize,
    global_vars: &mut BTreeMap<String, usize>,
    logs: &mut AnalysisLogs,
) -> Result<BTreeMap<String, Pattern>> {
    let mut args = BTreeMap::new();
    for (name, node) in named_args {
        let pattern = Pattern::from_node(
            &node,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            true,
            logs,
        )?;
        args.insert(
            context.lang.metavariable_prefix().to_owned() + &name,
            pattern,
        );
    }
    Ok(args)
}

impl Name for Call {
    fn name(&self) -> &'static str {
        "CALL"
    }
}

// todo parameters, and name should both be usize references
// argument should throw an error if its not a parameter at compile time
impl Matcher for Call {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let pattern_definition = &context.pattern_definitions()[self.index];

        pattern_definition.call(state, binding, context, logs, &self.args)
    }
}

#[derive(Debug, Clone)]
pub struct PrCall {
    index: usize,
    pub(crate) args: Vec<Option<Pattern>>,
}

impl PrCall {
    pub fn new(index: usize, args: Vec<Option<Pattern>>) -> Self {
        Self { index, args }
    }

    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        let name = node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing pattern, predicate, or sort name"))?;
        let name = name.utf8_text(context.src.as_bytes())?;
        let name = name.trim();
        let named_args_count = children_by_field_name_count(node, "named_args");
        let info = if let Some(info) = context.predicate_definition_info.get(name) {
            info
        } else if let Some(info) = context.function_definition_info.get(name) {
            info
        } else {
            bail!(
                "predicate or function definition not found: {}. Try running grit init.",
                name
            );
        };
        let params = collect_params(&info.parameters);
        let expected_params = Some(params.clone());
        let mut cursor = node.walk();
        let named_args = node
            .children_by_field_name("named_args", &mut cursor)
            .filter(|n| n.is_named());
        let named_args = node_to_args_pairs(
            named_args,
            context.src,
            context.lang,
            name,
            &expected_params,
        )?;
        let args = named_args_to_hash_map(
            named_args,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        if args.len() != named_args_count {
            return Err(anyhow!("duplicate named args for invocation of {}", name));
        }

        let args = match_args_to_params(name, args, &params, context.lang)?;
        Ok(Self::new(info.index, args))
    }
}

impl Name for PrCall {
    fn name(&self) -> &'static str {
        "PREDICATE_CALL"
    }
}

impl Evaluator for PrCall {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let predicate_definition = &context.predicate_definitions().get(self.index);
        if let Some(predicate_definition) = predicate_definition {
            let predicator = predicate_definition.call(state, context, &self.args, logs)?;
            Ok(FuncEvaluation {
                predicator,
                ret_val: None,
            })
        } else {
            let function_definition = &context.function_definitions().get(self.index);
            if let Some(function_definition) = function_definition {
                let res = function_definition.call(state, context, &self.args, logs)?;
                Ok(res)
            } else {
                bail!(
                    "predicate or function definition not found: {}. Try running grit init.",
                    self.index
                );
            }
        }
    }
}
