#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::Context;

// Exported API
pub use marzano_core::UncompiledPatternBuilder;

#[napi]
pub fn sum(a: i32, b: i32) -> Result<i32> {
    Ok(a + b)
}

/// We need this to make sure the builder is actually exposed to JS
#[napi]
pub fn debug_builder(builder: &UncompiledPatternBuilder) -> Result<String> {
    let builder_str = format!("{:?}", builder);
    Ok(builder_str)
}
