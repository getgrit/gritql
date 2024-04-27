use crate::{
    language::{
        FieldExpectationCondition, FieldExpectationCondition::Always,
        FieldExpectationCondition::OnlyIf, MarzanoLanguage, MarzanoParser, SortId, TSLanguage,
        Tree,
    },
    vue::get_vue_ranges,
};
use grit_util::{AnalysisLogs, AstNode, Parser, Replacement, SnippetTree};
use marzano_util::node_with_source::NodeWithSource;
use std::path::Path;

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

pub(crate) fn js_disregarded_field_values(
) -> Vec<(&'static str, &'static str, FieldExpectationCondition)> {
    vec![
        // Always disregarded:
        ("method_definition", "parenthesis", Always),
        ("function", "parenthesis", Always),
        ("function_declaration", "parenthesis", Always),
        ("generator_function", "parenthesis", Always),
        ("generator_function_declaration", "parenthesis", Always),
        ("arrow_function", "parenthesis", Always),
        // Disregarded if empty:
        ("function", "async", OnlyIf(vec![""])),
        ("arrow_function", "async", OnlyIf(vec![""])),
        ("generator_function", "async", OnlyIf(vec![""])),
        ("generator_function_declaration", "async", OnlyIf(vec![""])),
        ("method_definition", "async", OnlyIf(vec![""])),
        ("function_declaration", "async", OnlyIf(vec![""])),
        ("import_statement", "import", OnlyIf(vec![""])),
    ]
}

pub(crate) fn js_like_disregarded_field_values(
) -> Vec<(&'static str, &'static str, FieldExpectationCondition)> {
    let mut res = vec![
        // always disregarded:
        ("constructor_type", "parenthesis", Always),
        ("construct_signature", "parenthesis", Always),
        ("function_type", "parenthesis", Always),
        ("method_signature", "parenthesis", Always),
        ("abstract_method_signature", "parenthesis", Always),
        ("function_signature", "parenthesis", Always),
        // disregarded if empty:
        ("call_expression", "type_arguments", OnlyIf(vec![""])),
        ("new_expression", "type_arguments", OnlyIf(vec![""])),
        ("function", "return_type", OnlyIf(vec![""])),
        ("arrow_function", "return_type", OnlyIf(vec![""])),
        ("import_statement", "type", OnlyIf(vec![""])),
        ("public_field_definition", "static", OnlyIf(vec![""])),
        ("member_expression", "chain", OnlyIf(vec!["", "."])),
    ];
    res.extend(js_disregarded_field_values());
    res
}

pub(crate) struct MarzanoJsLikeParser(MarzanoParser);

impl MarzanoJsLikeParser {
    pub(crate) fn new<'a>(lang: &impl MarzanoLanguage<'a>) -> Self {
        Self(MarzanoParser::new(lang))
    }
}

impl Parser for MarzanoJsLikeParser {
    type Tree = Tree;

    fn parse_file(
        &mut self,
        body: &str,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
        new: bool,
    ) -> Option<Tree> {
        if path
            .and_then(Path::extension)
            .is_some_and(|ext| ext == "vue")
        {
            let js_name_array = ["js", "ts", "tsx", "jsx", "javascript", "typescript"];
            let parent_node_kind = "script_element";
            let ranges = get_vue_ranges(body, parent_node_kind, Some(&js_name_array)).ok()?;

            self.0.parser.set_included_ranges(&ranges).ok()?;
            self.0
                .parser
                .parse(body, None)
                .ok()?
                .map(|tree| Tree::new(tree, body))
        } else {
            self.0.parse_file(body, path, logs, new)
        }
    }

    fn parse_snippet(
        &mut self,
        pre: &'static str,
        source: &str,
        post: &'static str,
    ) -> SnippetTree<Tree> {
        self.0.parse_snippet(pre, source, post)
    }
}

pub(crate) fn js_like_is_comment(
    node: &NodeWithSource,
    comment_sort: SortId,
    jsx_sort: SortId,
) -> bool {
    let id = node.node.kind_id();
    id == comment_sort
        || (id == jsx_sort
            && node.node.named_child_count() == 1
            && node
                .node
                .named_child(0)
                .map(|c| c.kind_id() == comment_sort)
                .is_some_and(|b| b))
}

pub(crate) fn jslike_check_replacements(
    n: NodeWithSource<'_>,
    replacement_ranges: &mut Vec<Replacement>,
) {
    if n.node.kind() == "arrow_function" {
        let child = n.child_by_field_name("body");
        if let Some(child) = child {
            let range = child.range();
            if range.start_byte == range.end_byte {
                replacement_ranges.push(Replacement::new(range, "{}"));
            }
        }
    } else if n.node.is_error()
        && n.text()
            .is_ok_and(|t| ["var", "let", "const"].contains(&t.as_ref()))
        || n.node.kind() == "empty_statement"
    {
        replacement_ranges.push(Replacement::new(n.range(), ""));
    } else if n.node.kind() == "import_statement" {
        for n in n.named_children().filter(|n| n.node.is_error()) {
            if let Some(n) = n.children().last() {
                if n.node.kind() == "," {
                    replacement_ranges.push(Replacement::new(n.range(), ""))
                }
            }
        }
        if let Some(import_clause) = n.child_by_field_name("import") {
            if let Some(imports) = import_clause.child_by_field_name("name") {
                let named_imports = imports.named_children_by_field_name("imports").count();
                let namespace_import = imports.named_children_by_field_name("namespace").count();
                if named_imports == 0 && namespace_import == 0 {
                    replacement_ranges.push(Replacement::new(n.range(), ""));
                }
            }
        }
    } else if n.node.is_error() && n.text().is_ok_and(|n| n == ",") {
        for ancestor in n.ancestors() {
            if ancestor.node.kind() == "class_body" {
                replacement_ranges.push(Replacement::new(n.range(), ""));
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tsx::Tsx;
    use grit_util::Ast;
    use marzano_util::print_node::print_node;
    use std::path::Path;

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
        let mut parser = MarzanoJsLikeParser(MarzanoParser::new(&ts));
        let tree = parser
            .parse_file(
                snippet,
                Some(Path::new("test.vue")),
                &mut vec![].into(),
                false,
            )
            .unwrap();
        print_node(&tree.root_node().node);
    }
}
