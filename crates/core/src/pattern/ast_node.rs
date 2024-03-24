use crate::{binding::Binding, context::Context, resolve};

use super::{
    compiler::CompilationContext,
    list::List,
    patterns::Matcher,
    patterns::{Name, Pattern},
    resolved_pattern::ResolvedPattern,
    variable::VariableSourceLocations,
    State,
};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use marzano_language::language::{FieldId, Language, SortId};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct ASTNode {
    pub(crate) sort: SortId,
    pub(crate) args: Vec<(FieldId, bool, Pattern)>,
}

// TreeSitter Field IDs are roughly sequential so to
// avoid conflicts with the language's field IDs we
// chose a large number for our artificial field IDs.
const COMMENT_CONTENT_FIELD_ID: FieldId = 10000;

impl ASTNode {
    pub fn new(sort: SortId, args: Vec<(FieldId, bool, Pattern)>) -> Self {
        Self { sort, args }
    }

    // todo should the pattern always be a list? feels like it shouldn't
    // but don't remember why I implemented this way comeback to this later

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_args(
        named_args: Vec<(String, Node)>,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        sort: SortId,
        global_vars: &mut BTreeMap<String, usize>,
        is_rhs: bool,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        let mut args = Vec::new();
        if context.lang.is_comment(sort) {
            if named_args.len() > 1 {
                return Err(anyhow!("comment node has more than one field"));
            }
            if named_args.is_empty() {
                return Ok(Self::new(sort, args));
            }
            let (name, node) = &named_args[0];
            if *name != "content" {
                return Err(anyhow!("unknown field name {} for comment node", name));
            }
            let pattern = Pattern::from_node(
                &node.clone(),
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                false,
                logs,
            )?;
            args.push((COMMENT_CONTENT_FIELD_ID, false, pattern));
            return Ok(Self::new(sort, args));
        }
        for (name, node) in named_args {
            let node_fields = &context.lang.node_types()[usize::from(sort)];

            let node_field_names = node_fields
                .iter()
                .map(|f| {
                    context
                        .lang
                        .get_ts_language()
                        .field_name_for_id(f.id())
                        .unwrap()
                        .to_string()
                })
                .join(", ");

            let id = context
                .lang
                .get_ts_language()
                .field_id_for_name(&name)
                .ok_or_else(|| {
                    let node_sort = &context
                        .lang
                        .get_ts_language()
                        .node_kind_for_id(sort)
                        .unwrap()
                        .to_string();
                    anyhow!(
                        "invalid field `{}` for node `{}`, valid fields are: {}",
                        name,
                        node_sort,
                        node_field_names
                    )
                })?;

            let field = node_fields.iter().find(|f| f.id() == id).ok_or_else(|| {
                let node_sort = &context
                    .lang
                    .get_ts_language()
                    .node_kind_for_id(sort)
                    .unwrap()
                    .to_string();
                anyhow!(
                    "invalid field `{}` for node `{}`, valid fields are: {}",
                    name,
                    node_sort,
                    node_field_names
                )
            })?;

            let pattern = List::from_node_in_context(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                field,
                global_vars,
                is_rhs,
                logs,
            )?;
            args.push((id, field.multiple(), pattern));
        }
        if args.len() != args.iter().unique_by(|a| a.0).count() {
            return Err(anyhow!("duplicate field in node"));
        }
        Ok(Self::new(sort, args))
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
        context: &'a impl Context<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let binding = if let ResolvedPattern::Binding(binding) = binding {
            resolve!(binding.last())
        } else {
            return Ok(false);
        };
        let (src, node) = match binding {
            Binding::Empty(_, _, _) => return Ok(false),
            Binding::Node(src, node) => (src, node),
            // maybe String should instead be fake node? eg for comment_content
            Binding::String(_, _) => return Ok(false),
            Binding::List(src, node, id) => {
                let mut cursor = node.walk();
                let mut list = node.children_by_field_id(*id, &mut cursor);
                if let Some(child) = list.next() {
                    if list.next().is_some() {
                        return Ok(false);
                    }
                    return self.execute(
                        &ResolvedPattern::from_binding(Binding::Node(src, child)),
                        init_state,
                        context,
                        logs,
                    );
                }
                return Ok(false);
            }
            Binding::FileName(_) => return Ok(false),
            Binding::ConstantRef(_) => return Ok(false),
        };
        if node.kind_id() != self.sort {
            return Ok(false);
        }
        if self.args.is_empty() {
            return Ok(true);
        }
        if context.language().is_comment(self.sort) {
            let content = context.language().comment_text(node, src);
            let content = resolve!(content);
            let content = Binding::String(src, content.1);

            return self.args[0].2.execute(
                &ResolvedPattern::from_binding(content),
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
                    &ResolvedPattern::from_list(src, node.clone(), *field_id),
                    &mut cur_state,
                    context,
                    logs,
                )
            } else if let Some(child) = node.child_by_field_id(*field_id) {
                pattern.execute(
                    &ResolvedPattern::from_node(src, child),
                    &mut cur_state,
                    context,
                    logs,
                )
            } else {
                pattern.execute(
                    &ResolvedPattern::empty_field(src, node.clone(), *field_id),
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
