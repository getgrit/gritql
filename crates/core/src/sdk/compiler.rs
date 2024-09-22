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
#[derive(Clone, Copy)]
pub struct StatelessCompilerContext {
    lang: TargetLanguage,
}

impl StatelessCompilerContext {
    pub fn new(lang: TargetLanguage) -> Self {
        Self { lang }
    }

    /// Parse a snippet of code and returns a pattern
    pub fn parse_snippet(&mut self, content: &str) -> Result<Pattern<MarzanoQueryContext>> {
        let range = ByteRange::new(0, content.len());
        let snippet = parse_snippet_content(content, range, self, false)?;
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

#[cfg(test)]
mod tests {
    use marzano_language::target_language::TargetLanguage;

    use crate::{pattern_compiler::CompiledPatternBuilder, sdk::compiler::StatelessCompilerContext};

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
