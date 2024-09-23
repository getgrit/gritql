use anyhow::Result;
use grit_pattern_matcher::pattern::{And, Contains, Pattern};

use std::fmt::Debug;
use std::sync::Arc;

use crate::{
    built_in_functions::CallbackFn,
    problem::{MarzanoQueryContext, Problem},
};

use super::{LanguageSdk, StatelessCompilerContext};

#[cfg(feature = "wasm_core")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "napi")]
use super::binding::ResultBinding;
#[cfg(feature = "napi")]
use napi::bindgen_prelude::*;

#[derive(Clone)]
struct SimpleCallback {
    callback: Arc<CallbackFn>,
}

impl Debug for SimpleCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SimpleCallback")
    }
}

impl SimpleCallback {
    #[allow(unused)]
    fn new(callback: Arc<CallbackFn>) -> Self {
        SimpleCallback { callback }
    }
}

/// UncompiledPattern is used to build up complex patterns *before* building them
/// Late compilation allows us to reuse the same pattern across languages (where a snippet will ultimately be parsed differently
/// It also allows the pattern to be used as a root pattern, or dynamically inside a function callback
#[derive(Debug, Clone)]
#[allow(unused)]
enum UncompiledPattern {
    Contains {
        contains: Box<UncompiledPatternBuilder>,
    },
    Snippet {
        text: String,
    },
    Callback {
        callback: SimpleCallback,
    },
    And {
        patterns: Vec<UncompiledPatternBuilder>,
    },
}

#[cfg_attr(feature = "wasm_core", wasm_bindgen)]
#[cfg_attr(feature = "napi", napi)]
#[derive(Clone, Debug)]
pub struct UncompiledPatternBuilder {
    pattern: UncompiledPattern,
}

// Methods we can use in Rust, but are not exported directly to host languages
impl UncompiledPatternBuilder {
    #[allow(unused)]
    fn new(pattern: UncompiledPattern) -> Self {
        UncompiledPatternBuilder { pattern }
    }

    fn compile(
        self,
        context: &mut StatelessCompilerContext,
    ) -> Result<Pattern<MarzanoQueryContext>> {
        match self.pattern {
            UncompiledPattern::Snippet { text } => {
                let sdk = LanguageSdk::default();
                let pattern = sdk.snippet(&text)?;
                Ok(pattern)
            }
            UncompiledPattern::And { patterns } => {
                let mut compiled: Vec<Pattern<MarzanoQueryContext>> = vec![];
                for pattern in patterns {
                    compiled.push(pattern.compile(context)?);
                }
                Ok(Pattern::And(Box::new(And::new(compiled))))
            }
            UncompiledPattern::Contains { contains } => {
                let compiled = contains.compile(context)?;
                // TODO: we probably want to auto-bubble?
                Ok(Pattern::Contains(Box::new(Contains::new(compiled, None))))
            }
            UncompiledPattern::Callback { callback } => {
                let built_in = context.built_ins.add_callback(Box::new(
                    move |binding, context, state, logs| {
                        let result = (callback.callback)(binding, context, state, logs)?;
                        Ok(result)
                    },
                ));
                Ok(built_in)
            }
        }
    }

    pub fn build(self) -> Result<Problem> {
        let mut sdk = LanguageSdk::default();
        let mut compiler = sdk.compiler();

        let compiled = self.compile(&mut compiler)?;
        let built = sdk.build(compiler.built_ins, compiled)?;
        Ok(built)
    }
}

// This is the API that host languages will use
#[cfg_attr(feature = "wasm_core", wasm_bindgen)]
#[cfg_attr(feature = "napi", napi)]
#[cfg(feature = "napi_or_wasm")]
impl UncompiledPatternBuilder {
    #[napi(factory, js_name = "new_snippet")]
    pub fn new_snippet(text: String) -> Self {
        UncompiledPatternBuilder {
            pattern: UncompiledPattern::Snippet { text },
        }
    }

    /// Filter this pattern to only match instances that contain the other pattern
    #[napi(js_name = "contains")]
    pub fn contains(&self, other: &UncompiledPatternBuilder) -> Self {
        let me = self.clone();
        let contains = UncompiledPatternBuilder::new(UncompiledPattern::Contains {
            contains: Box::new(other.clone()),
        });
        UncompiledPatternBuilder {
            pattern: UncompiledPattern::And {
                patterns: vec![me, contains],
            },
        }
    }
}

/// This implements features that should only be used from Napi
#[cfg(feature = "napi")]
#[napi]
impl UncompiledPatternBuilder {
    /// Filter the pattern to only match instances that match a provided callback
    #[napi]
    pub fn filter(
        &self,
        callback: napi::threadsafe_function::ThreadsafeFunction<
            ResultBinding,
            napi::threadsafe_function::ErrorStrategy::Fatal,
        >,
    ) -> Self {
        let me = self.clone();

        let inner_callback =
            SimpleCallback::new(Arc::new(move |binding, context, state, _logs| {
                let runtime = context
                    .runtime
                    .handle
                    .as_ref()
                    .ok_or(anyhow::anyhow!("Async runtime required"))?;

                let foreign_binding = ResultBinding::new_unsafe(binding, context, state);

                let val = runtime
                    .block_on(async { callback.call_async::<bool>(foreign_binding).await })?;

                Ok(val)
            }));
        let callback_pattern = UncompiledPatternBuilder::new(UncompiledPattern::Callback {
            callback: inner_callback,
        });

        UncompiledPatternBuilder::new(UncompiledPattern::And {
            patterns: vec![me, callback_pattern],
        })
    }

    async fn run_inner(&self, files: Vec<RichFile>) -> Result<Vec<MatchResult>> {
        let problem = self.clone().build()?;
        let context = ExecutionContext::default();

        let results = problem.execute_files(files, &context);
        Ok(results)
    }

    /// Run the pattern on a list of files and return the number of matching files found
    #[napi]
    pub async fn run_on_files(&self, files: Vec<RichFile>) -> napi::Result<u32> {
        let results = self
            .run_inner(files)
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        let mut matched_files = 0;
        for result in results {
            if result.is_match() {
                matched_files += 1;
            }
        }
        Ok(matched_files as u32)
    }

    /// Apply the query to a single file and return the modified file (if any)
    /// @param file The file to apply the query to
    /// @returns The modified file (if it was modified)
    #[napi]
    pub async fn run_on_file(&self, file: RichFile) -> napi::Result<Option<RichFile>> {
        let results = self
            .run_inner(vec![file])
            .await
            .map_err(|e| napi::Error::from_reason(format!("Error: {:?}", e)))?;

        for result in results {
            match result {
                MatchResult::RemoveFile(file) => {
                    return Ok(Some(RichFile {
                        path: file.file_name().to_string(),
                        content: "".to_string(),
                    }));
                }
                MatchResult::Rewrite(rewrite) => {
                    return Ok(Some(RichFile {
                        path: rewrite.file_name().to_string(),
                        content: rewrite.content().unwrap_or_default().to_string(),
                    }));
                }
                _ => {}
            }
        }

        Ok(None)
    }
}
