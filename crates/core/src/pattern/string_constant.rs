use super::{
    patterns::{Matcher, Name},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{binding::Binding, context::Context};
use anyhow::{anyhow, Result};
use core::fmt::Debug;
use grit_util::AstNode;
use marzano_language::language::{Language, LeafEquivalenceClass, SortId};
use marzano_util::analysis_logs::AnalysisLogs;

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
        _context: &'a impl Context,
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
        _context: &'a impl Context,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let ResolvedPattern::Binding(b) = binding else {
            return Ok(false);
        };
        let Some(node) = b.last().and_then(Binding::singleton) else {
            return Ok(false);
        };
        if let Some(e) = &self.equivalence_class {
            Ok(e.are_equivalent(node.node.kind_id(), node.text().trim()))
        } else if self.sort != node.node.kind_id() {
            Ok(false)
        } else {
            Ok(node.text().trim() == self.text)
        }
    }
}
