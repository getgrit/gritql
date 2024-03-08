use super::{
    patterns::{Matcher, Name},
    resolved_pattern::ResolvedPattern,
    Context, State,
};
use crate::binding::{node_text, Binding};
use anyhow::{anyhow, Result};
use core::fmt::Debug;
use marzano_language::language::{Language, LeafEquivalenceClass, SortId};
use marzano_util::analysis_logs::AnalysisLogs;
use marzano_util::tree_sitter_util::named_children_by_field_id;

#[derive(Debug, Clone)]
pub struct StringConstant {
    pub text: String,
}
use tree_sitter::Node;

impl StringConstant {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub(crate) fn from_node(node: &Node, src: &str) -> Result<Self> {
        let text = node.utf8_text(src.as_bytes())?.trim().to_string();
        let text = text.strip_prefix('\"').unwrap().strip_suffix('\"').unwrap();
        let text = text.replace("\\\"", "\"").replace("\\\\", "\\");
        Ok(Self::new(text))
    }
}

impl Name for StringConstant {
    fn name(&self) -> &'static str {
        "STRING_CONSTANT"
    }
}

// this does what a raw string should do
// TODO: rename this, and implement StringConstant that checks sort.
impl Matcher for StringConstant {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        _context: &Context<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let text = binding.text(&state.files)?;
        if text == self.text {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Debug, Clone)]
pub struct AstLeafNode {
    sort: SortId,
    equivalence_class: Option<LeafEquivalenceClass>,
    text: String,
}

impl AstLeafNode {
    pub fn new(sort: SortId, text: &str, language: &impl Language) -> Result<Self> {
        let equivalence_class = language
            .get_equivalence_class(sort, text)
            .map_err(|e| anyhow!(e))?;
        let text = text.trim();
        Ok(Self {
            sort,
            equivalence_class,
            text: text.to_owned(),
        })
    }
}

impl Name for AstLeafNode {
    fn name(&self) -> &'static str {
        "AST_LEAF_NODE"
    }
}

impl Matcher for AstLeafNode {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        _state: &mut State<'a>,
        _context: &Context<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        if let ResolvedPattern::Binding(b) = binding {
            let (src, n) = match b.last() {
                Some(Binding::Node(src, n)) => (src, n.to_owned()),
                Some(Binding::List(src, n, f)) => {
                    let mut w = n.walk();
                    let mut l = named_children_by_field_id(n, &mut w, *f);
                    if let (Some(n), None) = (l.next(), l.next()) {
                        (src, n)
                    } else {
                        return Ok(false);
                    }
                }
                Some(Binding::ConstantRef(..))
                | Some(Binding::Empty(..))
                | Some(Binding::FileName(..))
                | Some(Binding::String(..))
                | None => return Ok(false),
            };
            if let Some(e) = &self.equivalence_class {
                let text = node_text(src, &n);
                return Ok(e.are_equivalent(n.kind_id(), text.trim()));
            } else if self.sort != n.kind_id() {
                return Ok(false);
            }
            let text = node_text(src, &n);
            if text.trim() == self.text {
                return Ok(true);
            } else {
                return Ok(false);
            }
        }
        Ok(false)
    }
}
