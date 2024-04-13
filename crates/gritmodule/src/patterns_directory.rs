use std::{collections::BTreeMap, mem};

use anyhow::{bail, Result};
use marzano_language::target_language::PatternLanguage;

#[derive(Debug)]
pub struct PatternsDirectory {
    pattern_to_language: BTreeMap<String, PatternLanguage>,
    java_script: BTreeMap<String, String>,
    type_script: BTreeMap<String, String>,
    tsx: BTreeMap<String, String>,
    html: BTreeMap<String, String>,
    css: BTreeMap<String, String>,
    json: BTreeMap<String, String>,
    java: BTreeMap<String, String>,
    c_sharp: BTreeMap<String, String>,
    python: BTreeMap<String, String>,
    markdown_block: BTreeMap<String, String>,
    markdown_inline: BTreeMap<String, String>,
    go: BTreeMap<String, String>,
    rust: BTreeMap<String, String>,
    ruby: BTreeMap<String, String>,
    solidity: BTreeMap<String, String>,
    hcl: BTreeMap<String, String>,
    yaml: BTreeMap<String, String>,
    sql: BTreeMap<String, String>,
    vue: BTreeMap<String, String>,
    toml: BTreeMap<String, String>,
    php: BTreeMap<String, String>,
    php_only: BTreeMap<String, String>,    
    universal: BTreeMap<String, String>,
}

pub struct LanguageLibrary {
    lang: PatternLanguage,
    // probably better to return two references instead of merging?
    library: BTreeMap<String, String>,
}

impl LanguageLibrary {
    fn new(lang: PatternLanguage, library: BTreeMap<String, String>) -> Self {
        Self { lang, library }
    }

    pub fn language(&self) -> PatternLanguage {
        self.lang.to_owned()
    }

    pub fn library(&self) -> BTreeMap<String, String> {
        // should really return a reference, but legacy
        self.library.to_owned()
    }
}

impl Default for PatternsDirectory {
    fn default() -> Self {
        Self::new()
    }
}

impl PatternsDirectory {
    pub fn new() -> Self {
        Self {
            pattern_to_language: BTreeMap::new(),
            java_script: BTreeMap::new(),
            type_script: BTreeMap::new(),
            tsx: BTreeMap::new(),
            html: BTreeMap::new(),
            css: BTreeMap::new(),
            json: BTreeMap::new(),
            java: BTreeMap::new(),
            c_sharp: BTreeMap::new(),
            python: BTreeMap::new(),
            markdown_block: BTreeMap::new(),
            markdown_inline: BTreeMap::new(),
            go: BTreeMap::new(),
            rust: BTreeMap::new(),
            ruby: BTreeMap::new(),
            solidity: BTreeMap::new(),
            hcl: BTreeMap::new(),
            yaml: BTreeMap::new(),
            sql: BTreeMap::new(),
            vue: BTreeMap::new(),
            toml: BTreeMap::new(),
            php: BTreeMap::new(),
            php_only: BTreeMap::new(),
            universal: BTreeMap::new(),
        }
    }

    fn get_language_directory_mut(
        &mut self,
        lang: PatternLanguage,
    ) -> &mut BTreeMap<String, String> {
        match lang {
            PatternLanguage::JavaScript => &mut self.java_script,
            PatternLanguage::TypeScript => &mut self.type_script,
            PatternLanguage::Tsx => &mut self.tsx,
            PatternLanguage::Html => &mut self.html,
            PatternLanguage::Css => &mut self.css,
            PatternLanguage::Json => &mut self.json,
            PatternLanguage::Java => &mut self.java,
            PatternLanguage::CSharp => &mut self.c_sharp,
            PatternLanguage::Python => &mut self.python,
            PatternLanguage::MarkdownBlock => &mut self.markdown_block,
            PatternLanguage::MarkdownInline => &mut self.markdown_inline,
            PatternLanguage::Go => &mut self.go,
            PatternLanguage::Rust => &mut self.rust,
            PatternLanguage::Ruby => &mut self.ruby,
            PatternLanguage::Solidity => &mut self.solidity,
            PatternLanguage::Hcl => &mut self.hcl,
            PatternLanguage::Yaml => &mut self.yaml,
            PatternLanguage::Sql => &mut self.sql,
            PatternLanguage::Vue => &mut self.vue,
            PatternLanguage::Toml => &mut self.toml,
            PatternLanguage::Php => &mut self.php,
            PatternLanguage::PhpOnly => &mut self.php_only,
            PatternLanguage::Universal => &mut self.universal,
        }
    }

    pub fn get_language_directory(&self, lang: PatternLanguage) -> &BTreeMap<String, String> {
        match lang {
            PatternLanguage::JavaScript => &self.java_script,
            PatternLanguage::TypeScript => &self.type_script,
            PatternLanguage::Tsx => &self.tsx,
            PatternLanguage::Html => &self.html,
            PatternLanguage::Css => &self.css,
            PatternLanguage::Json => &self.json,
            PatternLanguage::Java => &self.java,
            PatternLanguage::CSharp => &self.c_sharp,
            PatternLanguage::Python => &self.python,
            PatternLanguage::MarkdownBlock => &self.markdown_block,
            PatternLanguage::MarkdownInline => &self.markdown_inline,
            PatternLanguage::Go => &self.go,
            PatternLanguage::Rust => &self.rust,
            PatternLanguage::Ruby => &self.ruby,
            PatternLanguage::Solidity => &self.solidity,
            PatternLanguage::Hcl => &self.hcl,
            PatternLanguage::Yaml => &self.yaml,
            PatternLanguage::Sql => &self.sql,
            PatternLanguage::Vue => &self.vue,
            PatternLanguage::Toml => &self.toml,
            PatternLanguage::Php => &self.php,
            PatternLanguage::PhpOnly => &self.php_only,
            PatternLanguage::Universal => &self.universal,
        }
    }

    fn get_language_and_universal_directory(
        &self,
        language: PatternLanguage,
    ) -> Result<BTreeMap<String, String>> {
        if matches!(language, PatternLanguage::Universal) {
            bail!("cannot directly execute universal pattern")
        };
        let lang_library = self.get_language_directory(language);
        let mut lang_library = lang_library.to_owned();
        let universal = self
            .get_language_directory(PatternLanguage::Universal)
            .to_owned();
        let count = lang_library.len() + universal.len();
        lang_library.extend(universal);
        if count != lang_library.len() {
            bail!("language specific {} library and universal library have patterns with the same name", language.language_name())
        }
        Ok(lang_library)
    }

    pub fn get_language_directory_or_default(
        &self,
        lang: Option<PatternLanguage>,
    ) -> Result<BTreeMap<String, String>> {
        let language = lang.unwrap_or_default();
        self.get_language_and_universal_directory(language)
    }

    fn get_language_directory_from_name(&self, name: &str) -> Option<&BTreeMap<String, String>> {
        self.pattern_to_language
            .get(name)
            .map(|l| self.get_language_directory(*l))
    }

    // imo we should check if name matches [a-z][a-z0-9]*
    // as currently a pattern with no language header and an invalid pattern are
    // both treated as js patterns when the latter should be a not found error
    pub fn get_pattern_libraries(&self, root_pattern: &str) -> Result<LanguageLibrary> {
        let language = self
            .pattern_to_language
            .get(&format!("{}.grit", root_pattern))
            .copied()
            .unwrap_or(PatternLanguage::get_language(root_pattern).unwrap_or_default());
        let library = self.get_language_and_universal_directory(language)?;
        Ok(LanguageLibrary::new(language, library))
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.get_language_directory_from_name(name)
            .and_then(|d| d.get(name))
    }

    // do we want to do an overriding insert?
    // should we return a value depending on update?
    pub fn insert(&mut self, name: String, body: String, language: Option<PatternLanguage>) {
        let language = language.unwrap_or_default();
        self.pattern_to_language
            .entry(name.clone())
            .or_insert(language);
        let hashmap = self.get_language_directory_mut(language);
        hashmap.entry(name).or_insert(body);
    }

    pub fn merge(&mut self, mut other: PatternsDirectory) {
        other
            .pattern_to_language
            .extend(mem::take(&mut self.pattern_to_language));
        self.pattern_to_language = other.pattern_to_language;
        other.java_script.extend(mem::take(&mut self.java_script));
        self.java_script = other.java_script;
        other.type_script.extend(mem::take(&mut self.type_script));
        self.type_script = other.type_script;
        other.tsx.extend(mem::take(&mut self.tsx));
        self.tsx = other.tsx;
        other.html.extend(mem::take(&mut self.html));
        self.html = other.html;
        other.css.extend(mem::take(&mut self.css));
        self.css = other.css;
        other.json.extend(mem::take(&mut self.json));
        self.json = other.json;
        other.java.extend(mem::take(&mut self.java));
        self.java = other.java;
        other.c_sharp.extend(mem::take(&mut self.c_sharp));
        self.c_sharp = other.c_sharp;
        other.python.extend(mem::take(&mut self.python));
        self.python = other.python;
        other
            .markdown_block
            .extend(mem::take(&mut self.markdown_block));
        self.markdown_block = other.markdown_block;
        other
            .markdown_inline
            .extend(mem::take(&mut self.markdown_inline));
        self.markdown_inline = other.markdown_inline;
        other.go.extend(mem::take(&mut self.go));
        self.go = other.go;
        other.rust.extend(mem::take(&mut self.rust));
        self.rust = other.rust;
        other.ruby.extend(mem::take(&mut self.ruby));
        self.ruby = other.ruby;
        other.solidity.extend(mem::take(&mut self.solidity));
        self.solidity = other.solidity;
        other.hcl.extend(mem::take(&mut self.hcl));
        self.hcl = other.hcl;
        other.yaml.extend(mem::take(&mut self.yaml));
        self.yaml = other.yaml;
        other.sql.extend(mem::take(&mut self.sql));
        self.sql = other.sql;
        other.vue.extend(mem::take(&mut self.vue));
        self.vue = other.vue;
        other.toml.extend(mem::take(&mut self.toml));
        self.toml = other.toml;
        other.php.extend(mem::take(&mut self.php));
        self.php = other.php;
        other.php_only.extend(mem::take(&mut self.php_only));
        self.php_only = other.php_only;
        other.universal.extend(mem::take(&mut self.universal));
        self.universal = other.universal;
    }
}
