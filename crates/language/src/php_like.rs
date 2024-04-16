use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PHP_LIKE_EXACT_VARIABLE_REGEX: Regex = Regex::new(r"^\^([A-Za-z_][A-Za-z0-9_]*)$")
        .expect("Failed to compile PHP_LIKE_EXACT_VARIABLE_REGEX");
    static ref PHP_LIKE_VARIABLE_REGEX: Regex = Regex::new(r"\^(\.\.\.|[A-Za-z_][A-Za-z0-9_]*)")
        .expect("Failed to compile PHP_LIKE_VARIABLE_REGEX");
    static ref PHP_LIKE_BRACKET_VAR_REGEX: Regex = Regex::new(r"\^\[([A-Za-z_][A-Za-z0-9_]*)\]")
        .expect("Failed to compile PHP_LIKE_BRACKET_VAR_REGEX");
    pub static ref PHP_ONLY_CODE_SNIPPETS: Vec<(&'static str, &'static str)> = vec![
        ("", ""),
        ("", ";"),
        ("$", ";"),
        ("class GRIT_CLASS {", "}"),
        ("class GRIT_CLASS { ", " function GRIT_FN(); }"),
        (" GRIT_FN(", ") { }"),
        ("$GRIT_VAR = ", ";"),
        ("$GRIT_VAR = ", ""),
        ("[", "];"),
        ("", "{}"),
    ];
    pub static ref PHP_CODE_SNIPPETS: Vec<(&'static str, &'static str)> = {
        let mut php_tag_modifications: Vec<(&'static str, &'static str)> = PHP_ONLY_CODE_SNIPPETS
            .clone()
            .into_iter()
            .map(|(s1, s2)| {
                let owned_str1 = Box::leak(Box::new(format!("<?php {}", s1))) as &'static str;
                let owned_str2 = Box::leak(Box::new(format!("{} ?>", s2))) as &'static str;
                (owned_str1, owned_str2)
            })
            .collect();
        php_tag_modifications.extend(vec![("", "")]);
        php_tag_modifications
    };
}

pub(crate) fn php_like_metavariable_regex() -> &'static Regex {
    &PHP_LIKE_VARIABLE_REGEX
}

pub(crate) fn php_like_metavariable_bracket_regex() -> &'static Regex {
    &PHP_LIKE_BRACKET_VAR_REGEX
}

pub(crate) fn php_like_exact_variable_regex() -> &'static Regex {
    &PHP_LIKE_EXACT_VARIABLE_REGEX
}

pub(crate) fn php_like_metavariable_prefix() -> &'static str {
    "^"
}
