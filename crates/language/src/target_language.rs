use crate::language::Replacement;
use crate::language::{GritMetaValue, LeafEquivalenceClass, SnippetTree, TSLanguage};
use crate::{
    csharp::CSharp,
    css::Css,
    go::Go,
    hcl::Hcl,
    html::Html,
    java::Java,
    javascript::JavaScript,
    json::Json,
    language::{Field, FieldId, Language, SortId},
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
use enum_dispatch::enum_dispatch;
use marzano_util::analysis_logs::AnalysisLogs;
use marzano_util::node_with_source::NodeWithSource;
use marzano_util::position::Range;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;
use std::hash::Hash;
use tree_sitter::{Parser, Tree};

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
        match value {
            TargetLanguage::JavaScript(_) => PatternLanguage::JavaScript,
            TargetLanguage::TypeScript(_) => PatternLanguage::TypeScript,
            TargetLanguage::Tsx(_) => PatternLanguage::Tsx,
            TargetLanguage::Html(_) => PatternLanguage::Html,
            TargetLanguage::Css(_) => PatternLanguage::Css,
            TargetLanguage::Json(_) => PatternLanguage::Json,
            TargetLanguage::Java(_) => PatternLanguage::Java,
            TargetLanguage::CSharp(_) => PatternLanguage::CSharp,
            TargetLanguage::Python(_) => PatternLanguage::Python,
            TargetLanguage::MarkdownBlock(_) => PatternLanguage::MarkdownBlock,
            TargetLanguage::MarkdownInline(_) => PatternLanguage::MarkdownInline,
            TargetLanguage::Go(_) => PatternLanguage::Go,
            TargetLanguage::Rust(_) => PatternLanguage::Rust,
            TargetLanguage::Ruby(_) => PatternLanguage::Ruby,
            TargetLanguage::Solidity(_) => PatternLanguage::Solidity,
            TargetLanguage::Hcl(_) => PatternLanguage::Hcl,
            TargetLanguage::Yaml(_) => PatternLanguage::Yaml,
            TargetLanguage::Sql(_) => PatternLanguage::Sql,
            TargetLanguage::Vue(_) => PatternLanguage::Vue,
            TargetLanguage::Toml(_) => PatternLanguage::Toml,
            TargetLanguage::Php(_) => PatternLanguage::Php,
            TargetLanguage::PhpOnly(_) => PatternLanguage::PhpOnly,
        }
    }
}

impl PatternLanguage {
    pub fn is_initialized(&self) -> bool {
        match self {
            PatternLanguage::JavaScript => JavaScript::is_initialized(),
            PatternLanguage::TypeScript => TypeScript::is_initialized(),
            PatternLanguage::Tsx => Tsx::is_initialized(),
            PatternLanguage::Html => Html::is_initialized(),
            PatternLanguage::Css => Css::is_initialized(),
            PatternLanguage::Json => Json::is_initialized(),
            PatternLanguage::Java => Java::is_initialized(),
            PatternLanguage::CSharp => CSharp::is_initialized(),
            PatternLanguage::Python => Python::is_initialized(),
            PatternLanguage::MarkdownBlock => MarkdownBlock::is_initialized(),
            PatternLanguage::MarkdownInline => MarkdownInline::is_initialized(),
            PatternLanguage::Go => Go::is_initialized(),
            PatternLanguage::Rust => Rust::is_initialized(),
            PatternLanguage::Ruby => Ruby::is_initialized(),
            PatternLanguage::Solidity => Solidity::is_initialized(),
            PatternLanguage::Hcl => Hcl::is_initialized(),
            PatternLanguage::Yaml => Yaml::is_initialized(),
            PatternLanguage::Sql => Sql::is_initialized(),
            PatternLanguage::Vue => Vue::is_initialized(),
            PatternLanguage::Toml => Toml::is_initialized(),
            PatternLanguage::Php => Php::is_initialized(),
            PatternLanguage::PhpOnly => PhpOnly::is_initialized(),
            PatternLanguage::Universal => false,
        }
    }

    pub fn from_tree(tree: &Tree, src: &str) -> Option<Self> {
        let root = tree.root_node();
        let langdecl = root.child_by_field_name("language")?;
        let lang = langdecl
            .child_by_field_name("name")?
            .utf8_text(src.as_bytes())
            .ok()?;
        let lang = lang.trim();
        let flavor = langdecl
            .child_by_field_name("flavor")
            .and_then(|f| f.utf8_text(src.as_bytes()).ok());
        Self::from_string(lang, flavor.as_deref())
    }

    #[cfg(not(feature = "builtin-parser"))]
    pub fn get_language_with_parser(_parser: &mut Parser, _body: &str) -> Option<Self> {
        unimplemented!("grit_parser is unavailable when feature flag [builtin-parser] is off.")
    }

    #[cfg(feature = "builtin-parser")]
    pub fn get_language_with_parser(parser: &mut Parser, body: &str) -> Option<Self> {
        parser
            .set_language(&tree_sitter_gritql::language().into())
            .unwrap();
        let tree = parser.parse(body, None).unwrap();
        tree.and_then(|t| Self::from_tree(&t, body))
    }

    pub fn get_language(src: &str) -> Option<Self> {
        let mut parser: Parser = Parser::new().unwrap();
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
            PatternLanguage::Python => &["py"],
            PatternLanguage::MarkdownBlock => &["md", "mdx", "mdoc"],
            PatternLanguage::MarkdownInline => &["md", "mdx", "mdoc"],
            PatternLanguage::Go => &["go"],
            PatternLanguage::Rust => &["rs"],
            PatternLanguage::Ruby => &["rb"],
            PatternLanguage::Solidity => &["sol"],
            PatternLanguage::Hcl => &["hcl", "tf"],
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
            "py" => Some(Self::Python),
            "md" | "mdx" | "mdoc" => Some(Self::MarkdownBlock),
            "go" => Some(Self::Go),
            "rs" => Some(Self::Rust),
            "rb" => Some(Self::Ruby),
            "sol" => Some(Self::Solidity),
            "hcl" | "tf" => Some(Self::Hcl),
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

    // slightly inneficient but ensures the names are cosnsistent
    pub fn language_name(self) -> &'static str {
        self.try_into()
            .map(|l: TargetLanguage| l.language_name())
            .unwrap_or("Universal")
    }

    // todo use strum_crate enum_iter, and disable Universal variant
    pub fn enumerate() -> Vec<PatternLanguage> {
        vec![
            PatternLanguage::JavaScript,
            PatternLanguage::TypeScript,
            PatternLanguage::Tsx,
            PatternLanguage::Html,
            PatternLanguage::Css,
            PatternLanguage::Json,
            PatternLanguage::Java,
            PatternLanguage::CSharp,
            PatternLanguage::Python,
            PatternLanguage::MarkdownBlock,
            PatternLanguage::MarkdownInline,
            PatternLanguage::Go,
            PatternLanguage::Rust,
            PatternLanguage::Ruby,
            PatternLanguage::Solidity,
            PatternLanguage::Hcl,
            PatternLanguage::Yaml,
            PatternLanguage::Sql,
            PatternLanguage::Vue,
            PatternLanguage::Toml,
            PatternLanguage::Php,
            PatternLanguage::PhpOnly,
        ]
    }

    #[cfg(target_arch = "wasm32")]
    pub fn to_target_with_ts_lang(self, lang: TSLanguage) -> Result<TargetLanguage, String> {
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
            PatternLanguage::Universal => Err("Cannot convert universal to TSLang".to_string()),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_target_with_ts_lang(self, _lang: TSLanguage) -> Result<TargetLanguage, String> {
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

#[derive(Debug, Clone)]
#[enum_dispatch]
pub enum TargetLanguage {
    JavaScript(JavaScript),
    TypeScript(TypeScript),
    Tsx(Tsx),
    Html(Html),
    Css(Css),
    Json(Json),
    Java(Java),
    CSharp(CSharp),
    Python(Python),
    MarkdownBlock(MarkdownBlock),
    MarkdownInline(MarkdownInline),
    Go(Go),
    Rust(Rust),
    Ruby(Ruby),
    Solidity(Solidity),
    Hcl(Hcl),
    Yaml(Yaml),
    Vue(Vue),
    Toml(Toml),
    Sql(Sql),
    Php(Php),
    PhpOnly(PhpOnly),
}

// when built to wasm the language must be initialized with a parser at least once
// before it can be created without a parser.
impl TryFrom<PatternLanguage> for TargetLanguage {
    type Error = String;
    fn try_from(l: PatternLanguage) -> Result<TargetLanguage, String> {
        match l {
            PatternLanguage::JavaScript => Ok(TargetLanguage::JavaScript(JavaScript::new(None))),
            PatternLanguage::TypeScript => Ok(TargetLanguage::TypeScript(TypeScript::new(None))),
            PatternLanguage::Tsx => Ok(TargetLanguage::Tsx(Tsx::new(None))),
            PatternLanguage::Html => Ok(TargetLanguage::Html(Html::new(None))),
            PatternLanguage::Css => Ok(TargetLanguage::Css(Css::new(None))),
            PatternLanguage::Json => Ok(TargetLanguage::Json(Json::new(None))),
            PatternLanguage::Java => Ok(TargetLanguage::Java(Java::new(None))),
            PatternLanguage::CSharp => Ok(TargetLanguage::CSharp(CSharp::new(None))),
            PatternLanguage::Python => Ok(TargetLanguage::Python(Python::new(None))),
            PatternLanguage::MarkdownBlock => {
                Ok(TargetLanguage::MarkdownBlock(MarkdownBlock::new(None)))
            }
            PatternLanguage::MarkdownInline => {
                Ok(TargetLanguage::MarkdownInline(MarkdownInline::new(None)))
            }
            PatternLanguage::Go => Ok(TargetLanguage::Go(Go::new(None))),
            PatternLanguage::Rust => Ok(TargetLanguage::Rust(Rust::new(None))),
            PatternLanguage::Ruby => Ok(TargetLanguage::Ruby(Ruby::new(None))),
            PatternLanguage::Solidity => Ok(TargetLanguage::Solidity(Solidity::new(None))),
            PatternLanguage::Hcl => Ok(TargetLanguage::Hcl(Hcl::new(None))),
            PatternLanguage::Yaml => Ok(TargetLanguage::Yaml(Yaml::new(None))),
            PatternLanguage::Sql => Ok(TargetLanguage::Sql(Sql::new(None))),
            PatternLanguage::Vue => Ok(TargetLanguage::Vue(Vue::new(None))),
            PatternLanguage::Toml => Ok(TargetLanguage::Toml(Toml::new(None))),
            PatternLanguage::Php => Ok(TargetLanguage::Php(Php::new(None))),
            PatternLanguage::PhpOnly => Ok(TargetLanguage::PhpOnly(PhpOnly::new(None))),
            PatternLanguage::Universal => {
                Err("cannot instantiate Universal as a target language".to_string())
            }
        }
    }
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
    pub fn from_tree(tree: &Tree, src: &str) -> Option<Self> {
        PatternLanguage::from_tree(tree, src).map(|l| l.try_into().ok())?
    }

    pub fn get_language_with_parser(parser: &mut Parser, body: &str) -> Option<Self> {
        PatternLanguage::get_language_with_parser(parser, body).map(|l| l.try_into().ok())?
    }

    pub fn get_language(src: &str) -> Option<TargetLanguage> {
        PatternLanguage::get_language(src).map(|l| l.try_into().ok())?
    }

    pub fn from_string(name: &str, flavor: Option<&str>) -> Option<Self> {
        PatternLanguage::from_string(name, flavor).map(|l| l.try_into().ok())?
    }

    pub fn to_module_language(&self) -> PatternLanguage {
        match self {
            TargetLanguage::JavaScript(_) => PatternLanguage::JavaScript,
            TargetLanguage::TypeScript(_) => PatternLanguage::TypeScript,
            TargetLanguage::Tsx(_) => PatternLanguage::Tsx,
            TargetLanguage::Html(_) => PatternLanguage::Html,
            TargetLanguage::Css(_) => PatternLanguage::Css,
            TargetLanguage::Json(_) => PatternLanguage::Json,
            TargetLanguage::Java(_) => PatternLanguage::Java,
            TargetLanguage::CSharp(_) => PatternLanguage::CSharp,
            TargetLanguage::Python(_) => PatternLanguage::Python,
            TargetLanguage::MarkdownBlock(_) => PatternLanguage::MarkdownBlock,
            TargetLanguage::MarkdownInline(_) => PatternLanguage::MarkdownInline,
            TargetLanguage::Go(_) => PatternLanguage::Go,
            TargetLanguage::Rust(_) => PatternLanguage::Rust,
            TargetLanguage::Ruby(_) => PatternLanguage::Ruby,
            TargetLanguage::Solidity(_) => PatternLanguage::Solidity,
            TargetLanguage::Hcl(_) => PatternLanguage::Hcl,
            TargetLanguage::Yaml(_) => PatternLanguage::Yaml,
            TargetLanguage::Sql(_) => PatternLanguage::Sql,
            TargetLanguage::Vue(_) => PatternLanguage::Vue,
            TargetLanguage::Toml(_) => PatternLanguage::Toml,
            TargetLanguage::Php(_) => PatternLanguage::Php,
            TargetLanguage::PhpOnly(_) => PatternLanguage::PhpOnly,
        }
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
