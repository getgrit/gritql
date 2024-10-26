use crate::{
    language::{fields_for_nodes, Field, MarzanoLanguage, NodeTypes, SortId, TSLanguage, Tree},
    notebooks::MarzanoNotebookParser,
};
use grit_util::{Ast, AstNode, CodeRange, Language, Parser, Replacement};
use marzano_util::node_with_source::NodeWithSource;
use std::sync::OnceLock;

static NODE_TYPES_STRING: &str =
    include_str!("../../../resources/node-types/python-node-types.json");
static NODE_TYPES: OnceLock<Vec<Vec<Field>>> = OnceLock::new();
static LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();

#[cfg(not(feature = "builtin-parser"))]
fn language() -> TSLanguage {
    unimplemented!(
        "tree-sitter parser must be initialized before use when [builtin-parser] is off."
    )
}
#[cfg(feature = "builtin-parser")]
fn language() -> TSLanguage {
    tree_sitter_python::language().into()
}

#[derive(Debug, Clone, Copy)]
pub struct Python {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    comment_sort: SortId,
    skip_padding_sorts: [SortId; 1],
    language: &'static TSLanguage,
}

impl Python {
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {
        let language = LANGUAGE.get_or_init(|| lang.unwrap_or_else(language));
        let node_types = NODE_TYPES.get_or_init(|| fields_for_nodes(language, NODE_TYPES_STRING));
        let metavariable_sort = language.id_for_node_kind("grit_metavariable", true);
        let comment_sort = language.id_for_node_kind("comment", true);
        let skip_padding_sorts = [language.id_for_node_kind("string", true)];
        Self {
            node_types,
            metavariable_sort,
            comment_sort,
            skip_padding_sorts,
            language,
        }
    }
    pub(crate) fn is_initialized() -> bool {
        LANGUAGE.get().is_some()
    }
}

impl NodeTypes for Python {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

impl Language for Python {
    use_marzano_delegate!();

    fn language_name(&self) -> &'static str {
        "Python"
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[
            ("", ""),
            ("{ ", " }"),
            ("", "\ndef GRIT_FUNCTION():\n    return;"),
            ("GRIT_FN(", ")"),
        ]
    }

    fn comment_prefix(&self) -> &'static str {
        "#"
    }

    fn check_replacements(&self, n: NodeWithSource<'_>, replacements: &mut Vec<Replacement>) {
        if n.node.is_error() {
            if n.text().is_ok_and(|t| t == "->") || n.text().is_ok_and(|t| t == ",") {
                replacements.push(Replacement::new(n.range(), ""));
            }
            return;
        }
        if n.node.kind() == "import_from_statement" {
            if let Some(name_field) = n.node.child_by_field_name("name") {
                let names_text = name_field
                    .utf8_text(n.source.as_bytes())
                    .unwrap_or_default();
                // If we have an empty names text remove the whole thing
                if names_text.trim().is_empty() {
                    replacements.push(Replacement::new(n.range(), ""));
                    return;
                }
            }
            if let Ok(t) = n.text() {
                let mut end_range = n.range();
                end_range.start_byte = end_range.end_byte;
                let mut did_close_paren = false;

                // Delete: from x import ()
                let chars = t.chars().rev();
                for ch in chars {
                    end_range.start_byte -= 1;
                    if ch == ')' {
                        did_close_paren = true
                    } else if did_close_paren && ch == '(' {
                        // Delete: from x import ()
                        replacements.push(Replacement::new(n.range(), ""));
                        break;
                    } else if ch == ',' {
                        if !did_close_paren {
                            // Delete: the , from x import foo, *and keep looking*
                            replacements.push(Replacement::new(end_range, ""));
                        } else {
                            break;
                        }
                    } else if !ch.is_whitespace() {
                        break;
                    }
                }

                if !did_close_paren {
                    let mut removal_range = n.range();
                    removal_range.end_byte = removal_range.start_byte;
                    // If we have content after the newline, that is a problem and likely corrupt
                    for ch in t.chars() {
                        if ch == '\n' {
                            // Assume everything after this is a problem
                            replacements.push(Replacement::new(removal_range, ""));
                            break;
                        } else {
                            removal_range.end_byte += 1;
                        }
                    }
                }
            }
        }
    }

    fn should_pad_snippet(&self) -> bool {
        true
    }

    fn make_single_line_comment(&self, text: &str) -> String {
        format!("# {}\n", text)
    }
}

impl<'a> MarzanoLanguage<'a> for Python {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn is_comment_sort(&self, id: SortId) -> bool {
        id == self.comment_sort
    }

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }

    fn get_parser(&self) -> Box<dyn Parser<Tree = Tree>> {
        Box::new(MarzanoNotebookParser::new(self, "python"))
    }

    fn should_skip_padding(&self, node: &NodeWithSource<'_>) -> bool {
        self.skip_padding_sorts.contains(&node.node.kind_id())
    }

    fn get_skip_padding_ranges_for_snippet(&self, snippet: &str) -> Vec<CodeRange> {
        let mut parser = self.get_parser();
        let snippet = parser.parse_snippet("", snippet, "");
        let root = snippet.tree.root_node();
        MarzanoLanguage::get_skip_padding_ranges(self, &root)
    }
}

#[cfg(test)]
mod tests {
    use crate::language::nodes_from_indices;

    use super::*;

    #[test]
    fn kv_snippet() {
        let snippet = "$key: $value";
        let lang = Python::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        println!("nodes: {:#?}", nodes);
        assert!(!nodes.is_empty());
    }
}
