macro_rules! use_marzano_base_delegate {
    () => {
        type Node<'a> = NodeWithSource<'a>;

        fn align_padding<'a>(
            &self,
            node: &Self::Node<'a>,
            range: &grit_util::CodeRange,
            skip_ranges: &[grit_util::CodeRange],
            new_padding: Option<usize>,
            offset: usize,
            substitutions: &mut [(grit_util::EffectRange, String)],
        ) -> std::borrow::Cow<'a, str> {
            MarzanoLanguage::align_padding(
                self,
                node,
                range,
                skip_ranges,
                new_padding,
                offset,
                substitutions,
            )
        }

        fn pad_snippet<'a>(&self, snippet: &'a str, padding: &str) -> std::borrow::Cow<'a, str> {
            MarzanoLanguage::pad_snippet(self, snippet, padding)
        }

        fn get_skip_padding_ranges(&self, node: &Self::Node<'_>) -> Vec<grit_util::CodeRange> {
            MarzanoLanguage::get_skip_padding_ranges(self, node)
        }

        fn is_comment(&self, node: &NodeWithSource) -> bool {
            MarzanoLanguage::is_comment_node(self, node)
        }
    };
}

macro_rules! use_marzano_js_like_delegate {
    () => {
        use_marzano_base_delegate!();

        fn check_replacements(&self, n: NodeWithSource<'_>, replacements: &mut Vec<Replacement>) {
            jslike_check_replacements(n, replacements)
        }
    };
}

macro_rules! use_marzano_delegate {
    () => {
        use_marzano_base_delegate!();

        fn is_metavariable(&self, node: &NodeWithSource) -> bool {
            MarzanoLanguage::is_metavariable_node(self, node)
        }
    };
}

pub mod cpp;
pub mod csharp;
pub mod css;
pub mod foreign_language;
pub mod go;
pub mod grit_parser;
pub mod grit_ts_node;
pub mod hcl;
pub mod html;
pub mod java;
pub mod javascript;
mod js_like;
pub mod json;
pub mod language;
pub mod markdown_block;
pub mod markdown_inline;
mod notebooks;
pub mod php;
mod php_like;
pub mod php_only;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod solidity;
pub mod sourcemap;
pub mod sql;
pub mod target_language;
pub mod toml;
pub mod tsx;
pub mod typescript;
pub mod vue;
pub mod yaml;
