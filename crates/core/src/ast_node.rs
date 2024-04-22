use crate::binding::Binding;
use crate::marzano_resolved_pattern::MarzanoResolvedPattern;
use crate::{
    pattern::{
        ast_node_pattern::{AstLeafNodePattern, AstNodePattern},
        iter_pattern::PatternOrPredicate,
        patterns::{Matcher, Pattern, PatternName},
        resolved_pattern::ResolvedPattern,
        state::State,
        MarzanoContext,
    },
    problem::MarzanoQueryContext,
};
use anyhow::{anyhow, Result};
use grit_util::{AnalysisLogs, AstNode, Language};
use marzano_language::language::{FieldId, LeafEquivalenceClass, MarzanoLanguage, SortId};
use marzano_util::node_with_source::NodeWithSource;

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
        let Some(binding) = binding.get_last_binding() else {
            return Ok(false);
        };
        let Some(node) = binding.singleton() else {
            return Ok(false);
        };
        if binding.is_list() {
            return self.execute(
                &ResolvedPattern::from_node_binding(node),
                init_state,
                context,
                logs,
            );
        }

        if node.node.kind_id() != self.sort {
            return Ok(false);
        }
        if self.args.is_empty() {
            return Ok(true);
        }
        if context.language.is_comment_sort(self.sort) {
            let Some(content) = context.language.comment_text_range(&node) else {
                return Ok(false);
            };

            return self.args[0].2.execute(
                &ResolvedPattern::from_range_binding(content, node.source),
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
                    &MarzanoResolvedPattern::from_list_binding(node.clone(), *field_id),
                    &mut cur_state,
                    context,
                    logs,
                )
            } else if let Some(child) = node.child_by_field_id(*field_id) {
                pattern.execute(
                    &MarzanoResolvedPattern::from_node_binding(child),
                    &mut cur_state,
                    context,
                    logs,
                )
            } else {
                pattern.execute(
                    &MarzanoResolvedPattern::from_empty_binding(node.clone(), *field_id),
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
    pub fn new<'a>(sort: SortId, text: &str, language: &impl MarzanoLanguage<'a>) -> Result<Self> {
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
        let Some(node) = binding.get_last_binding().and_then(Binding::singleton) else {
            return Ok(false);
        };
        if let Some(e) = &self.equivalence_class {
            Ok(e.are_equivalent(node.node.kind_id(), node.text()?.trim()))
        } else if self.sort != node.node.kind_id() {
            Ok(false)
        } else {
            Ok(node.text()?.trim() == self.text)
        }
    }
}
