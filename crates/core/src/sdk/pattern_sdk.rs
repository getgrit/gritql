use anyhow::Result;
use grit_pattern_matcher::pattern::Pattern;
use marzano_util::{rich_path::RichFile, runtime::ExecutionContext};

use crate::{
    api::{InputFile, MatchResult},
    problem::{MarzanoQueryContext, Problem},
};

use super::LanguageSdk;

#[cfg(feature = "wasm_core")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "napi")]
use napi::bindgen_prelude::*;

/// UncompiledPattern is used to build up complex patterns *before* building them
/// Late compilation allows us to reuse the same pattern across languages (where a snippet will ultimately be parsed differently
/// It also allows the pattern to be used as a root pattern, or dynamically inside a function callback
#[derive(Debug, Clone)]
pub enum UncompiledPattern {
    Contains { contains: Box<UncompiledPattern> },
    Snippet { text: String },
}

#[cfg_attr(feature = "wasm_core", wasm_bindgen)]
#[cfg_attr(feature = "napi", napi)]
#[derive(Clone)]
pub struct UncompiledPatternBuilder {
    pattern: UncompiledPattern,
}

// Methods we can use in Rust, but are not exported directly to host languages
impl UncompiledPatternBuilder {
    fn compile(&self) -> Result<Pattern<MarzanoQueryContext>> {
        match &self.pattern {
            UncompiledPattern::Snippet { text } => {
                let sdk = LanguageSdk::default();
                let pattern = sdk.snippet(text)?;
                Ok(pattern)
            }
            _ => Err(anyhow::anyhow!(
                "Unsupported pattern type {:?}",
                self.pattern
            )),
        }
    }

    pub fn build(&self) -> Result<Problem> {
        let compiled = self.compile()?;
        let mut sdk = LanguageSdk::default();
        let built = sdk.build(compiled)?;
        Ok(built)
    }
}

// This is the API that host languages will use
#[cfg_attr(feature = "wasm_core", wasm_bindgen)]
#[cfg_attr(feature = "napi", napi)]
impl UncompiledPatternBuilder {
    #[napi(factory, js_name = "new_snippet")]
    pub fn new_snippet(text: String) -> Self {
        UncompiledPatternBuilder {
            pattern: UncompiledPattern::Snippet { text },
        }
    }
}

/// This implements features that should only be used from Napi
#[cfg(feature = "napi")]
#[napi]
impl UncompiledPatternBuilder {
    async fn run_inner(
        &self,
        file_names: Vec<String>,
        file_contents: Vec<String>,
    ) -> Result<Vec<MatchResult>> {
        let problem = self.build()?;
        let context = ExecutionContext::default();
        let files: Vec<RichFile> = file_names
            .into_iter()
            .zip(file_contents.into_iter())
            .map(|(path, content)| RichFile::new(path, content))
            .collect();
        let results = problem.execute_files(files, &context);
        Ok(results)
    }
    #[napi]
    pub async fn run(
        &self,
        file_names: Vec<String>,
        file_contents: Vec<String>,
    ) -> napi::Result<u32> {
        let results = self
            .run_inner(file_names, file_contents)
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        println!("results: {:?}", results);
        Ok(results.len() as u32)
    }
}
