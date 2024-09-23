use anyhow::Result;
use grit_pattern_matcher::pattern::Pattern;

use crate::problem::{MarzanoQueryContext, Problem};

use super::LanguageSdk;

#[cfg(feature = "wasm_core")]
use wasm_bindgen::prelude::*;

/// UncompiledPattern is used to build up complex patterns *before* building them
/// Late compilation allows us to reuse the same pattern across languages (where a snippet will ultimately be parsed differently
/// It also allows the pattern to be used as a root pattern, or dynamically inside a function callback
#[derive(Debug)]
pub enum UncompiledPattern {
    Contains { contains: Box<UncompiledPattern> },
    Snippet { text: String },
}

#[cfg_attr(feature = "wasm_core", wasm_bindgen)]
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
impl UncompiledPatternBuilder {
    pub fn new_snippet(text: String) -> Self {
        UncompiledPatternBuilder {
            pattern: UncompiledPattern::Snippet { text },
        }
    }
}
