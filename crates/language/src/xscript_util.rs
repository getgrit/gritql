use crate::{
    language::{default_parse_file, Language, SortId, TSLanguage},
    vue::get_vue_ranges,
};
use anyhow::anyhow;
use grit_util::AstNode;
use marzano_util::{
    node_with_source::NodeWithSource, tree_sitter_util::children_by_field_name_count,
};
use tree_sitter::{Node, Parser, Tree};

static STATEMENT_NODE_NAMES: &[&str] = &[
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

pub(crate) fn js_like_get_statement_sorts(lang: &TSLanguage) -> Vec<SortId> {
    STATEMENT_NODE_NAMES
        .iter()
        .map(|kind| lang.id_for_node_kind(kind, true))
        .collect()
}

pub(crate) fn js_skip_snippet_compilation_sorts() -> Vec<(&'static str, &'static str)> {
    vec![
        ("method_definition", "parenthesis"),
        ("function", "parenthesis"),
        ("function_declaration", "parenthesis"),
        ("generator_function", "parenthesis"),
        ("generator_function_declaration", "parenthesis"),
        ("arrow_function", "parenthesis"),
    ]
}

pub(crate) fn js_like_skip_snippet_compilation_sorts() -> Vec<(&'static str, &'static str)> {
    let mut res = vec![
        ("constructor_type", "parenthesis"),
        ("construct_signature", "parenthesis"),
        ("function_type", "parenthesis"),
        ("method_signature", "parenthesis"),
        ("abstract_method_signature", "parenthesis"),
        ("function_signature", "parenthesis"),
    ];
    res.extend(js_skip_snippet_compilation_sorts());
    res
}

pub(crate) fn js_optional_empty_field_compilation() -> Vec<(&'static str, &'static str)> {
    vec![
        ("function", "async"),
        ("arrow_function", "async"),
        ("generator_function", "async"),
        ("generator_function_declaration", "async"),
        ("method_definition", "async"),
        ("function_declaration", "async"),
        ("import_statement", "import"),
    ]
}

pub(crate) fn js_like_optional_empty_field_compilation() -> Vec<(&'static str, &'static str)> {
    let mut res = vec![
        ("call_expression", "type_arguments"),
        ("new_expression", "type_arguments"),
        ("function", "return_type"),
        ("arrow_function", "return_type"),
        ("import_statement", "type"),
        ("public_field_definition", "static"),
    ];
    res.extend(js_optional_empty_field_compilation());
    res
}

pub(crate) fn parse_file(
    lang: &impl Language,
    name: &str,
    body: &str,
    logs: &mut marzano_util::analysis_logs::AnalysisLogs,
    new: bool,
    parser: &mut Parser,
) -> anyhow::Result<Option<Tree>> {
    if name.ends_with(".vue") {
        let js_name_array = ["js", "ts", "tsx", "jsx", "javascript", "typescript"];
        let parent_node_kind = "script_element";
        let ranges = get_vue_ranges(body, parent_node_kind, Some(&js_name_array))?;
        parser.set_included_ranges(&ranges)?;
        parser
            .parse(body, None)?
            .ok_or(anyhow!("missing tree"))
            .map(Some)
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
        for ancestor in NodeWithSource::new(n.clone(), src).ancestors() {
            if ancestor.node.kind() == "class_body" {
                orphan_ranges.push(n.range());
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tsx::Tsx;
    use marzano_util::print_node::print_node;

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
        let js_name_array = ["js", "ts", "tsx", "jsx", "javascript", "typescript"];
        let parent_node_kind = "script_element";
        let ranges = get_vue_ranges(snippet, parent_node_kind, Some(&js_name_array));
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
        .unwrap()
        .unwrap();
        let root = tree.root_node();
        print_node(&root);
    }
}
