use anyhow::Context;
use grit_util::{Ast, Position};
use marzano_core::{
    api::{AnalysisLog, InputFile, MatchResult, PatternInfo},
    built_in_functions::BuiltIns,
    pattern_compiler::{CompilationResult},
    tree_sitter_serde::tree_sitter_node_to_json,
};
use marzano_language::{
    grit_parser::MarzanoGritParser,
    language::Tree,
    target_language::{PatternLanguage, TargetLanguage},
};
use marzano_util::rich_path::RichFile;
use marzano_util::runtime::{ExecutionContext, LanguageModelAPI};
use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
    sync::OnceLock,
};
use tree_sitter::{Language as TSLanguage, Parser as TSParser};
use wasm_bindgen::prelude::*;
use marzano_core::pattern_compiler::PatternBuilder;

static GRIT_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static JAVASCRIPT_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static TYPESCRIPT_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static TSX_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static HTML_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static CSS_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static JSON_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static JAVA_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static CSHARP_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static PYTHON_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static MARKDOWN_BLOCK_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static MARKDOWN_INLINE_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static GO_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static RUST_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static RUBY_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static SOLIDITY_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static HCL_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static YAML_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static SQL_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static VUE_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static TOML_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static PHP_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static PHP_ONLY_LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();

#[wasm_bindgen(js_name = initializeTreeSitter)]
pub async fn initialize_tree_sitter() -> Result<(), JsError> {
    web_tree_sitter_sg::TreeSitter::init().await
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    pub(crate) fn gritApiRequest(url: &str, headers: &str, body: &str) -> Result<String, JsValue>;
    #[wasm_bindgen(catch)]
    pub(crate) fn gritExternalFunctionCall(
        code: &str,
        arg_names: Vec<String>,
        arg_values: Vec<String>,
    ) -> Result<String, JsValue>;
}


pub async fn parse_input_files_internal(
    pattern: String,
    paths: Vec<String>,
    contents: Vec<String>,
    lib_paths: Vec<String>,
    lib_contents: Vec<String>,
) -> anyhow::Result<Vec<MatchResult>> {
    console_error_panic_hook::set_once();
    // TODO remove this line once initialize_tree_sitter function works
    let _ = web_tree_sitter_sg::TreeSitter::init().await;
    let mut pure_parser = setup_grit_parser().await?;
    let parser = &mut pure_parser;
    let ParsedPattern { libs, tree, lang } = get_parsed_pattern(&pattern, lib_paths, lib_contents, parser).await?;
    let node = tree.root_node();
    let parsed_pattern = tree_sitter_node_to_json(&node.node, &pattern, &lang).to_string();

    let mut results: Vec<MatchResult> = Vec::new();
    for (path, content) in paths.into_iter().zip(contents) {
        let path = PathBuf::from(path);
        let mut parser = setup_language_parser((&lang).into()).await?;
        let tree = parser.parse(content.as_bytes(), None).unwrap().unwrap();
        let input_file_debug_text =
            tree_sitter_node_to_json(&tree.root_node(), &content, &lang).to_string();
        let input_file = InputFile {
            source_file: path.to_string_lossy().to_string(),
            syntax_tree: input_file_debug_text,
        };
        let res = MatchResult::InputFile(input_file);
        results.push(res);
    }
    #[cfg(not(feature = "ai_builtins"))]
    let injected_builtins: Option<BuiltIns> = None;
    #[cfg(feature = "ai_builtins")]
    let injected_builtins = Some(ai_builtins::ai_builtins::get_ai_built_in_functions());
    let builder = PatternBuilder::start(
        pattern.clone(),
        &libs,
        lang,
        None,
        parser,
        injected_builtins,
    )?;
    match builder.compile(None, None) {
        Ok(c) => {
            let warning_logs = c
                .compilation_warnings
                .iter()
                .map(|w| MatchResult::AnalysisLog(w.clone().into()));

            let pinfo = PatternInfo {
                messages: vec![],
                variables: c.problem.compiled_vars(),
                source_file: pattern,
                parsed_pattern,
                valid: true,
            };
            let pinfo = MatchResult::PatternInfo(pinfo);
            results.push(pinfo);
            results.extend(warning_logs);
        }
        Err(e) => {
            let log = error_to_log(e);
            results.push(log);
        }
    };

    Ok(results)
}

#[wasm_bindgen(js_name = parseInputFiles)]
pub async fn parse_input_files(
    pattern: String,
    // The paths of the target language files to parse.
    paths: Vec<String>,
    // The contents of the target language files to parse, in the same order as `paths`.
    contents: Vec<String>,
    // Library file names, for the language of the pattern.
    lib_paths: Vec<String>,
    // Library file contents, in the same order as `lib_paths`.
    lib_contents: Vec<String>,
) -> Result<JsValue, JsError> {
    let results = match parse_input_files_internal(
        pattern,
        paths,
        contents,
        lib_paths,
        lib_contents,
    ).await {
        Ok(r) => r,
        Err(e) => vec![error_to_log(e)],
    };
    Ok(serde_wasm_bindgen::to_value(&results)?)
}

#[cfg(target_arch = "wasm32")]
async fn match_pattern_internal(
    pattern: String,
    paths: Vec<String>,
    contents: Vec<String>,
    lib_paths: Vec<String>,
    lib_contents: Vec<String>,
    llm_api_base: String,
    // LLM API bearer token
    llm_api_bearer_token: String,
) -> anyhow::Result<Vec<MatchResult>> {
    // TODO remove this line once initialize_tree_sitter function works
    let _ = web_tree_sitter_sg::TreeSitter::init().await;
    let mut pure_parser = setup_grit_parser().await?;
    let parser = &mut pure_parser;
    let ParsedPattern { libs, lang, .. } =
        get_parsed_pattern(&pattern, lib_paths, lib_contents, parser).await?;

    let context = ExecutionContext::new(
        |url, headers, json| {
            let body = serde_json::to_string(json)?;
            let mut header_map = HashMap::<&str, &str>::new();
            for (k, v) in headers.iter() {
                header_map.insert(k.as_str(), v.to_str()?);
            }
            let headers_str = serde_json::to_string(&header_map)?;
            let result = gritApiRequest(url, &headers_str, &body);
            match result {
                Ok(s) => Ok(s),
                Err(_e) => Err(anyhow::anyhow!("HTTP error when making AI request, likely due to a network error. Please make sure you are logged in, or try again later.")),
            }
        },
        |code: &[u8], param_names: Vec<String>, input_bindings: &[&str]| {
            let result = gritExternalFunctionCall(
                &String::from_utf8_lossy(code),
                param_names,
                input_bindings.iter().map(|s| s.to_string()).collect(),
            );
            match result {
                Ok(s) => Ok(s.into()),
                Err(e) => {
                    // TODO: figure out why we don't get the real error here
                    let unwrapped = e
                        .as_string()
                        .unwrap_or_else(|| "unknown error, check console for details".to_string());
                    Err(anyhow::anyhow!(
                        "Error calling external function: {}",
                        unwrapped
                    ))
                }
            }
        },
    );

    let context = if !llm_api_base.is_empty() {
        let llm_api = LanguageModelAPI {
            base_endpoint: llm_api_base,
            bearer_token: llm_api_bearer_token,
            can_cache: true,
        };
        context.with_llm_api(llm_api)
    } else {
        context
    };
    #[cfg(not(feature = "ai_builtins"))]
    let injected_builtins: Option<BuiltIns> = None;
    #[cfg(feature = "ai_builtins")]
    let injected_builtins = Some(ai_builtins::ai_builtins::get_ai_built_in_functions());
    let builder = PatternBuilder::start(
        pattern,
        &libs,
        lang,
        None,
        parser,
        injected_builtins)?;
    let CompilationResult {
        problem: pattern, ..
    } = builder.compile(None, None)?;
    let files: Vec<RichFile> = paths
        .into_iter()
        .zip(contents)
        .map(|(p, c)| RichFile::new(p, c))
        .collect();
    let results = pattern.execute_files(&files, &context);
    Ok(results)
}

fn error_to_log(e: anyhow::Error) -> MatchResult {
    match e.downcast::<grit_util::AnalysisLog>() {
        Ok(al) => MatchResult::AnalysisLog(AnalysisLog::from(al)),
        Err(er) => MatchResult::AnalysisLog(AnalysisLog {
            level: 200,
            message: er.to_string(),
            position: Position::first(),
            file: "PlaygroundPattern".to_string(),
            engine_id: "marzano".to_string(),
            syntax_tree: None,
            range: None,
            source: None,
        }),
    }
}

#[wasm_bindgen(js_name = matchPattern)]
#[cfg(target_arch = "wasm32")]
pub async fn match_pattern(
    pattern: String,
    // The paths of the files to match against.
    paths: Vec<String>,
    // The contents of the files to match against, in the same order as `paths`.
    contents: Vec<String>,
    // Library file names, for the language of the pattern.
    lib_paths: Vec<String>,
    // Library file contents, in the same order as `lib_paths`.
    lib_contents: Vec<String>,
    // LLM API base
    llm_api_base: String,
    // LLM API bearer token
    llm_api_bearer_token: String,
) -> Result<JsValue, JsError> {
    let result = match match_pattern_internal(
        pattern,
        paths,
        contents,
        lib_paths,
        lib_contents,
        llm_api_base,
        llm_api_bearer_token,
    ).await {
        Ok(r) => r,
        Err(e) => vec![error_to_log(e)],
    };
    Ok(serde_wasm_bindgen::to_value(&result)?)
}

struct ParsedPattern {
    libs: BTreeMap<String, String>,
    tree: Tree,
    lang: TargetLanguage,
}

async fn get_parsed_pattern(
    pattern: &str,
    lib_paths: Vec<String>,
    lib_contents: Vec<String>,
    parser: &mut MarzanoGritParser,
) -> anyhow::Result<ParsedPattern> {
    let libs = lib_paths.into_iter().zip(lib_contents).collect();
    let tree = parser.parse_file(pattern, None)?;
    let lang = get_language_for_tree(&tree).await?;
    Ok(ParsedPattern { libs, tree, lang })
}

#[cfg(test)]
fn get_parser_path() -> String {
    format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/wasm_parsers")
}
#[cfg(not(test))]
fn get_parser_path() -> String {
    "/wasm_parsers".to_string()
}

async fn setup_language_parser(lang: PatternLanguage) -> anyhow::Result<TSParser> {
    let mut parser = TSParser::new()?;
    let lang = get_cached_lang(&lang).await?;
    parser.set_language(lang)?;
    Ok(parser)
}

async fn get_cached_lang(lang: &PatternLanguage) -> anyhow::Result<&'static TSLanguage> {
    let lang_store = get_lang_store(lang)?;
    if let Some(lang) = lang_store.get() {
        Ok(lang)
    } else {
        let path = pattern_language_to_path(lang)?;
        let _language_already_set = lang_store.set(get_lang(&path).await?);
        Ok(lang_store.get().ok_or_else(|| anyhow::anyhow!("Failed to get language"))?)
    }
}

async fn setup_grit_parser() -> anyhow::Result<MarzanoGritParser> {
    let mut parser = TSParser::new()?;
    let lang_path = format!("{}{}", get_parser_path(), "/tree-sitter-gritql.wasm");
    let lang = if let Some(lang) = GRIT_LANGUAGE.get() {
        lang
    } else {
        let _language_already_set = GRIT_LANGUAGE.set(get_lang(&lang_path).await?);
        GRIT_LANGUAGE.get().ok_or_else(|| anyhow::anyhow!("Failed to setup GRIT_LANGUAGE"))?
    };
    parser.set_language(lang)?;
    Ok(MarzanoGritParser::from_initialized_ts_parser(parser))
}

async fn get_language_for_tree(tree: &Tree) -> anyhow::Result<TargetLanguage> {
    let lang = PatternLanguage::from_tree(tree).unwrap_or_default();
    if lang.is_initialized() {
        Ok(TargetLanguage::try_from(lang)?)
    } else {
        if matches!(
            lang,
            PatternLanguage::JavaScript
                | PatternLanguage::TypeScript
                | PatternLanguage::Tsx
                | PatternLanguage::Css
        ) {
            // javascript also parses vue files to look for javascript so
            // we need to initialize the Vue struct with a wasm parser
            let vue_lang = get_cached_lang(&PatternLanguage::Vue).await?;
            let _ = PatternLanguage::Vue
                .to_target_with_ts_lang(vue_lang.clone());
        }
        let ts_lang = get_cached_lang(&lang).await?;
        Ok(lang.to_target_with_ts_lang(ts_lang.clone())?)
    }
}

fn pattern_language_to_path(lang: &PatternLanguage) -> anyhow::Result<String> {
    let wasm_file = match lang {
        PatternLanguage::JavaScript => Ok("/tree-sitter-javascript.wasm"),
        PatternLanguage::TypeScript => Ok("/tree-sitter-typescript.wasm"),
        PatternLanguage::Tsx => Ok("/tree-sitter-tsx.wasm"),
        PatternLanguage::Html => Ok("/tree-sitter-html.wasm"),
        PatternLanguage::Css => Ok("/tree-sitter-css.wasm"),
        PatternLanguage::Json => Ok("/tree-sitter-json.wasm"),
        PatternLanguage::Java => Ok("/tree-sitter-java.wasm"),
        PatternLanguage::CSharp => Err(anyhow::anyhow!("CSharp wasm is not currently supported")),
        PatternLanguage::Python => Ok("/tree-sitter-python.wasm"),
        PatternLanguage::MarkdownBlock => Ok("/tree-sitter-markdown-block.wasm"), // def wrong
        PatternLanguage::MarkdownInline => Ok("/tree-sitter-markdown_inline.wasm"), // def wrong
        PatternLanguage::Go => Ok("/tree-sitter-go.wasm"),
        PatternLanguage::Rust => Ok("/tree-sitter-rust.wasm"),
        PatternLanguage::Ruby => Ok("/tree-sitter-ruby.wasm"),
        PatternLanguage::Solidity => Ok("/tree-sitter-solidity.wasm"),
        PatternLanguage::Hcl => Ok("/tree-sitter-hcl.wasm"),
        PatternLanguage::Yaml => Ok("/tree-sitter-yaml.wasm"),
        PatternLanguage::Sql => Ok("/tree-sitter-sql.wasm"),
        PatternLanguage::Vue => Ok("/tree-sitter-vue.wasm"),
        PatternLanguage::Toml => Ok("/tree-sitter-toml.wasm"),
        PatternLanguage::Php => Ok("/tree-sitter-php.wasm"),
        PatternLanguage::PhpOnly => Ok("/tree-sitter-php_only.wasm"),
        PatternLanguage::Universal => Err(anyhow::anyhow!("Universal does not have a parser")),
    }?;
    let final_file = format!("{}{}", get_parser_path(), wasm_file);
    Ok(final_file)
}

#[cfg(target_arch = "wasm32")]
async fn get_lang(parser_path: &str) -> anyhow::Result<TSLanguage> {
    let lang = web_tree_sitter_sg::Language::load_path(parser_path)
        .await
        .map_err(tree_sitter::LanguageError::from)?;
    Ok(TSLanguage::from(lang))
}

#[cfg(not(target_arch = "wasm32"))]
async fn get_lang(_path: &str) -> anyhow::Result<TSLanguage> {
    unreachable!()
}

#[cfg(target_arch = "wasm32")]
fn get_lang_store(language: &PatternLanguage) -> anyhow::Result<&'static OnceLock<TSLanguage>> {
    match language {
        PatternLanguage::JavaScript => Ok(&JAVASCRIPT_LANGUAGE),
        PatternLanguage::TypeScript => Ok(&TYPESCRIPT_LANGUAGE),
        PatternLanguage::Tsx => Ok(&TSX_LANGUAGE),
        PatternLanguage::Html => Ok(&HTML_LANGUAGE),
        PatternLanguage::Css => Ok(&CSS_LANGUAGE),
        PatternLanguage::Json => Ok(&JSON_LANGUAGE),
        PatternLanguage::Java => Ok(&JAVA_LANGUAGE),
        PatternLanguage::CSharp => Ok(&CSHARP_LANGUAGE),
        PatternLanguage::Python => Ok(&PYTHON_LANGUAGE),
        PatternLanguage::MarkdownBlock => Ok(&MARKDOWN_BLOCK_LANGUAGE),
        PatternLanguage::MarkdownInline => Ok(&MARKDOWN_INLINE_LANGUAGE),
        PatternLanguage::Go => Ok(&GO_LANGUAGE),
        PatternLanguage::Rust => Ok(&RUST_LANGUAGE),
        PatternLanguage::Ruby => Ok(&RUBY_LANGUAGE),
        PatternLanguage::Solidity => Ok(&SOLIDITY_LANGUAGE),
        PatternLanguage::Hcl => Ok(&HCL_LANGUAGE),
        PatternLanguage::Yaml => Ok(&YAML_LANGUAGE),
        PatternLanguage::Sql => Ok(&SQL_LANGUAGE),
        PatternLanguage::Vue => Ok(&VUE_LANGUAGE),
        PatternLanguage::Toml => Ok(&TOML_LANGUAGE),
        PatternLanguage::Php => Ok(&PHP_LANGUAGE),
        PatternLanguage::PhpOnly => Ok(&PHP_ONLY_LANGUAGE),
        PatternLanguage::Universal => Err(anyhow::anyhow!("Universal does not have a parser")),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_lang_store(_language: &PatternLanguage) -> anyhow::Result<&'static OnceLock<TSLanguage>> {
    unreachable!()
}

#[cfg(test)]
mod tests {

    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn parse_grit() {
        let _ = web_tree_sitter_sg::TreeSitter::init().await;

        let parser = setup_grit_parser().await;
        assert!(parser.is_ok());
    }
}
