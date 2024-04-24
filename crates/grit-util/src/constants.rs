use once_cell::sync::Lazy;
use regex::Regex;

pub const GRIT_METAVARIABLE_PREFIX: &str = "$";

pub static EXACT_VARIABLE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\$([A-Za-z_][A-Za-z0-9_]*)$").unwrap());
pub static EXACT_REPLACED_VARIABLE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^µ([A-Za-z_][A-Za-z0-9_]*)$").unwrap());
pub static VARIABLE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\$(\.\.\.|[A-Za-z_][A-Za-z0-9_]*)").unwrap());
pub static REPLACED_VARIABLE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"µ(\.\.\.|[A-Za-z_][A-Za-z0-9_]*)").unwrap());
pub static BRACKET_VAR_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\$\[([A-Za-z_][A-Za-z0-9_]*)\]").unwrap());
