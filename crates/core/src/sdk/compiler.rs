use crate::{
    ast_node::ASTNode,
    built_in_functions::BuiltIns,
    pattern_compiler::{
        compiler::{DefinitionInfo, SnippetCompilationContext},
        snippet_compiler::parse_snippet_content,
    },
    problem::MarzanoQueryContext,
};
use anyhow::{anyhow, bail, Result};
use grit_pattern_matcher::pattern::{DynamicSnippetPart, Pattern, PatternDefinition, Variable};
use grit_util::ByteRange;
use itertools::Itertools;
use marzano_language::{
    language::MarzanoLanguage, language::NodeTypes, target_language::TargetLanguage,
};
use std::collections::HashMap;

const COMMENT_CONTENT_FIELD_ID: u16 = 10000;

/// As opposed to our standard StatelessCompiler,
/// the StatelessCompiler can handle snippets without needing to maintain scopes
pub struct StatelessCompilerContext {
    lang: TargetLanguage,
    pub built_ins: BuiltIns,
}

impl StatelessCompilerContext {
    pub fn new(lang: TargetLanguage) -> Self {
        Self {
            lang,
            built_ins: BuiltIns::get_built_in_functions(),
        }
    }

    /// Parse a snippet of code and returns a pattern
    pub fn parse_snippet(&mut self, content: &str) -> Result<Pattern<MarzanoQueryContext>> {
        let range = ByteRange::new(0, content.len());
        let snippet = parse_snippet_content(content, range, self, false)?;
        Ok(snippet)
    }

    /// Creates a new AST node pattern with the given name and fields
    pub fn node(
        &self,
        name: &str,
        fields: HashMap<&str, Pattern<MarzanoQueryContext>>,
    ) -> Result<Pattern<MarzanoQueryContext>> {
        // Get the sort ID for the node type
        let sort = self.lang.get_ts_language().id_for_node_kind(name, true);
        if sort == 0 {
            return Err(anyhow!("invalid node type: {}", name));
        }

        // Convert fields into (field_id, is_multiple, pattern) tuples
        let mut args = Vec::new();

        // Special handling for comment nodes
        if self.lang.is_comment_sort(sort) {
            if fields.len() > 1 {
                return Err(anyhow!("comment node cannot have more than one field"));
            }
            if let Some(content) = fields.get("content") {
                args.push((COMMENT_CONTENT_FIELD_ID, false, content.clone()));
            }
            return Ok(Pattern::AstNode(Box::new(ASTNode::new(sort, args))));
        }

        // Get node field information
        let node_fields = &self.lang.node_types()[usize::from(sort)];
        let node_field_names = node_fields
            .iter()
            .map(|f| {
                self.lang
                    .get_ts_language()
                    .field_name_for_id(f.id())
                    .unwrap()
                    .to_string()
            })
            .join(", ");

        // Process each field
        for (name, pattern) in fields {
            let node_sort = self.lang.get_ts_language().node_kind_for_id(sort).unwrap();
            let field_id = self.lang.get_ts_language()
                .field_id_for_name(name)
                .ok_or_else(|| {
                    if node_field_names.is_empty() {
                        anyhow!(
                            "invalid field `{}` for AST node `{}`. `{}` does not expose any fields.",
                            name,
                            node_sort,
                            node_sort,
                        )
                    } else {
                        anyhow!(
                            "invalid field `{}` for AST node `{}`, valid fields are: {}",
                            name,
                            node_sort,
                            node_field_names
                        )
                    }
                })?;

            let field = node_fields
                .iter()
                .find(|f| f.id() == field_id)
                .ok_or_else(|| anyhow!("field {} not found in node type {}", name, node_sort))?;

            args.push((field_id, field.multiple(), pattern));
        }

        // Check for duplicate fields
        if args.len() != args.iter().unique_by(|a| a.0).count() {
            return Err(anyhow!("duplicate field in node"));
        }

        Ok(Pattern::AstNode(Box::new(ASTNode::new(sort, args))))
    }
}

impl SnippetCompilationContext for StatelessCompilerContext {
    fn get_lang(&self) -> &TargetLanguage {
        &self.lang
    }

    fn register_match_variable(&mut self) -> Result<Variable> {
        bail!("The $match variable is not supported in the stateless SDK")
    }

    fn register_snippet_variable(
        &mut self,
        name: &str,
        source_range: Option<ByteRange>,
    ) -> Result<DynamicSnippetPart> {
        Ok(DynamicSnippetPart::Variable(
            self.register_variable(name, source_range)?,
        ))
    }

    fn register_variable(&mut self, name: &str, _range: Option<ByteRange>) -> Result<Variable> {
        if name.starts_with("$GLOBAL_") {
            bail!("Global variables are not supported in stateless mode")
        }
        Ok(Variable::new_dynamic(name))
    }

    fn get_pattern_definition(&self, _name: &str) -> Option<&DefinitionInfo> {
        None
    }

    fn register_ephemeral_pattern(
        &mut self,
        pattern: Pattern<MarzanoQueryContext>,
    ) -> Result<PatternDefinition<MarzanoQueryContext>> {
        Ok(PatternDefinition::new_ephemeral(vec![], pattern))
    }
}

#[cfg(test)]
mod tests {
    use marzano_language::target_language::TargetLanguage;

    use crate::{
        pattern_compiler::CompiledPatternBuilder, sdk::compiler::StatelessCompilerContext,
    };

    #[test]
    fn test_stateless_snippet_compiler_self_equivalence() {
        let language = TargetLanguage::default();
        let mut compiler = StatelessCompilerContext::new(language);
        let pattern = compiler.parse_snippet("console.log").unwrap();

        // Second instance
        let pattern2 = compiler.parse_snippet("console.log").unwrap();
        println!("pattern: {:?}", pattern);
        println!("pattern2: {:?}", pattern2);

        assert_eq!(format!("{:?}", pattern), format!("{:?}", pattern2));
    }

    #[test]
    fn test_stateless_snippet_compiler_equivalence() {
        let language = TargetLanguage::from_string("js", None).unwrap();
        let mut compiler = StatelessCompilerContext::new(language);
        let pattern = compiler.parse_snippet("console.log(name)").unwrap();

        // Check how the traditional compiler compiles the same snippet
        let builder = CompiledPatternBuilder::start_empty(
            "`console.log(name)`",
            TargetLanguage::from_string("js", None).unwrap(),
        )
        .unwrap();
        let pattern2 = builder.compile(None, None, false).unwrap().root_pattern();

        assert_eq!(format!("{:?}", pattern), format!("{:?}", pattern2));
    }
}
