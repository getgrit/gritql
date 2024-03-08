use anyhow::Result;
use tree_sitter::Node;

use super::{
    patterns::{Matcher, Name},
    resolved_pattern::{ResolvedPattern, ResolvedSnippet},
    state::State,
    Context,
};
use crate::binding::{Binding, Constant};
use marzano_util::{analysis_logs::AnalysisLogs, tree_sitter_util::children_by_field_id_count};

#[derive(Debug, Clone)]
pub struct BooleanConstant {
    pub value: bool,
}

impl BooleanConstant {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub(crate) fn from_node(node: &Node, src: &str) -> Result<Self> {
        let text = node.utf8_text(src.as_bytes())?.trim().to_string();
        let value = match text.as_str() {
            "true" => true,
            "false" => false,
            _ => return Err(anyhow::anyhow!("Invalid boolean value")),
        };
        Ok(Self::new(value))
    }
}

impl Name for BooleanConstant {
    fn name(&self) -> &'static str {
        "BOOLEAN_CONSTANT"
    }
}

impl Matcher for BooleanConstant {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        _context: &Context<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let binding_as_bool = match binding {
            ResolvedPattern::Binding(bindings) => {
                if let Some(b) = bindings.last() {
                    evaluate_binding_truthiness(b)
                } else {
                    false
                }
            }
            ResolvedPattern::List(elements) => !elements.is_empty(),
            ResolvedPattern::Map(map) => !map.is_empty(),
            ResolvedPattern::Constant(c) => match c {
                Constant::Integer(i) => *i != 0,
                Constant::Float(d) => *d != 0.0,
                Constant::Boolean(b) => *b,
                Constant::String(s) => !s.is_empty(),
                Constant::Undefined => false,
            },
            ResolvedPattern::Snippets(s) => {
                if let Some(s) = s.last() {
                    match s {
                        ResolvedSnippet::Binding(b) => evaluate_binding_truthiness(b),
                        ResolvedSnippet::Text(t) => !t.is_empty(),
                        ResolvedSnippet::LazyFn(t) => !t.text(&state.files)?.is_empty(),
                    }
                } else {
                    false
                }
            }
            ResolvedPattern::File(..) => true,
            ResolvedPattern::Files(..) => true,
        };
        Ok(binding_as_bool == self.value)
    }
}

fn evaluate_binding_truthiness(b: &Binding) -> bool {
    match b {
        Binding::Empty(..) => false,
        Binding::List(_, node, field_id) => {
            let child_count = children_by_field_id_count(node, *field_id);
            child_count > 0
        }
        Binding::Node(..) => true,
        // This refers to a slice of the source code, not a Grit string literal, so it is truthy
        Binding::String(..) => true,
        Binding::FileName(_) => true,
        Binding::ConstantRef(c) => match c {
            Constant::Integer(i) => *i != 0,
            Constant::Float(d) => *d != 0.0,
            Constant::Boolean(b) => *b,
            Constant::String(s) => !s.is_empty(),
            Constant::Undefined => false,
        },
    }
}
