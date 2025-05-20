#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;

mod binding;
mod search;

pub use search::QueryBuilder;
