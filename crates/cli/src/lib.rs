mod analytics;
mod analyze;
pub mod commands;
mod community;
mod diff;
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

// We use git2, which depends on openssl, which by-default wants to
// dynamically link libopenssl. We explicitly depend on openssl to
// force on the vendored feature, making our binaries more portable.
//
// On windows this trick should *not* be used because git2 automatically
// uses completely different dependencies, and this trick would randomly
// force openssl into our build, breaking msvc.
#[cfg(not(windows))]
use openssl as _;
