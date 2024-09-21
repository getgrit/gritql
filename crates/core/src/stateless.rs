use anyhow::{bail, Result};
use grit_pattern_matcher::pattern::{DynamicSnippetPart, Pattern, Variable};
use grit_util::ByteRange;
use marzano_language::target_language::TargetLanguage;

use crate::{
    pattern_compiler::{
        compiler::SnippetCompilationContext, snippet_compiler::parse_snippet_content,
    },
    problem::MarzanoQueryContext,
};

/// As opposed to our standard StatelessCompiler,
/// the StatelessCompiler can handle snippets without needing to maintain scopes
pub struct StatelessCompilerContext {
    lang: TargetLanguage,
}

impl StatelessCompilerContext {
    #[allow(dead_code)]
    pub fn new(lang: TargetLanguage) -> Self {
        Self { lang }
    }

    /// Parse a snippet of code and returns a pattern
    #[allow(dead_code)]
    pub fn parse_snippet(&mut self, content: &str) -> Result<Pattern<MarzanoQueryContext>> {
        let range = ByteRange::new(0, content.len());
        let snippet = parse_snippet_content(content, range, self, false)?;
        println!("snippet: {:?}", snippet);
        Ok(snippet)
    }
}

impl SnippetCompilationContext for StatelessCompilerContext {
    fn get_lang(&self) -> &TargetLanguage {
        &self.lang
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
}
