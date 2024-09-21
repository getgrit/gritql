use crate::{
    csharp::CSharp,
    css::Css,
    go::Go,
    grit_parser::MarzanoGritParser,
    hcl::Hcl,
    html::Html,
    java::Java,
    javascript::JavaScript,
    json::Json,
    language::{
        Field, FieldId, LeafEquivalenceClass, MarzanoLanguage, NodeTypes, SortId, TSLanguage, Tree,
    },
    markdown_block::MarkdownBlock,
    markdown_inline::MarkdownInline,
    php::Php,
    php_only::PhpOnly,
    python::Python,
    ruby::Ruby,
    rust::Rust,
    solidity::Solidity,
    sql::Sql,
    toml::Toml,
    tsx::Tsx,
    typescript::TypeScript,
    vue::Vue,
    yaml::Yaml,
};
use anyhow::Result;
use clap::ValueEnum;
use grit_util::{Ast, AstNode, ByteRange, CodeRange, Language, Parser, SnippetTree};
use marzano_util::node_with_source::NodeWithSource;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::Hash;

#[cfg(feature = "finder")]
use anyhow::Error;
#[cfg(feature = "finder")]
use ignore::{types::TypesBuilder, Walk, WalkBuilder};
#[cfg(feature = "finder")]
use std::path::PathBuf;
#[cfg(feature = "finder")]
use std::str::FromStr;

#[derive(ValueEnum, Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[clap(rename_all = "lower")]
#[serde(rename_all = "lowercase")]
pub enum PatternLanguage {
    #[value(skip)]
    JavaScript,
    #[value(skip)]
    TypeScript,
    #[default]
    #[value(name = "js")]
    #[serde(rename = "js")]
    Tsx,
    Html,
    Css,
    Json,
    Java,
    CSharp,
    Python,
    #[value(name = "markdown")]
    MarkdownBlock,
    #[value(skip)]
    MarkdownInline,
    Go,
    Rust,
    Ruby,
    Solidity,
    Hcl,
    Yaml,
    Sql,
    Vue,
    Toml,
    Php,
    PhpOnly,
    #[value(skip)]
    Universal,
}

impl fmt::Display for PatternLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternLanguage::JavaScript => write!(f, "js"),
            PatternLanguage::TypeScript => write!(f, "js"),
            PatternLanguage::Tsx => write!(f, "js"),
            PatternLanguage::Html => write!(f, "html"),
            PatternLanguage::Css => write!(f, "css"),
            PatternLanguage::Json => write!(f, "json"),
            PatternLanguage::Java => write!(f, "java"),
            PatternLanguage::CSharp => write!(f, "csharp"),
            PatternLanguage::Python => write!(f, "python"),
            PatternLanguage::MarkdownBlock => write!(f, "markdown"),
            PatternLanguage::MarkdownInline => write!(f, "markdown"),
            PatternLanguage::Go => write!(f, "go"),
            PatternLanguage::Rust => write!(f, "rust"),
            PatternLanguage::Ruby => write!(f, "ruby"),
            PatternLanguage::Solidity => write!(f, "solidity"),
            PatternLanguage::Hcl => write!(f, "hcl"),
            PatternLanguage::Yaml => write!(f, "yaml"),
            PatternLanguage::Sql => write!(f, "sql"),
            PatternLanguage::Vue => write!(f, "vue"),
            PatternLanguage::Toml => write!(f, "toml"),
            PatternLanguage::Universal => write!(f, "universal"),
            PatternLanguage::Php => write!(f, "php"),
            PatternLanguage::PhpOnly => write!(f, "php"),
        }
    }
}

impl From<&TargetLanguage> for PatternLanguage {
    fn from(value: &TargetLanguage) -> Self {
        value.to_module_language()
    }
}

impl PatternLanguage {
    pub fn from_tree(tree: &Tree) -> Option<Self> {
        let root = tree.root_node();
        let langdecl = root.child_by_field_name("language")?;
        let lang = langdecl.child_by_field_name("name")?;
        let lang = lang.text().ok()?;
        let lang = lang.trim();
        let flavor = langdecl.child_by_field_name("flavor");
        let flavor = flavor.as_ref().and_then(|f| f.text().ok());
        Self::from_string(lang, flavor.as_deref())
    }

    #[cfg(not(feature = "builtin-parser"))]
    pub fn get_language_with_parser(_parser: &mut MarzanoGritParser, _body: &str) -> Option<Self> {
        unimplemented!("grit_parser is unavailable when feature flag [builtin-parser] is off.")
    }

    #[cfg(feature = "builtin-parser")]
    pub fn get_language_with_parser(parser: &mut MarzanoGritParser, body: &str) -> Option<Self> {
        let tree = parser.parse_file(body, None);
        tree.ok().and_then(|tree| Self::from_tree(&tree))
    }

    pub fn get_language(src: &str) -> Option<Self> {
        let mut parser = MarzanoGritParser::new().unwrap();
        Self::get_language_with_parser(&mut parser, src)
    }

    pub fn from_string(name: &str, flavor: Option<&str>) -> Option<Self> {
        match name {
            "js" => match flavor {
                Some("jsx") => Some(Self::Tsx),
                Some("flow") => Some(Self::Tsx),
                Some("flowComments") => Some(Self::Tsx),
                Some("typescript") => Some(Self::TypeScript),
                Some("js_do_not_use") => Some(Self::JavaScript),
                _ => Some(Self::Tsx),
            },
            "html" => Some(Self::Html),
            "css" => Some(Self::Css),
            "json" => Some(Self::Json),
            "java" => Some(Self::Java),
            "csharp" => Some(Self::CSharp),
            "markdown" => match flavor {
                Some("block") => Some(Self::MarkdownBlock),
                Some("inline") => Some(Self::MarkdownInline),
                _ => Some(Self::MarkdownInline),
            },
            "ipynb" => Some(Self::Python),
            "python" => Some(Self::Python),
            "go" => Some(Self::Go),
            "rust" => Some(Self::Rust),
            "ruby" => Some(Self::Ruby),
            "sol" | "solidity" => Some(Self::Solidity),
            "hcl" => Some(Self::Hcl),
            "yaml" => Some(Self::Yaml),
            "sql" => Some(Self::Sql),
            "vue" => Some(Self::Vue),
            "toml" => Some(Self::Toml),
            "php" => match flavor {
                Some("html") => Some(Self::Php),
                Some("only") => Some(Self::PhpOnly),
                _ => Some(Self::Php),
            },
            "universal" => Some(Self::Universal),
            _ => None,
        }
    }

    fn get_file_extensions(&self) -> &'static [&'static str] {
        match self {
            PatternLanguage::JavaScript => &["js", "jsx", "cjs", "mjs", "vue"],
            PatternLanguage::TypeScript => {
                &["js", "jsx", "ts", "tsx", "cjs", "mjs", "cts", "mts", "vue"]
            }
            PatternLanguage::Tsx => &["js", "jsx", "ts", "tsx", "cjs", "mjs", "cts", "mts", "vue"],
            PatternLanguage::Html => &["html"],
            PatternLanguage::Css => &["css", "vue"],
            PatternLanguage::Json => &["json"],
            PatternLanguage::Java => &["java"],
            PatternLanguage::CSharp => &["cs"],
            PatternLanguage::Python => &["py", "ipynb"],
            PatternLanguage::MarkdownBlock => &["md", "mdx", "mdoc"],
            PatternLanguage::MarkdownInline => &["md", "mdx", "mdoc"],
            PatternLanguage::Go => &["go"],
            PatternLanguage::Rust => &["rs"],
            PatternLanguage::Ruby => &["rb"],
            PatternLanguage::Solidity => &["sol"],
            PatternLanguage::Hcl => &["hcl", "tf", "tfvars"],
            PatternLanguage::Yaml => &["yaml", "yml"],
            PatternLanguage::Sql => &["sql"],
            PatternLanguage::Vue => &["vue"],
            PatternLanguage::Toml => &["toml"],
            PatternLanguage::Php => &["php", "phps", "phar", "phtml", "pht"],
            PatternLanguage::PhpOnly => &["php", "phps", "phar", "phtml", "pht"],
            PatternLanguage::Universal => &[],
        }
    }

    pub fn get_default_extension(&self) -> Option<&'static str> {
        match self {
            PatternLanguage::JavaScript => Some("js"),
            PatternLanguage::TypeScript => Some("ts"),
            PatternLanguage::Tsx => Some("tsx"),
            PatternLanguage::Html => Some("html"),
            PatternLanguage::Css => Some("css"),
            PatternLanguage::Json => Some("json"),
            PatternLanguage::Java => Some("java"),
            PatternLanguage::CSharp => Some("cs"),
            PatternLanguage::Python => Some("py"),
            PatternLanguage::MarkdownBlock => Some("md"),
            PatternLanguage::MarkdownInline => Some("md"),
            PatternLanguage::Go => Some("go"),
            PatternLanguage::Rust => Some("rs"),
            PatternLanguage::Ruby => Some("rb"),
            PatternLanguage::Solidity => Some("sol"),
            PatternLanguage::Hcl => Some("tf"),
            PatternLanguage::Yaml => Some("yaml"),
            PatternLanguage::Sql => Some("sql"),
            PatternLanguage::Vue => Some("vue"),
            PatternLanguage::Toml => Some("toml"),
            PatternLanguage::Php => Some("php"),
            PatternLanguage::PhpOnly => Some("php"),
            PatternLanguage::Universal => None,
        }
    }

    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "js" | "jsx" | "cjs" | "mjs" => Some(Self::Tsx),
            "ts" | "tsx" | "cts" | "mts" => Some(Self::Tsx),
            "html" => Some(Self::Html),
            "css" => Some(Self::Css),
            "json" => Some(Self::Json),
            "java" => Some(Self::Java),
            "cs" => Some(Self::CSharp),
            "ipynb" => Some(Self::Python),
            "py" => Some(Self::Python),
            "md" | "mdx" | "mdoc" => Some(Self::MarkdownBlock),
            "go" => Some(Self::Go),
            "rs" => Some(Self::Rust),
            "rb" => Some(Self::Ruby),
            "sol" => Some(Self::Solidity),
            "hcl" | "tf" | "tfvars" => Some(Self::Hcl),
            "yaml" | "yml" => Some(Self::Yaml),
            "sql" => Some(Self::Sql),
            "vue" => Some(Self::Vue),
            "php" | "phps" | "phtml" | "pht" => Some(Self::Php),
            _ => None,
        }
    }

    pub fn match_extension(&self, ext: &str) -> bool {
        self.get_file_extensions().contains(&ext)
    }

    // slightly inefficient but ensures the names are consistent
    pub fn language_name(self) -> &'static str {
        self.try_into()
            .map(|l: TargetLanguage| l.language_name())
            .unwrap_or("Universal")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn to_target_with_ts_lang(self, lang: TSLanguage) -> Result<TargetLanguage> {
        match self {
            PatternLanguage::JavaScript => {
                Ok(TargetLanguage::JavaScript(JavaScript::new(Some(lang))))
            }
            PatternLanguage::TypeScript => {
                Ok(TargetLanguage::TypeScript(TypeScript::new(Some(lang))))
            }
            PatternLanguage::Tsx => Ok(TargetLanguage::Tsx(Tsx::new(Some(lang)))),
            PatternLanguage::Html => Ok(TargetLanguage::Html(Html::new(Some(lang)))),
            PatternLanguage::Css => Ok(TargetLanguage::Css(Css::new(Some(lang)))),
            PatternLanguage::Json => Ok(TargetLanguage::Json(Json::new(Some(lang)))),
            PatternLanguage::Java => Ok(TargetLanguage::Java(Java::new(Some(lang)))),
            PatternLanguage::CSharp => Ok(TargetLanguage::CSharp(CSharp::new(Some(lang)))),
            PatternLanguage::Python => Ok(TargetLanguage::Python(Python::new(Some(lang)))),
            PatternLanguage::MarkdownBlock => Ok(TargetLanguage::MarkdownBlock(
                MarkdownBlock::new(Some(lang)),
            )),
            PatternLanguage::MarkdownInline => Ok(TargetLanguage::MarkdownInline(
                MarkdownInline::new(Some(lang)),
            )),
            PatternLanguage::Go => Ok(TargetLanguage::Go(Go::new(Some(lang)))),
            PatternLanguage::Rust => Ok(TargetLanguage::Rust(Rust::new(Some(lang)))),
            PatternLanguage::Ruby => Ok(TargetLanguage::Ruby(Ruby::new(Some(lang)))),
            PatternLanguage::Solidity => Ok(TargetLanguage::Solidity(Solidity::new(Some(lang)))),
            PatternLanguage::Hcl => Ok(TargetLanguage::Hcl(Hcl::new(Some(lang)))),
            PatternLanguage::Yaml => Ok(TargetLanguage::Yaml(Yaml::new(Some(lang)))),
            PatternLanguage::Sql => Ok(TargetLanguage::Sql(Sql::new(Some(lang)))),
            PatternLanguage::Vue => Ok(TargetLanguage::Vue(Vue::new(Some(lang)))),
            PatternLanguage::Toml => Ok(TargetLanguage::Toml(Toml::new(Some(lang)))),
            PatternLanguage::Php => Ok(TargetLanguage::Php(Php::new(Some(lang)))),
            PatternLanguage::PhpOnly => Ok(TargetLanguage::PhpOnly(PhpOnly::new(Some(lang)))),
            PatternLanguage::Universal => Err(anyhow::anyhow!(
                "Cannot convert universal to TSLang".to_string()
            )),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_target_with_ts_lang(self, _lang: TSLanguage) -> Result<TargetLanguage> {
        unreachable!()
    }
}

#[cfg(feature = "finder")]
pub fn expand_paths(
    start_paths: &[PathBuf],
    target_languages: Option<&[PatternLanguage]>,
) -> Result<Walk, Error> {
    use ignore::overrides::OverrideBuilder;

    let mut file_types = TypesBuilder::new();
    file_types.add_defaults();
    file_types.add("vue", "*.vue").unwrap();
    match target_languages {
        Some(languages) => {
            for &target_language in languages {
                match target_language {
                    PatternLanguage::Yaml => {
                        // This covers both .yaml and .yml
                        file_types.select("yaml");
                    }
                    PatternLanguage::Universal => {}
                    _ => {
                        for ext in target_language.get_file_extensions() {
                            file_types.add(ext, &format!("*.{}", ext)).unwrap();
                            file_types.select(ext);
                        }
                    }
                }
            }
        }
        None => {
            file_types.select("ts");
            file_types.select("js");
        }
    }

    let mut file_walker = WalkBuilder::new(start_paths[0].clone());
    file_walker.types(file_types.build()?);
    for path in start_paths.iter().skip(1) {
        file_walker.add(path);
    }
    file_walker.add_custom_ignore_filename(PathBuf::from_str(".gritignore")?);

    let grit = OverrideBuilder::new(".").add("!**/.grit/**")?.build()?;
    file_walker.overrides(grit);

    let final_walker = file_walker.standard_filters(true).hidden(false).build();
    Ok(final_walker)
}

// We used to use `enum_dispatch` for this, but it didn't handle the fact
// `TargetLanguage` needs to dispatch to two different traits. It didn't like
// the lifetime argument on `MarzanoLanguage` either.
//
// It's a bit of a pita, since we need to add all methods to dispatch here. But
// on the upside, we can now generate methods we used to implement manually.
// For convenience, we even generate a few `PatternLanguage` methods now.
macro_rules! generate_target_language {
    ($($language:ident),+) => {
        #[derive(Debug, Clone, Copy)]
        pub enum TargetLanguage {
            $($language($language)),+
        }

        impl Language for TargetLanguage {
            type Node<'a> = NodeWithSource<'a>;

            fn language_name(&self) -> &'static str {
                match self {
                    $(Self::$language(lang) => Language::language_name(lang)),+
                }
            }

            fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
                match self {
                    $(Self::$language(lang) => Language::snippet_context_strings(lang)),+
                }
            }

            fn metavariable_prefix(&self) -> &'static str {
                match self {
                    $(Self::$language(lang) => Language::metavariable_prefix(lang)),+
                }
            }

            fn comment_prefix(&self) -> &'static str {
                match self {
                    $(Self::$language(lang) => Language::comment_prefix(lang)),+
                }
            }

            fn metavariable_prefix_substitute(&self) -> &'static str {
                match self {
                    $(Self::$language(lang) => Language::metavariable_prefix_substitute(lang)),+
                }
            }

            fn metavariable_regex(&self) -> &'static Regex {
                match self {
                    $(Self::$language(lang) => Language::metavariable_regex(lang)),+
                }
            }

            fn replaced_metavariable_regex(&self) -> &'static Regex {
                match self {
                    $(Self::$language(lang) => Language::replaced_metavariable_regex(lang)),+
                }
            }

            fn metavariable_bracket_regex(&self) -> &'static Regex {
                match self {
                    $(Self::$language(lang) => Language::metavariable_bracket_regex(lang)),+
                }
            }

            fn exact_variable_regex(&self) -> &'static Regex {
                match self {
                    $(Self::$language(lang) => Language::exact_variable_regex(lang)),+
                }
            }

            fn exact_replaced_variable_regex(&self) -> &'static Regex {
                match self {
                    $(Self::$language(lang) => Language::exact_replaced_variable_regex(lang)),+
                }
            }

            fn is_comment(&self, node: &Self::Node<'_>) -> bool {
                match self {
                    $(Self::$language(lang) => Language::is_comment(lang, node)),+
                }
            }

            fn is_metavariable(&self, node: &Self::Node<'_>) -> bool {
                match self {
                    $(Self::$language(lang) => Language::is_metavariable(lang, node)),+
                }
            }

            fn is_statement(&self, node: &Self::Node<'_>) -> bool {
                match self {
                    $(Self::$language(lang) => Language::is_statement(lang, node)),+
                }
            }

            fn comment_text_range(&self, node: &Self::Node<'_>) -> Option<ByteRange> {
                match self {
                    $(Self::$language(lang) => Language::comment_text_range(lang, node)),+
                }
            }

            fn substitute_metavariable_prefix(&self, src: &str) -> String {
                match self {
                    $(Self::$language(lang) => Language::substitute_metavariable_prefix(lang, src)),+
                }
            }

            fn snippet_metavariable_to_grit_metavariable(&self, src: &str) -> Option<grit_util::GritMetaValue> {
                match self {
                    $(Self::$language(lang) => Language::snippet_metavariable_to_grit_metavariable(lang, src)),+
                }
            }

            fn check_replacements(&self, node: Self::Node<'_>, replacements: &mut Vec<grit_util::Replacement>) {
                match self {
                    $(Self::$language(lang) => Language::check_replacements(lang, node, replacements)),+
                }
            }

            fn take_padding(&self, current: char, next: Option<char>) -> Option<char> {
                match self {
                    $(Self::$language(lang) => Language::take_padding(lang, current, next)),+
                }
            }

            fn align_padding<'a>(
                &self,
                node: &Self::Node<'a>,
                range: &CodeRange,
                skip_ranges: &[CodeRange],
                new_padding: Option<usize>,
                offset: usize,
                substitutions: &mut [(grit_util::EffectRange, String)],
            ) -> std::borrow::Cow<'a, str> {
                match self {
                    $(Self::$language(lang) => Language::align_padding(
                        lang,
                        node,
                        range,
                        skip_ranges,
                        new_padding,
                        offset,
                        substitutions
                    )),+
                }
            }

            fn pad_snippet<'a>(&self, snippet: &'a str, padding: &str) -> std::borrow::Cow<'a, str> {
                match self {
                    $(Self::$language(lang) => Language::pad_snippet(lang, snippet, padding)),+
                }
            }

            fn get_skip_padding_ranges(&self, node: &Self::Node<'_>) -> Vec<grit_util::CodeRange> {
                match self {
                    $(Self::$language(lang) => Language::get_skip_padding_ranges(lang, node)),+
                }
            }

            fn should_pad_snippet(&self) -> bool {
                match self {
                    $(Self::$language(lang) => Language::should_pad_snippet(lang)),+
                }
            }

            fn make_single_line_comment(&self, text: &str) -> String {
                match self {
                    $(Self::$language(lang) => Language::make_single_line_comment(lang, text)),+
                }
            }
        }

        impl NodeTypes for TargetLanguage {
            fn node_types(&self) -> &[Vec<Field>] {
                match self {
                    $(Self::$language(lang) => NodeTypes::node_types(lang)),+
                }
            }
        }

        impl<'a> MarzanoLanguage<'a> for TargetLanguage {
            fn get_ts_language(&self) -> &TSLanguage {
                match self {
                    $(Self::$language(lang) => MarzanoLanguage::get_ts_language(lang)),+
                }
            }

            fn get_parser(&self) -> Box<dyn Parser<Tree = Tree>> {
                match self {
                    $(Self::$language(lang) => MarzanoLanguage::get_parser(lang)),+
                }
            }

            fn parse_snippet_contexts(&self, source: &str) -> Vec<SnippetTree<Tree>> {
                match self {
                    $(Self::$language(lang) => MarzanoLanguage::parse_snippet_contexts(lang, source)),+
                }
            }

            fn is_disregarded_snippet_field(&self, sort_id: SortId, field_id: FieldId, field_value: &Option<NodeWithSource<'_>>) -> bool {
                match self {
                    $(Self::$language(lang) => MarzanoLanguage::is_disregarded_snippet_field(lang, sort_id, field_id, field_value)),+
                }
            }

            fn is_comment_sort(&self, id: SortId) -> bool {
                match self {
                    $(Self::$language(lang) => MarzanoLanguage::is_comment_sort(lang, id)),+
                }
            }

            fn is_comment_node(&self, node: &NodeWithSource<'_>) -> bool {
                match self {
                    $(Self::$language(lang) => MarzanoLanguage::is_comment_node(lang, node)),+
                }
            }

            fn metavariable_sort(&self) -> SortId {
                match self {
                    $(Self::$language(lang) => MarzanoLanguage::metavariable_sort(lang)),+
                }
            }

            fn get_equivalence_class(
                &self,
                sort: SortId,
                text: &str,
            ) -> Result<Option<LeafEquivalenceClass>, String> {
                match self {
                    $(Self::$language(lang) => MarzanoLanguage::get_equivalence_class(lang, sort, text)),+
                }
            }
        }

        // when built to wasm the language must be initialized with a parser at least once
        // before it can be created without a parser.
        impl TryFrom<PatternLanguage> for TargetLanguage {
            type Error = anyhow::Error;
            fn try_from(lang: PatternLanguage) -> Result<Self> {
                match lang {
                    $(PatternLanguage::$language => Ok(Self::$language($language::new(None)))),+,
                    PatternLanguage::Universal => Err(
                        anyhow::anyhow!("cannot instantiate Universal as a target language".to_string())
                    )
                }
            }
        }

        impl PatternLanguage {
            pub fn enumerate() -> Vec<Self> {
                vec![$(Self::$language),+]
            }

            pub fn is_initialized(&self) -> bool {
                match self {
                    $(Self::$language => $language::is_initialized()),+,
                    Self::Universal => false,
                }
            }
        }

        impl TargetLanguage {
            pub fn to_module_language(&self) -> PatternLanguage {
                match self {
                    $(Self::$language(_) => PatternLanguage::$language),+
                }
            }
        }
    };
}

generate_target_language! {
    JavaScript,
    TypeScript,
    Tsx,
    Html,
    Css,
    Json,
    Java,
    CSharp,
    Python,
    MarkdownBlock,
    MarkdownInline,
    Go,
    Rust,
    Ruby,
    Solidity,
    Hcl,
    Yaml,
    Vue,
    Toml,
    Sql,
    Php,
    PhpOnly
}

impl fmt::Display for TargetLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetLanguage::JavaScript(_) => write!(f, "js"),
            TargetLanguage::TypeScript(_) => write!(f, "js"),
            TargetLanguage::Tsx(_) => write!(f, "js"),
            TargetLanguage::Html(_) => write!(f, "html"),
            TargetLanguage::Css(_) => write!(f, "css"),
            TargetLanguage::Json(_) => write!(f, "json"),
            TargetLanguage::Java(_) => write!(f, "java"),
            TargetLanguage::CSharp(_) => write!(f, "csharp"),
            TargetLanguage::Python(_) => write!(f, "python"),
            TargetLanguage::MarkdownBlock(_) => write!(f, "markdown"),
            TargetLanguage::MarkdownInline(_) => write!(f, "markdown"),
            TargetLanguage::Go(_) => write!(f, "go"),
            TargetLanguage::Rust(_) => write!(f, "rust"),
            TargetLanguage::Ruby(_) => write!(f, "ruby"),
            TargetLanguage::Solidity(_) => write!(f, "solidity"),
            TargetLanguage::Hcl(_) => write!(f, "hcl"),
            TargetLanguage::Yaml(_) => write!(f, "yaml"),
            TargetLanguage::Sql(_) => write!(f, "sql"),
            TargetLanguage::Vue(_) => write!(f, "vue"),
            TargetLanguage::Toml(_) => write!(f, "toml"),
            TargetLanguage::Php(_) => write!(f, "php"),
            TargetLanguage::PhpOnly(_) => write!(f, "php"),
        }
    }
}

impl TargetLanguage {
    pub fn from_tree(tree: &Tree) -> Option<Self> {
        PatternLanguage::from_tree(tree).map(|l| l.try_into().ok())?
    }

    pub fn get_language(src: &str) -> Option<TargetLanguage> {
        PatternLanguage::get_language(src).map(|l| l.try_into().ok())?
    }

    pub fn from_string(name: &str, flavor: Option<&str>) -> Option<Self> {
        PatternLanguage::from_string(name, flavor).map(|l| l.try_into().ok())?
    }

    pub fn get_default_extension(&self) -> &'static str {
        self.to_module_language().get_default_extension().unwrap()
    }

    pub fn from_extension(extension: &str) -> Option<Self> {
        PatternLanguage::from_extension(extension).map(|l| l.try_into().unwrap())
    }

    pub fn match_extension(&self, ext: &str) -> bool {
        self.to_module_language().match_extension(ext)
    }

    pub fn extract_single_line_comment(&self, text: &str) -> Option<String> {
        let re = match self {
            TargetLanguage::CSharp(_)
            | TargetLanguage::Go(_)
            | TargetLanguage::Java(_)
            | TargetLanguage::JavaScript(_)
            | TargetLanguage::Json(_)
            | TargetLanguage::Rust(_)
            | TargetLanguage::Solidity(_)
            | TargetLanguage::Tsx(_)
            | TargetLanguage::Php(_)
            | TargetLanguage::PhpOnly(_)
            | TargetLanguage::TypeScript(_) => Regex::new(r"//\s*(.*)").unwrap(),
            TargetLanguage::Python(_)
            | TargetLanguage::Ruby(_)
            | TargetLanguage::Toml(_)
            | TargetLanguage::Yaml(_) => Regex::new(r"#\s*(.*)").unwrap(),
            TargetLanguage::Hcl(_) => Regex::new(r"(#|//)\s*(.*)").unwrap(),
            TargetLanguage::Html(_)
            | TargetLanguage::Vue(_)
            | TargetLanguage::MarkdownBlock(_)
            | TargetLanguage::MarkdownInline(_) => Regex::new(r"<!--\s*(.*?)\s*-->").unwrap(),
            TargetLanguage::Css(_) => Regex::new(r"/\*\s*(.*?)\s*\*/").unwrap(),
            TargetLanguage::Sql(_) => Regex::new(r"--\s*(.*)").unwrap(),
        };
        let comment = re
            .captures(text)
            .and_then(|c| {
                c.get(if matches!(self, TargetLanguage::Hcl(_)) {
                    2
                } else {
                    1
                })
            })
            .map(|m| m.as_str().to_string());
        comment
    }
}

impl Default for TargetLanguage {
    fn default() -> Self {
        TargetLanguage::JavaScript(JavaScript::new(None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_javascript_comment() {
        let text = "// this is a comment\nconsole.log('hello')";
        let lang = TargetLanguage::JavaScript(JavaScript::new(None));
        let comment = lang.extract_single_line_comment(text).unwrap();
        assert_eq!(comment, "this is a comment");
    }

    #[test]
    fn extract_python_comment() {
        let text = "# this is a comment\nprint('hello')";
        let lang = TargetLanguage::Python(Python::new(None));
        let comment = lang.extract_single_line_comment(text).unwrap();
        assert_eq!(comment, "this is a comment");
    }

    #[test]
    fn extract_html_comment() {
        let text = "<!-- this is a comment -->\n<p>hello</p>";
        let lang = TargetLanguage::Html(Html::new(None));
        let comment = lang.extract_single_line_comment(text).unwrap();
        assert_eq!(comment, "this is a comment");
    }

    #[test]
    fn extract_css_comment() {
        let text = "/* this is a comment */\np { color: red; }";
        let lang = TargetLanguage::Css(Css::new(None));
        let comment = lang.extract_single_line_comment(text).unwrap();
        assert_eq!(comment, "this is a comment");
    }

    #[test]
    fn extract_sql_comment() {
        let text = "-- this is a comment\nSELECT * FROM table";
        let lang = TargetLanguage::Sql(Sql::new(None));
        let comment = lang.extract_single_line_comment(text).unwrap();
        assert_eq!(comment, "this is a comment");
    }

    #[test]
    fn extract_hcl_comment() {
        let text = "# this is a comment\nvariable \"name\" {}";
        let lang = TargetLanguage::Hcl(Hcl::new(None));
        let comment = lang.extract_single_line_comment(text).unwrap();
        assert_eq!(comment, "this is a comment");
        let other_text = "// this is a comment\nvariable \"name\" {}";
        let other_comment = lang.extract_single_line_comment(other_text).unwrap();
        assert_eq!(other_comment, "this is a comment");
    }
}
