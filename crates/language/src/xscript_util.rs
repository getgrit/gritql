use anyhow::{anyhow, Result};
use marzano_util::{cursor_wrapper::CursorWrapper, tree_sitter_util::children_by_field_name_count};
use tree_sitter::{Node, Parser, Range, Tree};
use tree_sitter_traversal::{traverse, Order};

use crate::{
    language::{default_parse_file, Language, SortId, TSLanguage},
    parent_traverse::{ParentTraverse, TreeSitterParentCursor},
    vue::Vue,
};

pub static STATEMENT_NODE_NAMES: &[&str] = &[
    "break_statement",
    "continue_statement",
    "debugger_statement",
    "declaration",
    // subtypes of declaration
    "class_declaration",
    "function_declaration",
    "generator_function_declaration",
    "lexical_declaration",
    "variable_declaration",
    //
    "do_statement",
    "empty_statement",
    "export_statement",
    "expression_statement",
    "for_in_statement",
    "for_statement",
    "if_statement",
    "import_statement",
    "labeled_statement",
    "return_statement",
    "statement_block",
    "switch_statement",
    "throw_statement",
    "try_statement",
    "while_statement",
    "with_statement",
];

pub fn jslike_get_statement_sorts(lang: &TSLanguage) -> Vec<SortId> {
    STATEMENT_NODE_NAMES
        .iter()
        .map(|kind| lang.id_for_node_kind(kind, true))
        .collect()
}

fn is_js_lang_attribute(node: &Node, text: &[u8]) -> bool {
    let js_name_array = ["js", "ts", "tsx", "jsx", "javascript", "typescript"];
    node.child_by_field_name("name")
        .is_some_and(|name| name.utf8_text(text).unwrap().trim() == "lang")
        && node
            .child_by_field_name("value")
            .and_then(|n| {
                if n.kind() == "attribute_value" {
                    Some(n)
                } else if n.kind() == "quoted_attribute_value" {
                    n.child_by_field_name("value")
                } else {
                    None
                }
            })
            .is_some_and(|lang| js_name_array.contains(&lang.utf8_text(text).unwrap().trim()))
}

fn append_code_range(node: &Node, text: &[u8], ranges: &mut Vec<Range>) {
    if node.kind() == "script_element" {
        let mut cursor = node.walk();
        if let Some(mut attributes) = node
            .child_by_field_name("start_tag")
            .map(|n| n.children_by_field_name("atributes", &mut cursor))
        {
            if attributes.any(|n| is_js_lang_attribute(&n, text)) {
                if let Some(code) = node.child_by_field_name("text") {
                    ranges.push(code.range())
                }
            }
        };
    }
}

// could probably be done better using a tree-sitter query?
fn get_vue_ranges(file: &str) -> Result<Vec<Range>> {
    let vue = Vue::new(None);
    let mut parser = Parser::new()?;
    let text = file.as_bytes();
    parser.set_language(vue.get_ts_language())?;
    let tree = parser.parse(file, None)?.ok_or(anyhow!("missing tree"))?;
    let cursor = tree.walk();
    let mut ranges = Vec::new();
    for n in traverse(CursorWrapper::from(cursor), Order::Pre) {
        append_code_range(&n, text, &mut ranges)
    }
    Ok(ranges)
}

pub(crate) fn parse_file(
    lang: &impl Language,
    name: &str,
    body: &str,
    logs: &mut marzano_util::analysis_logs::AnalysisLogs,
    new: bool,
    parser: &mut Parser,
) -> anyhow::Result<Tree> {
    if name.ends_with(".vue") {
        let ranges = get_vue_ranges(body)?;
        parser.set_included_ranges(&ranges)?;
        parser.parse(body, None)?.ok_or(anyhow!("missing tree"))
    } else {
        default_parse_file(lang.get_ts_language(), name, body, logs, new)
    }
}

pub(crate) fn jslike_check_orphaned(
    n: Node<'_>,
    src: &str,
    orphan_ranges: &mut Vec<tree_sitter::Range>,
) {
    if n.is_error()
        && ["var", "let", "const"].contains(&n.utf8_text(src.as_bytes()).unwrap().as_ref())
        || n.kind() == "empty_statement"
    {
        orphan_ranges.push(n.range());
    } else if n.kind() == "import_statement" {
        for n in n.named_children(&mut n.walk()).filter(|n| n.is_error()) {
            if let Some(n) = n.children(&mut n.walk()).last() {
                if n.kind() == "," {
                    orphan_ranges.push(n.range())
                }
            }
        }
        if let Some(import_clause) = n.child_by_field_name("import") {
            if let Some(imports) = import_clause.child_by_field_name("name") {
                let named_imports = children_by_field_name_count(&imports, "imports");
                let namespace_import = children_by_field_name_count(&imports, "namespace");
                if named_imports == 0 && namespace_import == 0 {
                    orphan_ranges.push(n.range());
                }
            }
        }
    } else if n.is_error() && n.utf8_text(src.as_bytes()).unwrap() == "," {
        for ancestor in ParentTraverse::new(TreeSitterParentCursor::new(n.clone())) {
            if ancestor.kind() == "class_body" {
                orphan_ranges.push(n.range());
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use marzano_util::print_node::print_node;

    use crate::tsx::Tsx;

    use super::*;

    #[test]
    fn gets_ranges() {
        let snippet = r#"
<script lang="ts">
export default {
    inheritAttrs: false,
};
</script>

<script setup lang="ts">
defineProps<{
    itemCount?: number;
    showingCount?: string;
}>();
</script>

<template>
    <transition name="fade">
        <span v-if="itemCount" class="item-count">
            {{ showingCount }}
        </span>
    </transition>
</template>"#;
        let ranges = get_vue_ranges(snippet);
        println!("RANGES: {:#?}", ranges);
        let ts = Tsx::new(None);
        let ts_ts = ts.get_ts_language();
        let mut parser = Parser::new().unwrap();
        parser.set_language(ts_ts).unwrap();
        let tree = parse_file(
            &ts,
            "test.vue",
            snippet,
            &mut vec![].into(),
            false,
            &mut parser,
        )
        .unwrap();
        let root = tree.root_node();
        print_node(&root);
    }
}
