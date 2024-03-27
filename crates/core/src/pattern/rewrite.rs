use super::{
    code_snippet::CodeSnippet,
    compiler::CompilationContext,
    dynamic_snippet::DynamicPattern,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    variable::VariableSourceLocations,
    variable_content::VariableContent,
    Effect, EffectKind, State,
};
use crate::context::Context;
use anyhow::{anyhow, bail, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::{AnalysisLogBuilder, AnalysisLogs};
use std::{borrow::Cow, collections::BTreeMap};
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct Rewrite {
    pub(crate) left: Pattern,
    pub(crate) right: DynamicPattern,
    pub(crate) _annotation: Option<String>,
}

impl Rewrite {
    pub fn new(left: Pattern, right: DynamicPattern, _annotation: Option<String>) -> Self {
        Self {
            left,
            right,
            _annotation,
        }
    }

    // do we want to add support for annotations?
    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
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
        Ok(Self::new(left, right, annotation))
    }

    /**
     * Execute a rewrite rule, returning the new binding.
     *
     * If called from a rewrite side-condition, the binding should be None.
     * In this case, the left-hand side must be a variable, and the binding
     * will be taken from the current state.
     *
     * If called from a rewrite pattern, the binding should be Some(the current node).
     */
    pub(crate) fn execute_generalized<'a>(
        &'a self,
        resolved: Option<&ResolvedPattern<'a>>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let resolved = match resolved {
            Some(b) => {
                if !self.left.execute(b, state, context, logs)? {
                    return Ok(false);
                } else {
                    Cow::Borrowed(b)
                }
            }
            None => {
                if let Pattern::Variable(v) = &self.left {
                    let var = state.trace_var(v);
                    if let Some(VariableContent {
                        value: Some(content),
                        ..
                    }) = state
                        .bindings
                        .get(var.scope)
                        .and_then(|scope| scope.last().unwrap().get(var.index))
                        .cloned()
                        .map(|b| *b)
                    {
                        Cow::Owned(content)
                    } else {
                        bail!("Variable {:?} not bound", v);
                    }
                } else {
                    bail!("Rewrite side-conditions must have variable on left-hand side");
                }
            }
        };
        let bindings = match resolved.as_ref() {
            ResolvedPattern::Binding(b) => b,
            ResolvedPattern::Constant(_) => {
                bail!("variable on left hand side of rewrite side-conditions cannot be bound to a constant")
            }
            ResolvedPattern::File(_) => {
                bail!("variable on left hand side of rewrite side-conditions cannot be bound to a file, try rewriting the content, or name instead")
            }
            ResolvedPattern::Files(_) => {
                bail!("variable on left hand side of rewrite side-conditions cannot be bound to a files node")
            }
            ResolvedPattern::List(_) => {
                bail!("variable on left hand side of rewrite side-conditions cannot be bound to a list pattern")
            }
            ResolvedPattern::Map(_) => {
                bail!("variable on left hand side of rewrite side-conditions cannot be bound to a map pattern")
            }
            ResolvedPattern::Snippets(_) => {
                bail!("variable on left hand side of rewrite side-conditions cannot be bound to snippets")
            }
        };
        let replacement: ResolvedPattern<'_> =
            ResolvedPattern::from_dynamic_pattern(&self.right, state, context, logs)?;
        let effects = bindings.iter().map(|b| Effect {
            binding: b.clone(),
            pattern: replacement.clone(),
            kind: EffectKind::Rewrite,
        });
        state.effects.extend(effects);
        Ok(true)
    }
}

impl Name for Rewrite {
    fn name(&self) -> &'static str {
        "REWRITE"
    }
}

impl Matcher for Rewrite {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        self.execute_generalized(Some(binding), state, context, logs)
    }
}

impl Evaluator for Rewrite {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let predicator = self.execute_generalized(None, state, context, logs)?;
        Ok(FuncEvaluation {
            predicator,
            ret_val: None,
        })
    }
}
