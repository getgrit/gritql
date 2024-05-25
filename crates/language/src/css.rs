use crate::{
    language::{
        fields_for_nodes, Field, MarzanoLanguage, MarzanoParser, NodeTypes, SortId, TSLanguage,
        Tree,
    },
    vue::get_vue_ranges,
};
use grit_util::{AnalysisLogs, FileOrigin, Language, Parser, SnippetTree};
use marzano_util::node_with_source::NodeWithSource;
use std::{path::Path, sync::OnceLock};

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

impl NodeTypes for Css {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

impl Language for Css {
    type Node<'a> = NodeWithSource<'a>;

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

    fn is_comment(&self, node: &NodeWithSource) -> bool {
        MarzanoLanguage::is_comment_node(self, node)
    }

    fn is_metavariable(&self, node: &NodeWithSource) -> bool {
        MarzanoLanguage::is_metavariable_node(self, node)
    }

    fn make_single_line_comment(&self, text: &str) -> String {
        format!("/* {} */\n", text)
    }
}

impl<'a> MarzanoLanguage<'a> for Css {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn get_parser(&self) -> Box<dyn Parser<Tree = Tree>> {
        Box::new(MarzanoCssParser::new(self))
    }

    fn is_comment_sort(&self, id: SortId) -> bool {
        id == self.comment_sort
    }

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }
}

pub(crate) struct MarzanoCssParser(MarzanoParser);

impl MarzanoCssParser {
    pub(crate) fn new<'a>(lang: &impl MarzanoLanguage<'a>) -> Self {
        Self(MarzanoParser::new(lang))
    }
}

impl Parser for MarzanoCssParser {
    type Tree = Tree;

    fn parse_file(
        &mut self,
        body: &str,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
        old_tree: FileOrigin<'_, Tree>,
    ) -> Option<Tree> {
        if path
            .and_then(Path::extension)
            .is_some_and(|ext| ext == "vue")
        {
            let parent_node_kind = "style_element";
            let ranges = get_vue_ranges(body, parent_node_kind, None).ok()?;
            if ranges.is_empty() {
                return None;
            }

            self.0.parser.set_included_ranges(&ranges).ok()?;
            self.0
                .parser
                .parse(body, None)
                .ok()?
                .map(|tree| Tree::new(tree, body))
        } else {
            self.0.parse_file(body, path, logs, old_tree)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::nodes_from_indices;
    use grit_util::Ast;
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
        let mut parser = MarzanoCssParser::new(&css);
        let tree = parser
            .parse_file(
                snippet,
                Some(Path::new("test.vue")),
                &mut vec![].into(),
                FileOrigin::Fresh,
            )
            .unwrap();
        print_node(&tree.root_node().node);
    }
}
