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
