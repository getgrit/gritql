mod analytics;
mod analyze;
pub mod commands;
mod community;
pub mod error;
mod flags;
mod github;
mod jsonl;
mod lister;
mod messenger_variant;
mod resolver;
mod result_formatting;
mod scan;
mod updater;
mod utils;
mod ux;
#[cfg(feature = "workflows_v2")]
mod workflows;

// git2 uses openssl, but breaks windows, so we need 
// to import openssl and specify the vendored feature in order
// to prevet git2 from breaking on windows
use openssl as _;