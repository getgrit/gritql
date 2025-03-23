#[macro_use]
extern crate napi_derive;

use marzano_gritmodule::{
    config::{get_stdlib_modules, DefinitionSource, ResolvedGritDefinition},
    fetcher::ModuleRepo,
    resolver::resolve_patterns,
    searcher::find_grit_dir_from,
};
use napi::bindgen_prelude::*;
use std::path::{Path, PathBuf};

use anyhow::Context;
use api::SharedGritConfig;

use marzano_gritmodule::api::{parse_grit_config, read_grit_config};

mod binding;
mod search;

pub use search::QueryBuilder;
