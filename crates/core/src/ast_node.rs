use crate::binding::Binding;
use crate::marzano_resolved_pattern::MarzanoResolvedPattern;
use crate::pattern::ast_node_pattern::AstLeafNodePattern;
use crate::pattern::iter_pattern::PatternOrPredicate;
use crate::pattern::MarzanoContext;
use crate::{context::ExecContext, resolve};
use crate::{
    pattern::{
        ast_node_pattern::AstNodePattern,
        patterns::{Matcher, Pattern, PatternName},
        resolved_pattern::ResolvedPattern,
        state::State,
    },
    problem::MarzanoQueryContext,
};
use anyhow::{anyhow, Result};
use grit_util::AstNode;
use marzano_language::language::{FieldId, Language, LeafEquivalenceClass, SortId};
use marzano_util::{analysis_logs::AnalysisLogs, node_with_source::NodeWithSource};

#[derive(Debug, Clone)]
pub struct ASTNode {
    pub(crate) sort: SortId,
    pub(crate) args: Vec<(FieldId, bool, Pattern<MarzanoQueryContext>)>,
}

impl ASTNode {
    pub fn new(sort: SortId, args: Vec<(FieldId, bool, Pattern<MarzanoQueryContext>)>) -> Self {
        Self { sort, args }
    }
}

impl AstNodePattern<MarzanoQueryContext> for ASTNode {
    fn children(&self) -> Vec<PatternOrPredicate<MarzanoQueryContext>> {
        self.args
            .iter()
            .map(|a| PatternOrPredicate::Pattern(&a.2))
            .collect()
    }

    fn matches_kind_of(&self, node: &NodeWithSource) -> bool {
        self.sort == node.node.kind_id()
    }
}

impl PatternName for ASTNode {
    fn name(&self) -> &'static str {
        "ASTNODE"
    }
}

impl Matcher<MarzanoQueryContext> for ASTNode {
    fn execute<'a>(
        &'a self,
        binding: &MarzanoResolvedPattern<'a>,
        init_state: &mut State<'a, MarzanoQueryContext>,
        context: &'a MarzanoContext,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let binding = if let MarzanoResolvedPattern::Binding(binding) = binding {
            resolve!(binding.last())
        } else {
            return Ok(false);
        };
        let Some(node) = binding.singleton() else {
            return Ok(false);
        };
        if binding.is_list() {
            return self.execute(&ResolvedPattern::from_node(node), init_state, context, logs);
        }

        let NodeWithSource { node, source } = node;
        if node.kind_id() != self.sort {
            return Ok(false);
        }
        if self.args.is_empty() {
            return Ok(true);
        }
        if context.language().is_comment(self.sort) {
            let content = context.language().comment_text(&node, source);
            let content = resolve!(content);

            return self.args[0].2.execute(
                &ResolvedPattern::from_range(content.1, source),
                init_state,
                context,
                logs,
            );
        }
        let mut running_state = init_state.clone();
        for (field_id, is_list, pattern) in &self.args {
            let mut cur_state = running_state.clone();

            let res = if *is_list {
                pattern.execute(
                    &MarzanoResolvedPattern::from_list(
                        NodeWithSource::new(node.clone(), source),
                        *field_id,
                    ),
                    &mut cur_state,
                    context,
                    logs,
                )
            } else if let Some(child) = node.child_by_field_id(*field_id) {
                pattern.execute(
                    &MarzanoResolvedPattern::from_node(NodeWithSource::new(child, source)),
                    &mut cur_state,
                    context,
                    logs,
                )
            } else {
                pattern.execute(
                    &MarzanoResolvedPattern::empty_field(
                        NodeWithSource::new(node.clone(), source),
                        *field_id,
                    ),
                    &mut cur_state,
                    context,
                    logs,
                )
            };
            if res? {
                running_state = cur_state;
            } else {
                return Ok(false);
            }
        }
        *init_state = running_state;
        Ok(true)
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

impl AstLeafNodePattern<MarzanoQueryContext> for AstLeafNode {}

impl PatternName for AstLeafNode {
    fn name(&self) -> &'static str {
        "AST_LEAF_NODE"
    }
}

impl Matcher<MarzanoQueryContext> for AstLeafNode {
    fn execute<'a>(
        &'a self,
        binding: &MarzanoResolvedPattern<'a>,
        _state: &mut State<'a, MarzanoQueryContext>,
        _context: &'a MarzanoContext<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let MarzanoResolvedPattern::Binding(b) = binding else {
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
