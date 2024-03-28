use std::sync::OnceLock;

use crate::{
    language::{default_parse_file, fields_for_nodes, Field, Language, SortId, TSLanguage},
    vue::get_vue_ranges,
};
use anyhow::anyhow;
use tree_sitter::{Parser, Tree};

static NODE_TYPES_STRING: &str = include_str!("../../../resources/node-types/css-node-types.json");

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
    tree_sitter_css::language().into()
}

#[derive(Debug, Clone)]
pub struct Css {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    comment_sort: SortId,
    language: &'static TSLanguage,
}

impl Css {
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {
        let language = LANGUAGE.get_or_init(|| lang.unwrap_or_else(language));
        let node_types = NODE_TYPES.get_or_init(|| fields_for_nodes(language, NODE_TYPES_STRING));
        let metavariable_sort = language.id_for_node_kind("grit_metavariable", true);
        let comment_sort = language.id_for_node_kind("comment", true);
        Self {
            node_types,
            metavariable_sort,
            comment_sort,
            language,
        }
    }
    pub(crate) fn is_initialized() -> bool {
        LANGUAGE.get().is_some()
    }
}

impl Language for Css {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn language_name(&self) -> &'static str {
        "CSS"
    }
    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[
            ("", ""),
            ("GRIT_BLOCK { ", " }"),
            ("GRIT_BLOCK { GRIT_PROPERTY: ", " }"),
        ]
    }

    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }

    fn is_comment(&self, id: SortId) -> bool {
        id == self.comment_sort
    }

    fn parse_file(
        &self,
        name: &str,
        body: &str,
        logs: &mut marzano_util::analysis_logs::AnalysisLogs,
        new: bool,
    ) -> anyhow::Result<Option<Tree>> {
        let mut parser = Parser::new().unwrap();
        parser.set_language(self.get_ts_language())?;
        if name.ends_with(".vue") {
            let parent_node_kind = "style_element";
            let ranges = get_vue_ranges(body, parent_node_kind, None)?;
            if ranges.is_empty() {
                return Ok(None);
            }
            parser.set_included_ranges(&ranges)?;
            parser
                .parse(body, None)?
                .ok_or(anyhow!("missing tree"))
                .map(Some)
        } else {
            default_parse_file(self.get_ts_language(), name, body, logs, new)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::nodes_from_indices;
    use marzano_util::print_node::print_node;

    #[test]
    fn import_variable() {
        let snippet = r#"var(--red)"#;
        let lang = Css::new(None);
        let mut parser = tree_sitter::Parser::new().unwrap();
        parser.set_language(lang.get_ts_language()).unwrap();
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
        print_node(&nodes[0].node);
    }

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
    showingCount?: boolean;
}>();
</script>

<template>
    <transition name="fade">
        <span v-if="itemCount" class="item-count">
            {{ showingCount }}
        </span>
    </transition>
</template>

<style lang="scss" scoped>
.item-count {
    position: relative;
    display: none;
    margin: 0 8px;
    color: var(--theme--foreground-subdued);
    white-space: nowrap;

    @media (min-width: 600px) {
        display: inline;
    }
}

.fade-enter-active,
.fade-leave-active {
    transition: opacity var(--medium) var(--transition);
}

.fade-enter,
.fade-leave-to {
    opacity: 0;
}
</style>
"#;
        let parent_node_kind = "style_element";
        let ranges = get_vue_ranges(snippet, parent_node_kind, None);
        println!("RANGES: {:#?}", ranges);
        let css = Css::new(None);
        let css_ts = css.get_ts_language();
        let mut parser = Parser::new().unwrap();
        parser.set_language(css_ts).unwrap();
        let tree = css
            .parse_file("test.vue", snippet, &mut vec![].into(), false)
            .unwrap()
            .unwrap();
        let root = tree.root_node();
        print_node(&root);
    }
}
