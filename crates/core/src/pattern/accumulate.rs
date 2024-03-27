use anyhow::{anyhow, bail, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::borrow::Cow;
use std::collections::BTreeMap;

use super::code_snippet::CodeSnippet;
use super::compiler::CompilationContext;
use super::dynamic_snippet::DynamicPattern;
use super::functions::{Evaluator, FuncEvaluation};
use super::variable::VariableSourceLocations;
use super::{
    patterns::Pattern,
    patterns::{Matcher, Name},
    resolved_pattern::ResolvedPattern,
    State,
};
use super::{Effect, EffectKind};
use crate::context::Context;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct Accumulate {
    pub(crate) left: Pattern,
    pub(crate) right: Pattern,
    dynamic_right: Option<DynamicPattern>,
}

impl Accumulate {
    pub fn new(left: Pattern, right: Pattern, dynamic_right: Option<DynamicPattern>) -> Self {
        Self {
            left,
            right,
            dynamic_right,
        }
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
        let left_node = node
            .child_by_field_name("left")
            .ok_or_else(|| anyhow!("missing variable of patternAccumulateString"))?;
        let left = Pattern::from_node(
            &left_node,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        let right_node = node
            .child_by_field_name("right")
            .ok_or_else(|| anyhow!("missing pattern of patternAccumulateString"))?;
        let right = Pattern::from_node(
            &right_node,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            true,
            logs,
        )?;
        let dynamic_right = match right.clone() {
            Pattern::Dynamic(r) => Some(r),
            Pattern::CodeSnippet(CodeSnippet {
                dynamic_snippet: Some(r),
                ..
            }) => Some(r),
            Pattern::Variable(v) => Some(DynamicPattern::Variable(v)),
            Pattern::ASTNode(_)
            | Pattern::List(_)
            | Pattern::ListIndex(_)
            | Pattern::Map(_)
            | Pattern::Accessor(_)
            | Pattern::Call(_)
            | Pattern::Regex(_)
            | Pattern::File(_)
            | Pattern::Files(_)
            | Pattern::Bubble(_)
            | Pattern::Limit(_)
            | Pattern::CallBuiltIn(_)
            | Pattern::CallFunction(_)
            | Pattern::CallForeignFunction(_)
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
            | Pattern::Dots
            | Pattern::Like(_)
            | Pattern::Sequential(_) => None,
        };
        Ok(Self::new(left, right, dynamic_right))
    }
}

impl Name for Accumulate {
    fn name(&self) -> &'static str {
        "ACCUMULATE"
    }
}

impl Matcher for Accumulate {
    fn execute<'a>(
        &'a self,
        context_node: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        if let Pattern::Variable(var) = &self.left {
            let var = state.trace_var(var);
            let append = ResolvedPattern::from_pattern(&self.right, state, context, logs)?;
            if let Some(base) = state.bindings[var.scope].back_mut().unwrap()[var.index]
                .value
                .as_mut()
            {
                base.extend(append, &mut state.effects, context.language())?;
                Ok(true)
            } else {
                bail!(
                    "Variable {} is not bound",
                    state.bindings[var.scope].last().unwrap()[var.index].name
                )
            }
        } else {
            let resolved = if !self.left.execute(context_node, state, context, logs)? {
                return Ok(false);
            } else {
                Cow::Borrowed(context_node)
            };
            let bindings = match resolved.as_ref() {
                ResolvedPattern::Binding(b) => b,
                ResolvedPattern::Constant(_) => {
                    bail!("variable on left hand side of insert side-conditions cannot be bound to a constant")
                }
                ResolvedPattern::File(_) => {
                    bail!("variable on left hand side of insert side-conditions cannot be bound to a file, try rewriting the content, or name instead")
                }
                ResolvedPattern::Files(_) => {
                    bail!("variable on left hand side of insert side-conditions cannot be bound to a files node")
                }
                ResolvedPattern::List(_) => {
                    bail!("variable on left hand side of insert side-conditions cannot be bound to a list pattern")
                }
                ResolvedPattern::Map(_) => {
                    bail!("variable on left hand side of insert side-conditions cannot be bound to a map pattern")
                }
                ResolvedPattern::Snippets(_) => {
                    bail!("variable on left hand side of insert side-conditions cannot be bound to snippets")
                }
            };
            let dynamic_right = match &self.dynamic_right {
                Some(r) => r,
                None => {
                    bail!(
                        "Insert right hand side must be a code snippet when LHS is not a variable, but found: {:?}", self.right
                    )
                }
            };
            let mut replacement: ResolvedPattern<'_> =
                ResolvedPattern::from_dynamic_pattern(dynamic_right, state, context, logs)?;
            let effects: Result<Vec<Effect>> = bindings
                .iter()
                .map(|b| {
                    let is_first = !state.effects.iter().any(|e| e.binding == *b);
                    replacement.normalize_insert(b, is_first, context.language())?;
                    Ok(Effect {
                        binding: b.clone(),
                        pattern: replacement.clone(),
                        kind: EffectKind::Insert,
                    })
                })
                .collect();
            let effects = effects?;
            state.effects.extend(effects);
            Ok(true)
        }
    }
}

impl Evaluator for Accumulate {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        if let Pattern::Variable(var) = &self.left {
            let var = state.trace_var(var);
            let append = ResolvedPattern::from_pattern(&self.right, state, context, logs)?;
            if let Some(base) = state.bindings[var.scope].back_mut().unwrap()[var.index]
                .value
                .as_mut()
            {
                base.extend(append, &mut state.effects, context.language())?;
                Ok(FuncEvaluation {
                    predicator: true,
                    ret_val: None,
                })
            } else {
                bail!(
                    "Variable {} is not bound",
                    state.bindings[var.scope].last().unwrap()[var.index].name
                )
            }
        } else {
            bail!("Insert side-conditions must have variable on left-hand side");
        }
    }
}
