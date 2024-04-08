use super::{
    patterns::Matcher,
    patterns::{Name, Pattern},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{context::Context, resolve};
use anyhow::Result;
use marzano_language::language::{FieldId, Language, SortId};
use marzano_util::{analysis_logs::AnalysisLogs, node_with_source::NodeWithSource};

#[derive(Debug, Clone)]
pub struct ASTNode {
    pub(crate) sort: SortId,
    pub(crate) args: Vec<(FieldId, bool, Pattern)>,
}

impl ASTNode {
    pub fn new(sort: SortId, args: Vec<(FieldId, bool, Pattern)>) -> Self {
        Self { sort, args }
    }
}

impl Name for ASTNode {
    fn name(&self) -> &'static str {
        "ASTNODE"
    }
}

impl Matcher for ASTNode {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let binding = if let ResolvedPattern::Binding(binding) = binding {
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
                    &ResolvedPattern::from_list(source, node.clone(), *field_id),
                    &mut cur_state,
                    context,
                    logs,
                )
            } else if let Some(child) = node.child_by_field_id(*field_id) {
                pattern.execute(
                    &ResolvedPattern::from_node(NodeWithSource::new(child, source)),
                    &mut cur_state,
                    context,
                    logs,
                )
            } else {
                pattern.execute(
                    &ResolvedPattern::empty_field(source, node.clone(), *field_id),
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
