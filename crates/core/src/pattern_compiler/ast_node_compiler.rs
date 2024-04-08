use super::{
    compiler::NodeCompilationContext, list_compiler::ListCompiler, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::pattern::ast_node::ASTNode;
use anyhow::{anyhow, Result};
use itertools::Itertools;
use marzano_language::language::{FieldId, Language, SortId};
use marzano_util::node_with_source::NodeWithSource;

// TreeSitter Field IDs are roughly sequential so to
// avoid conflicts with the language's field IDs we
// chose a large number for our artificial field IDs.
const COMMENT_CONTENT_FIELD_ID: FieldId = 10000;

pub(crate) struct AstNodeCompiler;

impl AstNodeCompiler {
    // todo should the pattern always be a list? feels like it shouldn't
    // but don't remember why I implemented this way comeback to this later
    pub(crate) fn from_args(
        named_args: Vec<(String, NodeWithSource)>,
        sort: SortId,
        context: &mut NodeCompilationContext,
    ) -> Result<ASTNode> {
        let mut args = Vec::new();
        if context.compilation.lang.is_comment(sort) {
            if named_args.len() > 1 {
                return Err(anyhow!("comment node has more than one field"));
            }
            if named_args.is_empty() {
                return Ok(ASTNode::new(sort, args));
            }
            let (name, node) = &named_args[0];
            if *name != "content" {
                return Err(anyhow!("unknown field name {} for comment node", name));
            }
            let pattern = PatternCompiler::from_node(node.clone(), context)?;
            args.push((COMMENT_CONTENT_FIELD_ID, false, pattern));
            return Ok(ASTNode::new(sort, args));
        }
        for (name, node) in named_args {
            let node_fields = &context.compilation.lang.node_types()[usize::from(sort)];

            let node_field_names = node_fields
                .iter()
                .map(|f| {
                    context
                        .compilation
                        .lang
                        .get_ts_language()
                        .field_name_for_id(f.id())
                        .unwrap()
                        .to_string()
                })
                .join(", ");

            let id = context
                .compilation
                .lang
                .get_ts_language()
                .field_id_for_name(&name)
                .ok_or_else(|| {
                    let node_sort = &context
                        .compilation
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
                    .compilation
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

            let pattern = ListCompiler::from_node_in_context(node, field, context)?;
            args.push((id, field.multiple(), pattern));
        }
        if args.len() != args.iter().unique_by(|a| a.0).count() {
            return Err(anyhow!("duplicate field in node"));
        }
        Ok(ASTNode::new(sort, args))
    }
}
