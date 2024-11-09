"""
A script to add a new language to the gritql metavariable-grammars.
"""

# Future imports (must occur at the beginning of the file):
from __future__ import annotations  # https://www.python.org/dev/peps/pep-0585/

# Standard library imports:
import os
import re
import json
from argparse import Namespace, ArgumentParser
from subprocess import run, check_output, CalledProcessError

resources_path = os.path.dirname(os.path.realpath(__file__))
repo_path = os.path.abspath(os.path.join(resources_path, os.pardir))

# https://regex101.com/r/nRTpjv/1
ALL_LANGUAGES = re.compile(
    r"""\/{51}
\/\/ Constants
\/{51}
const allLanguages = (?P<all_languages>\[
  (\s*"[\w\-]+",\n?)+
  (?P<last_language>\s*"[\w\-]+",)
\])""",
)

LANGUAGE_TEMPLATE_RS = """use crate::language::{{fields_for_nodes, Field, MarzanoLanguage, NodeTypes, SortId, TSLanguage}};
use grit_util::Language;
use marzano_util::node_with_source::NodeWithSource;
use std::sync::OnceLock;

static NODE_TYPES_STRING: &str =
    include_str!("../../../resources/node-types/{language}-node-types.json");
static NODE_TYPES: OnceLock<Vec<Vec<Field>>> = OnceLock::new();
static LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();

#[cfg(not(feature = "builtin-parser"))]
fn language() -> TSLanguage {{
    unimplemented!(
        "tree-sitter parser must be initialized before use when [builtin-parser] is off."
    )
}}
#[cfg(feature = "builtin-parser")]
fn language() -> TSLanguage {{
    tree_sitter_{language}::language().into()
}}

#[derive(Debug, Clone, Copy)]
pub struct {language_title_case} {{
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    comment_sorts: [SortId; 2],
    language: &'static TSLanguage,
}}

impl NodeTypes for {language_title_case} {{
    fn node_types(&self) -> &[Vec<Field>] {{
        self.node_types
    }}
}}

impl {language_title_case} {{
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {{
        todo!();
    }}
    pub(crate) fn is_initialized() -> bool {{
        LANGUAGE.get().is_some()
    }}
}}

impl {language_title_case} {{
    use_marzano_delegate!();

    fn language_name(&self) -> &'static str {{
        "{language}"
    }}

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {{
        todo!();
    }}
}}

impl<'a> MarzanoLanguage<'a> for {language_title_case} {{
    fn get_ts_language(&self) -> &TSLanguage {{
        self.language
    }}

    fn is_comment_sort(&self, id: SortId) -> bool {{
        self.comment_sorts.contains(&id)
    }}

    fn metavariable_sort(&self) -> SortId {{
        self.metavariable_sort
    }}
}}

#[cfg(test)]
mod tests {{
    use crate::language::nodes_from_indices;

    use super::*;

    #[test]
    fn pair_snippet() {{
        let snippet = "public";
        let lang = {language_title_case}::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }}
}}
"""


def main(args: Namespace):
    """Automate the process of adding a new language to the gritql metavariable-grammars."""
    language = args.language.lower()
    language_upper_case = language.upper()
    language_title_case = language.title()
    language_rs = LANGUAGE_TEMPLATE_RS.format(
        language=language, language_title_case=language_title_case
    )

    grammar_js_path = os.path.join(resources_path, "edit_grammars.mjs")
    with open(grammar_js_path, "r") as f:
        grammar_js = f.read()
    match = ALL_LANGUAGES.search(grammar_js)
    assert match, "Could not find the allLanguages array in edit_grammars.mjs"
    print("Found the allLanguages array in edit_grammars.mjs")
    print(match.group(0))

    # Extract the allLanguages array and insert the new language
    orig_all_languages_str = match.group("all_languages")
    all_languages: list[str] = json.loads(orig_all_languages_str.replace(",\n]", "\n]"))
    assert language not in all_languages, f"{language} already exists"
    all_languages.append(language)
    all_languages.sort()  # sort the languages alphabetically
    all_languages_str = json.dumps(all_languages, indent=2).replace("\n]", ",\n]")
    print(all_languages_str)

    updated_grammar_js = grammar_js.replace(orig_all_languages_str, all_languages_str)

    # Write the updated edit_grammars.mjs file
    with open(grammar_js_path, "w") as f:
        f.write(updated_grammar_js)

    # check if tree-sitter is installed:
    try:
        output = check_output(["node", "--version"], cwd=repo_path).decode().strip()
        print("using", output)
    except CalledProcessError:
        print("node is not installed. Please install it first.")
        return

    run(["node", "./resources/edit_grammars.mjs", language], check=True, cwd=repo_path)

    language_rs_path = os.path.join(repo_path, "crates/language/src", f"{language}.rs")
    if os.path.exists(language_rs_path):
        print(f"{language_rs_path} already exists")
    else:
        print(f"adding a new template crates/language/src/{language}.rs file...")
        with open(language_rs_path, "w") as f:
            f.write(language_rs)

    run(["git", "status"], check=True, cwd=repo_path)

    # TODO: automate more of these steps:
    print("*" * 80)
    print(f"""Please complete the following steps:
    1. patch the metavariable grammar to include $.grit_metavariable anywhere we want to substitute a metavariable
    2. add `tree-sitter-{language}` to crates/language/Cargo.toml [dependencies] and [features.builtin-parser]
    3. add `pub mod {language};` to crates/language/src/lib.rs
    4. add `use crate::{language}::{language_title_case}` to crates/language/src/target_language.rs and add it to all enums and match statements
    5. add {language} to the `PatternsDirectory` struct in crates/gritmodule/src/patterns_directory.rs and add it to all match statements
    6. add {language_title_case} to all match statements in crates/lsp/src/language.rs
    7. add {language_upper_case}_LANGUAGE as a static in crates/wasm-bindings/src/match_pattern.rs and add it to all match statements
    8. add test cases for {language} in crates/core/src/test.rs
    """)


if __name__ == "__main__":
    parser = ArgumentParser(
        description="Add a new language to the gritql metavariable-grammars."
    )
    parser.add_argument(
        "--language",
        "--lang",
        help="The name of the language to add. e.g. 'kotlin'",
        required=True,
    )
    args = parser.parse_args()
    main(args)
