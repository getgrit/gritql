use crate::pattern_compiler::src_to_problem_libs;
use anyhow::{anyhow, Context, Result};
use api::MatchResult;
use grit_util::{Range, VariableMatch};
use insta::{assert_debug_snapshot, assert_snapshot, assert_yaml_snapshot};
use lazy_static::lazy_static;
use marzano_auth::env::ENV_VAR_GRIT_API_URL;
use marzano_auth::testing::get_testing_auth_info;
use marzano_language::language::MarzanoLanguage;
use marzano_language::target_language::{PatternLanguage, TargetLanguage};
use marzano_util::rich_path::RichFile;
use marzano_util::runtime::{ExecutionContext, LanguageModelAPI};
use problem::Problem;
use similar::{ChangeTag, TextDiff};
use std::collections::{BTreeMap, HashMap};
use std::{env, path::Path, path::PathBuf};
use tree_sitter::Parser as TSParser;
use trim_margin::MarginTrimmable;
use walkdir::WalkDir;

use super::*;

pub fn src_to_problem(src: String, default_lang: TargetLanguage) -> Result<Problem> {
    let libs = BTreeMap::new();
    src_to_problem_libs(src, &libs, default_lang, None, None, None, None).map(|cr| cr.problem)
}

// #[deprecated(note = "remove after migrating tests to MatchResult")]
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub input_file_debug_text: String,
    pub the_match: Option<ExecutionMatch>,
}

impl ExecutionResult {
    pub fn is_some(&self) -> bool {
        self.the_match.is_some()
    }
    pub fn is_none(&self) -> bool {
        self.the_match.is_none()
    }
}

// todo: remove after migrating tests
// #[deprecated(note = "remove after migrating tests to MatchResult")]
#[derive(Debug, Clone)]
pub struct ExecutionMatch {
    pub ranges: Vec<Range>,
    pub variables: Vec<VariableMatch>,
    pub rewrite: Option<String>,
    pub filename: String,
    pub new_files: HashMap<String, String>,
}

// #[deprecated(note = "remove after migrating tests to MatchResult")]
fn match_pattern_one_file(
    pattern: String,
    file: &str,
    src: &str,
    default_language: TargetLanguage,
) -> Result<ExecutionResult> {
    let libs = BTreeMap::new();
    match_pattern_libs(pattern, &libs, file, src, default_language)
}

fn create_test_context() -> Result<ExecutionContext> {
    let context = ExecutionContext::default();

    // Exchange client tokens for a test token
    let auth = get_testing_auth_info()?;

    let api = env::var(ENV_VAR_GRIT_API_URL)
        .with_context(|| format!("{} env var not set", ENV_VAR_GRIT_API_URL))?;

    let api = LanguageModelAPI {
        base_endpoint: api,
        bearer_token: auth.access_token,
        can_cache: true,
    };

    Ok(context.with_llm_api(api))
}

lazy_static! {
    static ref TEST_EXECUTION_CONTEXT: Result<ExecutionContext> = create_test_context();
}

#[allow(clippy::wildcard_enum_match_arm)]
fn match_pattern_libs(
    pattern: String,
    libs: &BTreeMap<String, String>,
    file: &str,
    src: &str,
    default_language: TargetLanguage,
) -> Result<ExecutionResult> {
    let default_context = ExecutionContext::default();
    let context = TEST_EXECUTION_CONTEXT.as_ref().unwrap_or(&default_context);

    let pattern =
        src_to_problem_libs(pattern, libs, default_language, None, None, None, None)?.problem;
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), src.to_owned()), context);
    let mut execution_result = ExecutionResult {
        input_file_debug_text: "".to_string(),
        the_match: None,
    };
    for result in results.clone() {
        match result {
            MatchResult::InputFile(input) => {
                execution_result.input_file_debug_text = input.syntax_tree;
            }
            MatchResult::Match(m) => {
                execution_result.the_match = Some(ExecutionMatch {
                    ranges: m.ranges,
                    variables: m.variables,
                    rewrite: None,
                    filename: m.source_file,
                    new_files: HashMap::new(),
                });
            }
            MatchResult::Rewrite(r) => {
                execution_result.the_match = Some(ExecutionMatch {
                    ranges: r.original.ranges,
                    variables: r.original.variables,
                    rewrite: Some(r.rewritten.content),
                    filename: r.rewritten.source_file,
                    new_files: HashMap::new(),
                })
            }
            _ => {}
        }
    }

    for result in results {
        if let MatchResult::CreateFile(f) = result {
            execution_result
                .the_match
                .as_mut()
                .unwrap()
                .new_files
                .insert(f.rewritten.source_file, f.rewritten.content);
        }
    }
    Ok(execution_result)
}

struct TestArg {
    pattern: String,
    source: String,
}

struct TestArgExpected {
    pattern: String,
    source: String,
    expected: String,
}

struct TestArgExpectedWithLibs {
    pattern: String,
    lib_patterns: Vec<String>,
    source: String,
    expected: String,
}

struct TestArgExpectedWithNewFile {
    pattern: String,
    source: String,
    expected: String,
    new_file_name: String,
    new_file_body: String,
}

fn get_fixtures_root() -> Result<PathBuf> {
    let parent_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let actual_path = PathBuf::from(parent_dir).join("fixtures");

    Ok(actual_path)
}

fn test_rewrite(dir: &str, pattern: &str, test: &str) -> Result<()> {
    let (result, expected) = test_setup(dir, pattern, test)?;
    assert!(result.is_some(), "PATTERN failed to match");
    let result = result.the_match.unwrap();
    let expected = expected.ok_or_else(|| anyhow!("expected result not found"))?;
    assert!(result.rewrite.is_some(), "rewrite not found");
    let rewrite = result.rewrite.unwrap();
    if rewrite.trim() != expected.trim() {
        let diff = TextDiff::from_lines(expected.trim(), rewrite.trim());
        let mut diff_str = String::new();
        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            if !matches!(change.tag(), ChangeTag::Equal) {
                diff_str.push_str(&format!("{}{}", sign, change));
            }
        }
        let message = format!(
            "Output:\n{}\n-------\n did not equal\nExpected:\n{}\n-------\nDiff:\n{}",
            rewrite.trim(),
            expected.trim(),
            diff_str
        );
        println!("{}", message);
        // save actual in a ${test}.actual.tsx file
        let actual_filename = test.to_owned() + ".actual.tsx";
        let root_filename = get_fixtures_root()?;
        // println!("root_filename: {}", root_filename.display());
        let actual_path = root_filename
            .join(pattern)
            .join("expected")
            .join(actual_filename);

        std::fs::write(actual_path, &rewrite)?;
    }
    assert_eq!(
        rewrite.trim(),
        expected.trim(),
        "\nRewrite did not equal expected\nRewrite: \n{}\nExpected: \n{}",
        rewrite,
        expected
    );
    // println!("Rewrite: \n{}", rewrite);
    new_files_assertion(pattern, test, result.new_files)
}

fn new_files_assertion(
    pattern: &str,
    test: &str,
    output_files: HashMap<String, String>,
) -> Result<()> {
    let root = get_fixtures_root()?;
    let test = Path::new(test);
    let test = test.file_stem().unwrap();
    let files_path = format!(
        "{}/test_patterns/{}/expected/{}_files/",
        root.display(),
        pattern,
        test.to_str().unwrap()
    );
    let files_path = Path::new(&files_path);
    if !files_path.is_dir() {
        return Ok(());
    }
    let files = std::fs::read_dir(files_path)?;
    let count = files.count();
    assert_eq!(
        count,
        output_files.len(),
        "Expected {} new files, but got {} files",
        count,
        output_files.len()
    );
    let files = WalkDir::new(files_path);
    let files = files
        .into_iter()
        .filter_map(|file| file.ok())
        .filter(|file| file.metadata().unwrap().is_file());
    // this test does not support directory paths, only simple file paths
    for file in files {
        let name = file.file_name();
        let name = name.to_str().unwrap();
        let content = std::fs::read(file.path())?;
        let content = String::from_utf8(content)?;
        let content = content.trim();
        let message = format!("pattern result missing file: {}", name);
        assert!(output_files.contains_key(name), "{}", message);
        let output_content = output_files.get(name).unwrap().trim();
        let message = format!(
            "Output:\n{}\n-------\n did not equal\nExpected:\n{}",
            output_content, content
        );
        assert_eq!(content, output_content, "{}", message);
    }
    Ok(())
}

fn test_match(pattern: &str, test: &str) -> Result<()> {
    let (result, _) = test_setup("test_patterns", pattern, test)?;
    assert!(result.is_some(), "pattern FAILED to match");
    Ok(())
}

fn test_no_match(pattern: &str, test: &str) -> Result<()> {
    let (result, _) = test_setup("test_patterns", pattern, test)?;
    assert!(result.is_none(), "pattern matched when it shouldn't have");
    Ok(())
}

fn test_setup(dir: &str, pattern: &str, test: &str) -> Result<(ExecutionResult, Option<String>)> {
    let root = get_fixtures_root()?;
    let input = root.join(dir).join(pattern).join("input").join(test);
    let expected = root.join(dir).join(pattern).join("expected").join(test);
    let pattern = root
        .join(dir)
        .join(pattern)
        .join(format!("{}.grit", pattern));

    let lang = TargetLanguage::from_extension(
        Path::new(test)
            .extension()
            .ok_or_else(|| anyhow!("test parameter {} must have an extension", test))?
            .to_str()
            .ok_or_else(|| anyhow!("test parameter {} is malformed path", test))?,
    )
    .ok_or_else(|| {
        anyhow!(
            "test parameter {} extension didn't correspond to any supported language",
            test
        )
    })?;
    let pattern = std::fs::read_to_string(pattern)?;
    let input = std::fs::read_to_string(input)?;
    let expected = std::fs::read_to_string(expected).ok();
    Ok((
        match_pattern_one_file(pattern, "test-file.tsx", &input, lang)?,
        expected,
    ))
}

fn run_test_expected(arg: TestArgExpected) -> Result<()> {
    let pattern = arg.pattern;
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let source = arg.source;
    let result = match_pattern_one_file(pattern, "test-file.tsx", &source, js_lang)?;
    validate_execution_result(result, arg.expected)
}

fn run_test_expected_libs(arg: TestArgExpectedWithLibs) -> Result<()> {
    let pattern = arg.pattern;
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let source = arg.source;
    let mut libs = BTreeMap::new();
    for (i, pattern) in arg.lib_patterns.iter().enumerate() {
        libs.insert(format!("TestPattern-{}", i).to_owned(), pattern.to_owned());
    }
    let result = match_pattern_libs(pattern, &libs, "test-file.tsx", &source, js_lang)?;
    validate_execution_result(result, arg.expected)
}

fn validate_execution_result(result: ExecutionResult, expected: String) -> Result<()> {
    let result = result
        .the_match
        .ok_or_else(|| anyhow!("pattern failed to MATCH"))?;
    let rewrite = result
        .rewrite
        .ok_or_else(|| anyhow!("found a match but no rewrite"))?;
    let rewrite = rewrite
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    let rewrite = rewrite.trim();
    let expected = expected
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    let expected = expected.trim();
    if rewrite != expected {
        panic!("Tests do not match:\nExpected:\n-----------\n{}\n-----------\nReceived:\n-----------\n{}\n-----------", expected, rewrite)
    }
    Ok(())
}

fn run_test_expected_with_new_file(arg: TestArgExpectedWithNewFile) -> Result<()> {
    let pattern = arg.pattern;
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let source = arg.source;
    let result = match_pattern_one_file(pattern, "test-file.tsx", &source, js_lang)?;
    let result = result.the_match.unwrap();
    let rewrite = result.rewrite.unwrap();
    let rewrite = rewrite.trim();
    let expected = arg.expected.trim();
    if rewrite != expected {
        panic!("{} != {}", rewrite, expected)
    }
    if let Some(file_body) = result.new_files.get(&arg.new_file_name) {
        let file_body = file_body.trim();
        let expected = arg.new_file_body.trim();
        if file_body != expected {
            panic!("{} != {}", file_body, expected)
        }
    } else {
        panic!(
            "new file {} not found in {:?}",
            arg.new_file_name, result.new_files
        )
    }
    Ok(())
}

fn run_test_match(arg: TestArg) -> Result<()> {
    let pattern = arg.pattern;
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let source = arg.source;
    let mut parser = TSParser::new()?;
    parser.set_language(js_lang.get_ts_language()).unwrap();
    let result = match_pattern_one_file(pattern, "test-file.tsx", &source, js_lang)?;
    assert!(result.is_some());
    Ok(())
}

fn run_test_no_match(arg: TestArg) -> Result<()> {
    let pattern = arg.pattern;
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let source = arg.source;
    let mut parser = TSParser::new()?;
    parser.set_language(js_lang.get_ts_language()).unwrap();
    let result = match_pattern_one_file(pattern, "test-file.tsx", &source, js_lang)?;
    assert!(!result.is_some());
    Ok(())
}

#[test]
fn test_compile_time_logging() {
    let pattern = "
    `let foo = $foo` as $foo where {
        $foo <: contains `bar`
      }
    "
    .to_string();
    let libs = BTreeMap::new();
    let default_language = PatternLanguage::Tsx.try_into().unwrap();
    let pattern =
        src_to_problem_libs(pattern, &libs, default_language, None, None, None, None).unwrap();
    let res = format!("{:?}", pattern.compilation_warnings);
    assert_snapshot!(res);
}

#[test]
fn warns_against_snippet_useless_rewrite() {
    let pattern = "
    `export type {$types}` => `export type {$types}` where {
        $types <: contains `Props`
      }
    "
    .to_string();
    let libs = BTreeMap::new();
    let default_language = PatternLanguage::Tsx.try_into().unwrap();
    let pattern =
        src_to_problem_libs(pattern, &libs, default_language, None, None, None, None).unwrap();
    let res = format!("{:?}", pattern.compilation_warnings);
    assert_snapshot!(res);
}

#[test]
fn does_not_warn_against_regular_snippet_rewrite() {
    // technically this pattern is also useless, but the warning
    // is only for verbatim `snippetA` => `snippetA` rewrites
    let pattern = "
    `const $a = $b` => `const $b = $a` where {
        $a <: $b
      }
    "
    .to_string();
    let libs = BTreeMap::new();
    let default_language = PatternLanguage::Tsx.try_into().unwrap();
    let pattern =
        src_to_problem_libs(pattern, &libs, default_language, None, None, None, None).unwrap();
    assert!(pattern.compilation_warnings.is_empty())
}

#[test]
fn warns_against_snippet_regex_without_metavars() {
    let pattern = "
    `let foo = $foo` where {
        $foo <: r`[a-zA-Z]*`
      }
    "
    .to_string();
    let libs = BTreeMap::new();
    let default_language = PatternLanguage::Tsx.try_into().unwrap();
    let pattern =
        src_to_problem_libs(pattern, &libs, default_language, None, None, None, None).unwrap();
    let res = format!("{:?}", pattern.compilation_warnings);
    assert_snapshot!(res);
}

#[test]
fn does_not_warn_against_snippet_regex_with_metavars() {
    let pattern = "
`console.log(\"$message\")` where {
    $name = \"Lucy\",
    $message <: r`([a-zA-Z]*), $name`($greeting) => `$name, $greeting`
}
    "
    .to_string();
    let libs = BTreeMap::new();
    let default_language = PatternLanguage::Tsx.try_into().unwrap();
    let pattern =
        src_to_problem_libs(pattern, &libs, default_language, None, None, None, None).unwrap();
    assert!(pattern.compilation_warnings.is_empty())
}

#[test]
fn pattern_only_snippet() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language js
                |
                |`import $import from "$source"`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |import { assert } from 'chai';
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn simple_spread_snippet_operator() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                engine marzano(0.1)
                language js
                `console.log($message, $...)` => `console.error($message)`"#
                .to_owned(),
            source: r#"
            console.log("foo")
            console.log("bar", "baz")
            console.log("qux", "quux", "buux")
            console.log()"#
                .to_owned(),
            expected: r#"
            console.error("foo")
            console.error("bar")
            console.error("qux")
            console.log()"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn simple_spread_snippet_operator_statments() {
    run_test_expected({
        TestArgExpected {
            pattern: r"
                engine marzano(0.1)
                language js
                `console.log($foo)
                $...
                console.warn($bar)` => `console.error($foo)\nconsole.error($bar)`"
                .to_owned(),
            source: r#"
            console.log("foo")
            for (let i = 0; i < 10; i++) {
                console.log("bar", "baz")
            }
            console.warn("bar")"#
                .to_owned(),
            expected: "console.error(\"foo\")\nconsole.error(\"bar\")".to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn template_string_metavariable_match() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language js
                |
                |js"styled`$_`"
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const StyledText = styled`
                |text-decoration: ${props => (props.noUnderline ? 'none' : 'underline')};
                |&:hover {
                |  text-decoration: underline;
                |  color: ${color('linkTextHover')};
                |}
                |`;
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn import_clause_metavariable() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language js
                |
                |`import ShouldNotBeRemoved, $clause from 'node-fetch';`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"import ShouldNotBeRemoved, { fetch } from 'node-fetch';"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn template_string_metavariable_match_simple() {
    run_test_match({
        TestArg {
            pattern: r#"
                language js

                js"`foo ${bar}`"
                "#
            .to_owned(),
            source: r#"`foo ${bar}`"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn template_string_metavariable_match_fancy() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language js
                |
                |js"styled`
                |text-decoration: ${$_};
                |&:hover {
                |  text-decoration: underline;
                |  color: ${$foo};
                |}`"
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const StyledText = styled`
                |text-decoration: ${props => (props.noUnderline ? 'none' : 'underline')};
                |&:hover {
                |  text-decoration: underline;
                |  color: ${color('linkTextHover')};
                |}`;
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn extends_component_snippet() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language js
                |
                |`class $_ extends $component { $_ }`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |class Button extends Component {
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn not_is_undefined() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |or {
                |    `1`,
                |    `2` where $is_two = true
                |} where {
                |    if (!$is_two <: undefined) {
                |        $n = `two`
                |    } else {
                |        $n = `one`
                |    }
                |} => $n"#
                .trim_margin()
                .unwrap(),
            source: r#"
            console.log(1);
            console.log(2)"#
                .to_owned(),
            expected: r#"
            console.log(one);
            console.log(two)"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn is_not_undefined() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |or {
                |    `1`,
                |    `2` where $is_two = true
                |} where {
                |    if ($is_two <: not undefined) {
                |        $n = `two`
                |    } else {
                |        $n = `one`
                |    }
                |} => $n"#
                .trim_margin()
                .unwrap(),
            source: r#"
            console.log(1);
            console.log(2)"#
                .to_owned(),
            expected: r#"
            console.log(one);
            console.log(two)"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn optional_chaining_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`!$x?.length` => `$x?.length === 0`"#
                .trim_margin()
                .unwrap(),
            source: r#"
            if (!repos?.length) {
                return true;
              }
              if (!repos.length) {
                  return true;
                }"#
            .to_owned(),
            expected: r#"
            if (repos?.length === 0) {
                return true;
              }
              if (!repos.length) {
                  return true;
                }"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn chaining_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`!$x.length` => `$x.length === 0`"#
                .trim_margin()
                .unwrap(),
            source: r#"
            if (!repos?.length) {
                return true;
            }
            if (!repos.length) {
                return true;
            }"#
            .to_owned(),
            expected: r#"
            if (repos.length === 0) {
                return true;
            }
            if (repos.length === 0) {
                return true;
            }"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn open_ai_imports_comma_deletion() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
            engine marzano(0.1)
            language js

            `import $old from $src` where {
                $old <:
                    import_clause(name = named_imports($imports)) where {
                            $imports <: some bubble $name => . where {
                                $name <: not "toFile",
                            },
                            $old => js"OpenAI, $old",
                    }
            }"#
            .to_owned(),
            source: r#"
            |import { abcdef, ghijkl, mnopqr, toFile } from 'openai';"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |import OpenAI, {    toFile } from 'openai';"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn deduplicate_insertions() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`{ $params }` where {
                |    $params += `, 'hello': world`
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
            hello({ thing: 'two', });"#
                .to_owned(),
            expected: r#"
            hello({ thing: 'two', 'hello': world });"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn deduplicate_open_insertions() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
            pattern upsert($key, $value) {
                and {
                    or {
                        `{ $params }` where { $params <: . } => `{ $key: $value }`,
                        `{ $params }` where {
                            $params <: or {
                                some `$keylike: $old` where {
                                    $keylike <: or {
                                        $key,
                                        string(fragment=$fragment) where {
                                            $key <: r"(.+)"($raw),
                                            $fragment <: $raw
                                        }
                                    },
                                    $old => $value
                                },
                                $obj where {
                                    $obj += `, $key: $value`
                                }
                            }
                        }
                    }
                }
            }

            `new OpenAI($params)` where {
                $params <: upsert(key=`"baseURL"`, value=`"https://openrouter.ai/api/v1"`),
                $params <: contains `defaultHeaders: $headers` where {
                  $headers <: upsert(key=`"HTTP-Referer"`, value=`YOUR_SITE_URL`),
                  $headers <: upsert(key=`"X-Title"`, value=`YOUR_SITE_NAME`)
                }
            }
            "#
            .to_owned(),
            source: r#"
            import OpenAI from 'openai';
            const openai = new OpenAI({
              apiKey: OPENROUTER_API_KEY,
              defaultHeaders: {
                'X-Custom-Header': 'hello',
              },
            });"#
                .to_owned(),
            expected: r#"
            import OpenAI from 'openai';
            const openai = new OpenAI({
              apiKey: OPENROUTER_API_KEY,
              defaultHeaders: {
                'X-Custom-Header': 'hello', "HTTP-Referer": YOUR_SITE_URL, "X-Title": YOUR_SITE_NAME
              },
               "baseURL": "https://openrouter.ai/api/v1"
            });"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn only_deduplicate_rewrites() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`this.you` => `this.me`"#
                .trim_margin()
                .unwrap(),
            source: r#"
            var increment = function (i) {
                console.log([foo,,bar])
              return i + 1;
            };

            var remember = function (me) {
              this.you = me;
            };"#
            .to_owned(),
            expected: r#"
            var increment = function (i) {
                console.log([foo,,bar])
              return i + 1;
            };

            var remember = function (me) {
              this.me = me;
            };"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn only_deduplicate_rewrites2() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`console.log` => `foo`"#
                .trim_margin()
                .unwrap(),
            source: r#"
            import { LogLevel } from '../../types';

            console.log('hello');

            const [, extractedFilePath, , , lineNumber, column, errorCode, errorMessage] = match;"#
                .to_owned(),
            expected: r#"
            import { LogLevel } from '../../types';

            foo('hello');

            const [, extractedFilePath, , , lineNumber, column, errorCode, errorMessage] = match;"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn only_deduplicate_rewrites3() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`column` => ."#
                .trim_margin()
                .unwrap(),
            source: r#"
            const [, extractedFilePath, , , lineNumber, column, errorCode, errorMessage] = match;"#
                .to_owned(),
            expected: r#"
            const [, extractedFilePath, , , lineNumber,  errorCode, errorMessage] = match;"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn only_deduplicate_rewrites4() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`extractedFilePath` => ."#
                .trim_margin()
                .unwrap(),
            source: r#"
            const [, extractedFilePath, , , lineNumber, column, errorCode, errorMessage] = match;"#
                .to_owned(),
            expected: r#"
            const [,  , , lineNumber, column, errorCode, errorMessage] = match;"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn only_deduplicate_rewrites5() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |file($body) where $body <: contains `test($name, $fn)` => `it($name, $fn)`"#
                .trim_margin()
                .unwrap(),
            source: r#"
            test('foo', async () => {
                `"bar"`,
            });"#
                .to_owned(),
            expected: r#"
            it('foo', async () => {
                `"bar"`,
            });"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn only_deduplicate_rewrites6() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                engine marzano(0.1)
                language json
                pair($key, $value) => . where {
                    $key <: `"extends"`,
                }"#
            .to_owned(),
            source: r#"
            {
                "env": "bar",
                "extends": "foo",
                "baz": 1
            }"#
            .to_owned(),
            expected: r#"
            {
                "env": "bar",

                "baz": 1
            }"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn lone_comma_deletion() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`extractedFilePath` => ."#
                .trim_margin()
                .unwrap(),
            source: r#"
            const [extractedFilePath,] = match;"#
                .to_owned(),
            expected: r#"
            const [] = match;"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn empty_line_deletion() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |import_statement() => ."#
                .trim_margin()
                .unwrap(),
            source: r#"
                |import fetch from 'node-fetch';
                |console.log("hello");
                |import defaultNotNamedFetch, { fetch } from 'node-fetch';
                |console.log("hello");"#
                .trim_margin()
                .unwrap(),
            expected: r#"
                |console.log("hello");
                |console.log("hello");
                "#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn empty_line_deletion_with_tab() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`console.log($_)` => ."#
                .trim_margin()
                .unwrap(),
            source: r#"
                |function sayHi() {
                |	console.log('hi');
                |}"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |function sayHi() {
                |}"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn empty_line_deletion_with_spaces() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`console.log($_)` => ."#
                .trim_margin()
                .unwrap(),
            source: r#"
                |function sayHi() {
                |  console.log('hi');
                |console.log('trailing spaces');
                |}"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |function sayHi() {
                |}"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn trailing_comma_many_comma_deletion() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`foo` => ."#
                .trim_margin()
                .unwrap(),
            source: r#"
            const [ , , foo, , ] = match;"#
                .to_owned(),
            expected: r#"
            const [ , ,  , ] = match;"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn trailing_comma_doesnt_break_working() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`foo` => ."#
                .trim_margin()
                .unwrap(),
            source: r#"
            const [ bar, , foo, , ] = match;"#
                .to_owned(),
            expected: r#"
            const [ bar, ,  , ] = match;"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn file_name_on_rhs() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |`($args) => { return $value }` => `$filename`"#
                .trim_margin()
                .unwrap(),
            source: r#"
            var times = (x, y) => {
              return x * y;
            };"#
            .to_owned(),
            expected: r#"var times = test-file.tsx;"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn json_invert_pair_snippet() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language json
                |`$foo: $bar` => `$bar: $foo`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"{ "foo": "bar" }"#.to_owned(),
            expected: r#"{ "bar": "foo" }"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn go_hello_world() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language go
                |`fmt.Println($foo)` => `fmt.Println("goodbye")`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
            func main() {
                fmt.Println("hello world")
            }"#
            .to_owned(),
            expected: r#"
            func main() {
                fmt.Println("goodbye")
            }"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn go_imports() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language go
                |`import ($mod)` where {
                |    $mod => `"foo";`
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
            import ("fmt"; "bar")
            "#
            .to_owned(),
            expected: r#"
            import ("foo";)
            "#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn go_imports_string_fragment_metqvariable() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language go
                |`import "$mod"` where {
                |    $mod => `foo`
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
            import "fmt"
            "#
            .to_owned(),
            expected: r#"
            import "foo"
            "#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn go_binary_operators() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language go
                |`$a = $b` => `$b = $a`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
            package morestrings
            func ReverseRunes(s string) string {
                r := []rune(s)
                for i, j := 0, len(r)-1; i < len(r)/2; i, j = i+1, j-1 {
                    r[i], r[j] = r[j], r[i]
                }
                return string(r)
            }"#
            .to_owned(),
            expected: r#"
            package morestrings
            func ReverseRunes(s string) string {
                r := []rune(s)
                for i, j := 0, len(r)-1; i < len(r)/2; i+1, j-1 = i, j {
                    r[j], r[i] = r[i], r[j]
                }
                return string(r)
            }"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn simple_sql() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language sql
                |`SELECT $people FROM $source;` => `SELECT Enemies FROM Rolodex;`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"SELECT Friends FROM Contacts;"#.to_owned(),
            expected: r#"SELECT Enemies FROM Rolodex;"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn sql_metavariable_identifier() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language sql
                |`public.$table` => `private.$table`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"SELECT Friends FROM public.friends;"#.to_owned(),
            expected: r#"SELECT Friends FROM private.friends;"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn sql_metavariable_create_procedure() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language sql
                |`CREATE OR REPLACE PROCEDURE $name($arg1,$arg2) AS $block;` => `$name\n$arg1\n$arg2\n$block`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"CREATE OR REPLACE PROCEDURE greetings(IN a varchar2(40),OUT b varchar(40)) AS BEGIN SELECT 1 FROM DUAL; END;"#.to_owned(),
            expected: r#"
            |greetings
            |IN a varchar2(40)
            |OUT b varchar(40)
            |BEGIN SELECT 1 FROM DUAL; END"#.trim_margin().unwrap()
        }
    })
    .unwrap();
}

#[test]
fn sol_ether_transfer() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language sol
                |pattern EtherTransfer($amount) {
                |  bubble($amount) or {
                |      `$sender.call{value: $amount}($_)` => `MARKER`,
                |      `$sender.call.value($amount)($_)`,
                |      `$call($amount)` where {
                |          $call <: `$address.$functionName`,
                |          $functionName <: r".*transfer.*"
                |      }
                |  }
                |}
                |
                |EtherTransfer(amount = $_)
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |pragma solidity ^0.8.9;
            |contract HelloWorld {
            |  string public greet = "Hello World!";
            |
            |  function claim(
            |    uint256 numPasses,
            |    uint256 amount,
            |    uint256 mpIndex,
            |    bytes32[] calldata merkleProof
            |  ) external payable {
            |    require(isValidClaim(numPasses,amount,mpIndex,merkleProof));
            |
            |    //return any excess funds to sender if overpaid
            |    uint256 excessPayment = msg.value.sub(numPasses.mul(mintPasses[mpIndex].mintPrice));
            |    (bool returnExcessStatus, ) = _msgSender().call{value: excessPayment}("");
            |
            |    mintPasses[mpIndex].claimedMPs[msg.sender] = mintPasses[mpIndex].claimedMPs[msg.sender].add(numPasses);
            |    _mint(msg.sender, mpIndex, numPasses, "");
            |    emit Claimed(mpIndex, msg.sender, numPasses);
            |  }
            |}
            |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |pragma solidity ^0.8.9;
            |contract HelloWorld {
            |  string public greet = "Hello World!";
            |
            |  function claim(
            |    uint256 numPasses,
            |    uint256 amount,
            |    uint256 mpIndex,
            |    bytes32[] calldata merkleProof
            |  ) external payable {
            |    require(isValidClaim(numPasses,amount,mpIndex,merkleProof));
            |
            |    //return any excess funds to sender if overpaid
            |    uint256 excessPayment = msg.value.sub(numPasses.mul(mintPasses[mpIndex].mintPrice));
            |    (bool returnExcessStatus, ) = MARKER;
            |
            |    mintPasses[mpIndex].claimedMPs[msg.sender] = mintPasses[mpIndex].claimedMPs[msg.sender].add(numPasses);
            |    _mint(msg.sender, mpIndex, numPasses, "");
            |    emit Claimed(mpIndex, msg.sender, numPasses);
            |  }
            |}
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn sol_nested_loop() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language sol
                |pattern Loop($body) {
                |    or {
                |        `while($_) { $body }`,
                |        `for ($_; $_; $_) { $body }`
                |    }
                |}
                |
                |Loop($body) where $body <: contains Loop(body = $_) => `MARKER`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |pragma solidity ^0.8.9;
            |contract HelloWorld {
            |    string public greet = "Hello World!";
            |
            |    function foo(string memory _greet) public {
            |        while(other) {
            |            greet = foo(bar);
            |            while(foo) {
            |                greet = foo(bar);
            |            }
            |        }
            |    }
            |}
            |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |pragma solidity ^0.8.9;
            |contract HelloWorld {
            |    string public greet = "Hello World!";
            |
            |    function foo(string memory _greet) public {
            |        while(other) {
            |            greet = foo(bar);
            |            MARKER
            |        }
            |    }
            |}
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn sol_no_round_up() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language sol
                |`$_.mulDivRoundUp($amount, $_)` => `MARKER`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |pragma solidity ^0.8.9;
            |contract HelloWorld {
            |  string public greet = "Hello World!";
            |
            |  function claim(
            |    uint256 numPasses,
            |    uint256 amount,
            |    uint256 mpIndex,
            |    bytes32[] calldata merkleProof
            |  ) external payable {
            |    require(isValidClaim(numPasses,amount,mpIndex,merkleProof));
            |
            |    //return any excess funds to sender if overpaid
            |    uint256 excessPayment = msg.value.sub(numPasses.mul(mintPasses[mpIndex].mintPrice));
            |    (bool returnExcessStatus, ) = _msgSender().call{value: excessPayment}("");
            |
            |    mintPasses[mpIndex].claimedMPs[msg.sender] = mintPasses[mpIndex].mulDivRoundUp(numPasses, 3);
            |    _mint(msg.sender, mpIndex, numPasses, "");
            |    emit Claimed(mpIndex, msg.sender, numPasses);
            |  }
            |}
            |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |pragma solidity ^0.8.9;
            |contract HelloWorld {
            |  string public greet = "Hello World!";
            |
            |  function claim(
            |    uint256 numPasses,
            |    uint256 amount,
            |    uint256 mpIndex,
            |    bytes32[] calldata merkleProof
            |  ) external payable {
            |    require(isValidClaim(numPasses,amount,mpIndex,merkleProof));
            |
            |    //return any excess funds to sender if overpaid
            |    uint256 excessPayment = msg.value.sub(numPasses.mul(mintPasses[mpIndex].mintPrice));
            |    (bool returnExcessStatus, ) = _msgSender().call{value: excessPayment}("");
            |
            |    mintPasses[mpIndex].claimedMPs[msg.sender] = MARKER;
            |    _mint(msg.sender, mpIndex, numPasses, "");
            |    emit Claimed(mpIndex, msg.sender, numPasses);
            |  }
            |}
            |"#
        .trim_margin()
        .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn sol_no_unused_variables_1() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |// THIS PATTERN OVERMATCHES IN THE LOCAL CASE Z SHOULD NOT BE MATCHED
                |language sol
                |or {
                |  // find all our state variable definitions
                |  state_variable_declaration($name) as $dec where {
                |      $dec <: within contract_declaration() as $contract,
                |      $contract <: not contains function_definition(parameters=contains $name)
                |  },
                |
                |  // find all our local variable definitions
                |  variable_declaration(name=$id) as $def where {
                |      // that are *not* used outside the variable declaration
                |      ! $def <: within function_body(body=contains $id where {
                |          $id <: not within $def
                |      })
                |  }
                |} => `MARKER`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |pragma experimental ABIEncoderV2;
            |
            |import "./base.sol";
            |
            |contract DerivedA is Base {
            |    // i is not used in the current contract
            |    A i = A(1);
            |
            |    int internal j = 500;
            |
            |    function assign3(A memory x) public returns (uint) {
            |        return g[1] + x.a + uint(j);
            |    }
            |}
            |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |pragma experimental ABIEncoderV2;
            |
            |import "./base.sol";
            |
            |contract DerivedA is Base {
            |    // i is not used in the current contract
            |    MARKER
            |
            |    MARKER
            |
            |    function assign3(A memory x) public returns (uint) {
            |        return g[1] + x.a + uint(j);
            |    }
            |}
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn sol_no_unused_variables_2() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language sol
                |or {
                |  // find all our state variable definitions
                |  state_variable_declaration($name) as $dec where {
                |      $dec <: within contract_declaration() as $contract,
                |      $contract <: not contains function_definition(parameters=contains $name)
                |  },
                |
                |  // find all our local variable definitions
                |  variable_declaration(name=$id) as $def where {
                |      // that are *not* used outside the variable declaration
                |      ! $def <: within function_body(body=contains $id where {
                |          $id <: not within $def
                |      })
                |  }
                |} => `MARKER`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |pragma solidity ^0.5.0;
            |contract UnusedVariables {
            |    int a = 1;
            |
            |    // x is not accessed
            |    function neverAccessed(int test) public pure returns (int) {
            |        int z = 10;
            |
            |        if (test > z) {
            |            // x is not used
            |            int x = test - z;
            |
            |            return test - z;
            |        }
            |
            |        return z;
            |    }
            |}
            |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |pragma solidity ^0.5.0;
            |contract UnusedVariables {
            |    MARKER
            |
            |    // x is not accessed
            |    function neverAccessed(int test) public pure returns (int) {
            |        int z = 10;
            |
            |        if (test > z) {
            |            // x is not used
            |            MARKER = test - z;
            |
            |            return test - z;
            |        }
            |
            |        return z;
            |    }
            |}
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn sol_upgradable_proxy() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language sol
                |contract_declaration($heritage) where {
                |  $heritage <: contains or { "Proxy", "ERC1967Upgrade", "TransparentUpgradeableProxy", "UUPSUpgradeable" }
                |} => `MARKER`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |// SPDX-License-Identifier: MIT
            |// compiler version must be greater than or equal to 0.8.13 and less than 0.9.0
            |pragma solidity ^0.8.9;
            |
            |contract HelloWorld is UUPSUpgradeable, Another {
            |    string public greet = "Hello World!";
            |
            |    function foo(string memory _greet) public {
            |        greet = foo(bar);
            |    }
            |}
            |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |// SPDX-License-Identifier: MIT
            |// compiler version must be greater than or equal to 0.8.13 and less than 0.9.0
            |pragma solidity ^0.8.9;
            |
            |MARKER
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn jsx_attribute_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |`as` => `foo`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"<Header as='h1'>First Header</Header>"#.to_owned(),
            expected: r#"<Header foo='h1'>First Header</Header>"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn jsx_dotted_component() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |`PageContainer.Header` => `foobar`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
console.log(PageContainer.Header);

const foo = <PageContainer.Header sx={{ flexGrow: 1, gap: '20px' }} {...rest} />
"#
            .to_owned(),
            expected: r#"
console.log(foobar);

const foo = <foobar sx={{ flexGrow: 1, gap: '20px' }} {...rest} />
"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn imports() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |
                |pattern before_each_file_prep_imports() {
                |    $_ where {
                |        $GLOBAL_IMPORTED_SOURCES = [],
                |        $GLOBAL_IMPORTED_NAMES = [],
                |    }
                |}
                |
                |pattern the_import_statement($imports, $source) {
                |    import_statement(import = import_clause(name = named_imports($imports)), $source)
                |}
                |
                |pattern imported_from($from) {
                |    $name where {
                |        $program <: program($statements),
                |        $statements <: some bubble($name, $from) the_import_statement($imports, source = $from) where {
                |            $imports <: some $name,
                |        }
                |    }
                |}
                |
                |pattern ensure_import_from($source) {
                |    $name where {
                |        if ($name <: not imported_from(from = $source)) {
                |            if ($GLOBAL_IMPORTED_SOURCES <: not some [$program, $source]) {
                |                $GLOBAL_IMPORTED_SOURCES += [$program, $source]
                |            } else {
                |                true
                |            },
                |            if ($GLOBAL_IMPORTED_NAMES <: not some [$program, $name, $source]) {
                |                $GLOBAL_IMPORTED_NAMES += [$program, $name, $source]
                |            } else {
                |                true
                |            }
                |        } else {
                |            true
                |        }
                |    }
                |}
                |
                |pattern process_one_source($p, $all_imports) {
                |    [$p, $source] where {
                |        $imported_names = [],
                |        $GLOBAL_IMPORTED_NAMES <: some bubble($p, $source, $imported_names, $all_imports) [$p, $name, $source] where {
                |            $imported_names += $name,
                |        },
                |        $joined_imported_names = join(list = $imported_names, separator = ", "),
                |        if ($p <: program(statements = some the_import_statement($imports, $source))) {
                |            $imports => `$imports, $joined_imported_names`
                |        } else {
                |            $all_imports += `import { $joined_imported_names } from $source;\n`
                |        }
                |    }
                |}
                |
                |pattern insert_imports() {
                |    $p where {
                |        $all_imports = [],
                |        $GLOBAL_IMPORTED_SOURCES <: some process_one_source($p, $all_imports),
                |        if ($all_imports <: not []) {
                |            $p => `$all_imports\n$p`
                |        } else {
                |            true
                |        }
                |    }
                |}
                |
                |pattern after_each_file_handle_imports() {
                |  file(body = $p) where $p <: maybe insert_imports()
                |}
                |
                |pattern remove_import($from) {
                |    $name where {
                |        // Handle named imports
                |        $program <: maybe contains bubble($name, $from) `import $clause from $raw_source` as $import where {
                |          $raw_source <: contains `$from`,
                |          $clause <: or {
                |            // Handle module import
                |            import_clause(default=$name) where {
                |                $import => .
                |            },
                |            // Handle named import
                |            import_clause(name = named_imports($imports)) where {
                |                $others = `false`,
                |                if ($imports <: [$name]) {
                |                    $import => .
                |                } else {
                |                    $imports <: some $name => .
                |                }
                |            }
                |          }
                |        }
                |    }
                |}
                |
                |pattern replace_import($old, $new) {
                |    $name where {
                |        $name <: remove_import(from = $old),
                |        $name <: ensure_import_from(source=$new)
                |    }
                |}
                |
                |pattern literal_value() {
                |  or { number(), string(), `null`, `undefined`}
                |}
                |
                |pattern function_like($name, $args, $statements) {
                |  or {
                |    `function $name($args) { $statements }`,
                |    `($args) => { $statements }`,
                |    `($args) => $statements`
                |  }
                |}
                |
                |// All core stdlib functions can be done here
                |pattern before_each_file_stdlib() {
                |  before_each_file_prep_imports()
                |}
                |
                |pattern after_each_file_stdlib() {
                |  after_each_file_handle_imports()
                |}
                |
                |
                |// These could be redefined in the future (not presently supported)
                |pattern before_each_file() {
                |  before_each_file_stdlib()
                |}
                |
                |pattern after_each_file() {
                |  after_each_file_stdlib()
                |}
                |
                |contains or {
                |    `v4` as $v4 where {
                |      $source = `"uuid"`,
                |      // Use ensure_import_from to ensure a metavariable is imported.
                |      $v4 <: ensure_import_from($source),
                |    },
                |    `orderBy` as $orderBy where {
                |      $orderBy <: replace_import(old=`"underscore"`, new=`"lodash"`)
                |    },
                |    `fetch` as $fetch where {
                |      $from = `node-fetch`,
                |      // Use remove_import to remove an import entirely
                |      $fetch <: remove_import($from)
                |    },
                |    `class $_ extends $comp { $_ }` where {
                |      $comp <: `Component`,
                |      $source = `"React"`,
                |      $comp <: ensure_import_from($source)
                |    }
                |}
                "#
            .trim_margin()
            .unwrap(),
            source: r#"
            |import { orderBy } from 'underscore';
            |import fetch from 'elsewhere';
            |import { fetch } from 'node-fetch';
            |import { fetch, more } from 'node-fetch';
            |import fetch from 'node-fetch';
            |
            |console.log(orderBy([1, 2, 3]));
            |
            |console.log(v4());
            |
            |fetch();"#
                .trim_margin().unwrap(),
            expected: r#"
            |import { orderBy } from "lodash";
            | import { v4 } from "uuid";
            |
            |
            |import fetch from 'elsewhere';
            |import {  more } from 'node-fetch';
            |
            |console.log(orderBy([1, 2, 3]));
            |
            |console.log(v4());
            |
            |fetch();"#
                .trim_margin().unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn prefer_is_nan() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |
                |pattern any_equals($a, $b) {
                |  or { `$a == $b` , `$a === $b` , `$b == $a` , `$b === $a` }
                |}
                |
                |pattern any_not_equals($a, $b) {
                |  or {
                |      binary_expression(operator = or { `!==` , `!=` }, left = $a, right = $b),
                |      binary_expression(operator = or { `!==` , `!=` }, left = $b, right = $a)
                |  }
                |}
                |
                |or {
                |  any_equals(a = `NaN`, $b) => `isNaN($b)`,
                |  any_not_equals(a = `NaN`, $b) => `!isNaN($b)`
                |}
                "#
            .trim_margin()
            .unwrap(),
            source: r#"
            if (foo == NaN) {}
            if (foo === NaN) {}
            if (foo != NaN) {}"#
                .to_owned(),
            expected: r#"
            if (isNaN(foo)) {}
            if (isNaN(foo)) {}
            if (!isNaN(foo)) {}"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn python_handle_multiline_strings() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |
                |`$_ = $x` => `$x`"#
                .trim_margin()
                .unwrap(),
            source: r#"
                |def test_yaml_file():
                |    """some test comment"""
                |    variable = """
                |title: "Title"
                |        """"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |def test_yaml_file():
            |    """some test comment"""
            |    """
            |title: "Title"
            |        """"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn import_specifier_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |
                |`print($x)` where {
                |    $x <: contains `$key: $value`
                |} => `print($value)`"#
                .trim_margin()
                .unwrap(),
            source: r#"print({"foo": "bar"})"#.to_owned(),
            expected: r#"print("bar")"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn python_linearize_padding() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |
                |`with $open: $body` =>
                |    `async with $open:
                |    $body` where {
                |        $body <: contains `for $x in $y: $loop` => `for ever:
                |    $loop`
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |def read_and_process_file(file_path: str) -> None:
            |    with open(file_path, 'r') as file:
            |        for line in file:
            |            process_line(line)
            |            process_line(line)"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |def read_and_process_file(file_path: str) -> None:
            |    async with open(file_path, 'r') as file:
            |        for ever:
            |            process_line(line)
            |            process_line(line)"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn yaml_linearize_padding() {
    run_test_expected({
        TestArgExpected {
            pattern: r"
                |language yaml
                |`foo: $items` where $items += `- nicer`"
                .trim_margin()
                .unwrap(),
            source: r#"
                |foo:
                |  - item: name
                |    value: 3
                |  - item: two
                |    value: 4"#
                .trim_margin()
                .unwrap(),
            expected: r#"
                |foo:
                |  - item: name
                |    value: 3
                |  - item: two
                |    value: 4
                |  - nicer"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn yaml_match_list() {
    run_test_expected({
        TestArgExpected {
            pattern: r"
                 |language yaml
                 |`jobs: $jobs` where {
                 |    $jobs <: some bubble `- name: $x` where {
                 |       $x => `BOB`
                 |    }
                 |}"
            .trim_margin()
            .unwrap(),
            source: r#"
                 |---
                 |resources:
                 |# The repo with our Dockerfile
                 |- name: concourse-examples
                 |  type: git
                 |  icon: github
                 |  source:
                 |    uri: https://github.com/concourse/examples.git
                 |    branch: master
                 |
                 |jobs:
                 |- name: build-and-use-image
                 |- name: hello"#
                .trim_margin()
                .unwrap(),
            expected: r#"
                     |---
                     |resources:
                     |# The repo with our Dockerfile
                     |- name: concourse-examples
                     |  type: git
                     |  icon: github
                     |  source:
                     |    uri: https://github.com/concourse/examples.git
                     |    branch: master
                     |
                     |jobs:
                     |- name: BOB
                     |- name: BOB"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn undefined_assignment() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |
                |contains bubble($async_client, $client) {
                |    `openai.completions.acreate` where {
                |        if ($async_client <: undefined) {
                |            $async_client = `async_client`,
                |        },
                |        $new = `$async_client.completions.create`
                |    } => `$new`
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"await openai.completions.acreate(...)"#.to_owned(),
            expected: r#"await async_client.completions.create(...)"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn undefined_assignment2() {
    run_test_expected({
        TestArgExpected {
            pattern: r"
                |engine marzano(0.1)
                |language python
                |
                |contains bubble($async_client, $client) {
                |    `openai.completions.acreate` where {
                |        if ($async_client <: undefined) {
                |            $async_client = `async_client`,
                |            $new = `async_client = AsyncClient()\nawait $async_client.completions.acreate`
                |        } else {
                |            $new = `$async_client.completions.acreate`
                |        }
                |
                |    } => `$new`
                |}"
            .trim_margin()
            .unwrap(),
            source: r#"
            |await openai.completions.acreate(...)
            |
            |// Avoid initializing twice
            |await openai.completions.acreate(...)"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |await async_client = AsyncClient()
            |await async_client.completions.acreate(...)
            |
            |// Avoid initializing twice
            |await async_client.completions.acreate(...)"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn python_easy_sub() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |
                |`with explicit_read_replica_session() as $session: $body`
                |where {
                |    $body <: contains $session  => `db.session`
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |    with explicit_read_replica_session() as rr_session:
            |        insights = rr_session.query(Insight.insight_key).all()
            |        return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |    with explicit_read_replica_session() as rr_session:
            |        insights = db.session.query(Insight.insight_key).all()
            |        return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn predicate_maybe() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |
                |`function $foo() { $body }` where {
                |    maybe false,
                |    maybe $body => `baz`,
                |    $foo => `bar`
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |function hello() {
            |    console.log("world")
            |}"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |function bar() {
            |    baz
            |}
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn only_rewrite_matching_vars() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |contains `with $_:$body` where {
                |  $body <: contains $foo where {
                |    $foo <: `foo`,
                |    $foo <: not within call(),
                |    $foo => `MARKER`,
                |  }
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |def foo() -> list[Any]:
            |    with bar() as foo:
            |        foo
            |        insights = foo.call()"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |def foo() -> list[Any]:
            |    with bar() as foo:
            |        MARKER
            |        insights = foo.call()"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn only_rewrite_matching_vars_tricky() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |
                |`with bar() as $foo: $body` where {
                |    $body <: contains {
                |       $foo where {
                |           $foo <: not within call(),
                |           $foo => `MARKER`,
                |       }
                |    }
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |def foo() -> list[Any]:
            |   with bar() as foo:
            |      insights = foo.call()
            |      foo
            |      something(foo = foo)"#
                .trim_margin()
                .unwrap(),
            // first foo rewrites because it had already matched in this scope
            expected: r#"
            |def foo() -> list[Any]:
            |   with bar() as MARKER:
            |      insights = foo.call()
            |      MARKER
            |      something(foo = foo)"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn only_rewrite_matching_vars_tricky1() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |
                |`with bar() as $foo: $body` where {
                |    $body <: contains {
                |       $foo where {
                |           $foo <: within call(),
                |           $foo => `MARKER`,
                |       }
                |    }
                |}"#
            .trim_margin()
            .unwrap(),
            // first foo rewrites even though it does not match the inner condition
            // because it had already matched in this scope
            source: r#"
            |def foo() -> list[Any]:
            |   with bar() as foo:
            |      insights = foo.call()
            |      foo
            |      something(foo = foo)"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |def foo() -> list[Any]:
            |   with bar() as MARKER:
            |      insights = MARKER.call()
            |      foo
            |      something(MARKER = MARKER)"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn only_rewrite_matching_vars1() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |`with $as_expr:$body` where {
                |  $new_session = `read_replica_session_context`,
                |  $as_expr <: contains `explicit_read_replica_session() as $session` => `$new_session()`,
                |  $body <: maybe contains $session => `db.session` where { $session <: not within call()}
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |   with explicit_read_replica_session() as rr_session:
            |       insights = rr_session.query(Insight.insight_key).all()
            |       rr_session
            |       something(rr_session = rr_session)
            |       return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |   with read_replica_session_context():
            |       insights = rr_session.query(Insight.insight_key).all()
            |       db.session
            |       something(rr_session = rr_session)
            |       return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn python_dont_regress_ramp1() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |
                |`with $as_expr:$body` where {
                |  $new_session = `read_replica_session_context`,
                |  $as_expr <: contains `explicit_read_replica_session() as $session` => `$new_session()`,
                |  $body <: maybe contains bubble($session) { // we need a bubble here to treat each "$session" instance differently
                |      $session => `db.session` where { $session <: not within keyword_argument(name=$session) }
                |  },
                |  // As a special case, rewrite keyword *values* we excluded earlier
                |  $body <: maybe contains keyword_argument(value=$session => `db.session`)
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |   with explicit_read_replica_session() as rr_session:
            |      insights = rr_session.query(Insight.insight_key).all()
            |      rr_session
            |      something(rr_session = rr_session)
            |      return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |   with read_replica_session_context():
            |      insights = db.session.query(Insight.insight_key).all()
            |      db.session
            |      something(rr_session = db.session)
            |      return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn python_dont_regress_ramp_no_bubble() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |
                |`with $as_expr:$body` where {
                |  $new_session = `read_replica_session_context`,
                |  $as_expr <: contains `explicit_read_replica_session() as $session` => `$new_session()`,
                |  $body <: maybe contains {
                |      $session => `db.session` where { $session <: not within keyword_argument(name=$session) }
                |  },
                |  // As a special case, rewrite keyword *values* we excluded earlier
                |  $body <: maybe contains keyword_argument(value=$session => `db.session`)
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |   with explicit_read_replica_session() as rr_session:
            |      insights = rr_session.query(Insight.insight_key).all()
            |      rr_session
            |      something(rr_session = rr_session)
            |      return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |   with read_replica_session_context():
            |      insights = db.session.query(Insight.insight_key).all()
            |      db.session
            |      something(rr_session = db.session)
            |      return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn python_print_to_log() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |pattern print_to_log() {
                |    `print($x)` => `log($x)`
                |}
                |print_to_log()"#
                .trim_margin()
                .unwrap(),
            source: r#"
            print("hello world!")
            "#
            .to_owned(),
            expected: r#"
            log("hello world!")
            "#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn python_multi_line_sub() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |
                |`with explicit_read_replica_session() as $session: $body`
                |=> `with explicit_read_replica_session():
                |    $body`"#
                .trim_margin()
                .unwrap(),
            source: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |    with explicit_read_replica_session() as rr_session:
            |        insights = rr_session.query(Insight.insight_key).all()
            |        return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |    with explicit_read_replica_session():
            |        insights = rr_session.query(Insight.insight_key).all()
            |        return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn python_pad_corectly_please_match() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |
                |`with explicit_read_replica_session() as $session:
                |    $body`
                |where {
                |    $body <: contains $session => `db.session`
                |}
                |=> `with explicit_read_replica_session():
                |    $body`"#
                .trim_margin()
                .unwrap(),
            source: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |    with explicit_read_replica_session() as rr_session:
            |        insights = rr_session.query(Insight.insight_key).all()
            |        return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |    with explicit_read_replica_session():
            |        insights = db.session.query(Insight.insight_key).all()
            |        return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn python_pad_corectly_no_panic() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |
                |`with explicit_read_replica_session() as $session: $body`
                |where {
                |    $body <: contains $session => `db.session`
                |}
                |=> `with explicit_read_replica_session():
                |    $body`"#
                .trim_margin()
                .unwrap(),
            source: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |    with explicit_read_replica_session() as rr_session:
            |        insights = rr_session.query(Insight.insight_key).all()
            |        return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |    with explicit_read_replica_session():
            |        insights = db.session.query(Insight.insight_key).all()
            |        return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn python_dont_regress_ramp() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |
                |`with $as_expr:$body` where {
                |    $as_expr <: contains `explicit_read_replica_session() as $session` => `explicit_read_replica_session()`,
                |    $body <: contains $session => `db.session`
                |}"#
                .trim_margin()
                .unwrap(),
            source: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |    with explicit_read_replica_session() as rr_session:
            |        insights = rr_session.query(Insight.insight_key).all()
            |        return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |def fetch_all_insight_keys_using_read_replica() -> list[Any]:
            |    with explicit_read_replica_session():
            |        insights = db.session.query(Insight.insight_key).all()
            |        return [insight.insight_key for insight in insights]"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_python_multiline() {
    run_test_expected({
        TestArgExpected {
            pattern: r"
                |engine marzano(0.1)
                |language python
                |`return $x` => `log($x)\nlog($x)`"
                .trim_margin()
                .unwrap(),
            source: r#"
                |class Person:
                |  def __str__(self):
                |    return "foo"
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |class Person:
            |  def __str__(self):
            |    log("foo")
            |    log("foo")
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_python_multiline_first_not_on_start() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |`"foo"` => `log("foo")\nlog("foo")`"#
                .trim_margin()
                .unwrap(),
            source: r#"
                |class Person:
                |  def __str__(self):
                |    return "foo"
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |class Person:
            |  def __str__(self):
            |    return log("foo")
            |    log("foo")
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn quote_snippet_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"foo" => js"baz"
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"import { foo as bar } from 'chai';"#.to_owned(),
            expected: r#"import { baz as bar } from 'chai';"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn simple_toml() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language toml
                |`[$foo]` where {
                |    $foo => `bar`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"[workspace]"#.to_owned(),
            expected: r#"[bar]"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn toml_key_replacement() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language toml
                |`key = $value` => `new_key = $value`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"key = "original_value""#.to_owned(),
            expected: r#"new_key = "original_value""#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn toml_nested_metavar() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language toml
                |`name = $value` where {
                |  $value <: `"marzano-cli"` => `"marzano-madness"`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
            | [package]
            | name = "marzano-cli"
            | version = "0.1.1"
            | edition = "2021"
            | authors = ["Grit Developers <support@grit.io>"]
            "#
            .to_owned()
            .trim_margin()
            .unwrap(),
            expected: r#"
            | [package]
            | name = "marzano-madness"
            | version = "0.1.1"
            | edition = "2021"
            | authors = ["Grit Developers <support@grit.io>"]
            "#
            .to_owned()
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn toml_within() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language toml
                |`[dependencies]
                |$deps` where {
                |  $deps <: some bubble `$name = $version` where {
                |    $version <: string(),
                |    $version => `{ version = $version }`
                |  }
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
            | [lib]
            | path = "src/lib.rs"
            |
            | [dependencies]
            | anyhow = "1.0.70"
            | clap = { version = "4.1.13", features = ["derive"] }
            | indicatif = "0.17.5"
            "#
            .to_owned()
            .trim_margin()
            .unwrap(),
            expected: r#"
            | [lib]
            | path = "src/lib.rs"
            |
            | [dependencies]
            | anyhow = { version = "1.0.70" }
            | clap = { version = "4.1.13", features = ["derive"] }
            | indicatif = { version = "0.17.5" }
            "#
            .to_owned()
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn toml_array_append() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language toml
                |`array = [$values]` => `array = [$values, "new_element"]`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"array = ["one", "two"]"#.to_owned(),
            expected: r#"array = ["one", "two", "new_element"]"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn toml_table_rename() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language toml
                |`[$x]
                |$values` where $x <: `other` => `renamed`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
            | [other]
            | nest = "marzano-cli"
            | [package]
            | name = "marzano-cli"
            | version = "0.1.1"
            | edition = "2021"
            | authors = ["Grit Developers <support@grit.io>"]
            "#
            .to_owned()
            .trim_margin()
            .unwrap(),
            expected: r#"
            | [renamed]
            | nest = "marzano-cli"
            | [package]
            | name = "marzano-cli"
            | version = "0.1.1"
            | edition = "2021"
            | authors = ["Grit Developers <support@grit.io>"]
            "#
            .to_owned()
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn multi_args_snippet() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language js
                |
                |`Array($args)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |Array(1, 2, 3);
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn hcl_pair() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language hcl
                |
                |`$arg: $red`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |default_address = "127.0.0.1"
                |default_message = upper("Incident: ${incident}")
                |default_options = {
                |  priority: "High",
                |  color: "Red"
                |}
                |
                |incident_rules {
                |  # Rule number 1
                |  rule "down_server" "infrastructure" {
                |    incident = 100
                |    options  = var.override_options ? var.override_options : var.default_options
                |    server   = default_address
                |    message  = default_message
                |  }
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn hcl_implicit_regex() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language hcl
                |
                |`required_version = "~> $current_version"` => `required_version = "~> v1.5.0"`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |default_address = "127.0.0.1"
                |required_version = "~> v5.4.0"
                |incident_rules {
                |  rule "down_server" "infrastructure" {
                |    required_version = "~> v1.0.0"
                |  }
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |default_address = "127.0.0.1"
                |required_version = "~> v1.5.0"
                |incident_rules {
                |  rule "down_server" "infrastructure" {
                |    required_version = "~> v1.5.0"
                |  }
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn includes_or() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`console.log($_)` as $haystack where {
                |   $haystack <: includes or { "world", "handsome" }
                |} => `console.log("Goodbye world!")`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log("Hello world!");
                |console.log("Hello handsome!");
                |console.log("But not me, sadly.");
                |console.log("Hi, Hello world!");
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log("Goodbye world!");
                |console.log("Goodbye world!");
                |console.log("But not me, sadly.");
                |console.log("Goodbye world!");
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn includes_and() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`console.log($_)` as $haystack where {
                |   $haystack <: includes and { "Hello", "handsome" }
                |} => `console.log("Goodbye world!")`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log("Hello world!");
                |console.log("Hello handsome!");
                |console.log("But not me, handsome.");
                |console.log("Hi, Hello world!");
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log("Hello world!");
                |console.log("Goodbye world!");
                |console.log("But not me, handsome.");
                |console.log("Hi, Hello world!");
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn includes_any() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`console.log($_)` as $haystack where {
                |   $haystack <: includes any { "Hello", "handsome" }
                |} => `console.log("Goodbye world!")`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log("Hello world!");
                |console.log("Hello handsome!");
                |console.log("But not me, handsome.");
                |console.log("Hi, Hello world!");
                |console.log("Just not this....");
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log("Goodbye world!");
                |console.log("Goodbye world!");
                |console.log("Goodbye world!");
                |console.log("Goodbye world!");
                |console.log("Just not this....");
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn includes_regex() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`console.log($_)` as $haystack where {
                |   $haystack <: includes r"Hello"
                |} => `console.log("Goodbye world!")`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log("Hello world!");
                |console.log("Hello handsome!");
                |console.log("But not me, sadly.");
                |console.log("Hi, Hello world!");
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log("Goodbye world!");
                |console.log("Goodbye world!");
                |console.log("But not me, sadly.");
                |console.log("Goodbye world!");
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn includes_regex_with_capture() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`console.log($_)` as $haystack where {
                |   $haystack <: includes r"Hello (\w+)"($name)
                |} => `console.log("Goodbye $name!")`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log("Hello world!");
                |console.log("Hello handsome!");
                |console.log("But not me, sadly.");
                |console.log("Hi, Hello world!");
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log("Goodbye world!");
                |console.log("Goodbye handsome!");
                |console.log("But not me, sadly.");
                |console.log("Goodbye world!");
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn parses_simple_pattern() {
    let pattern = r#"
        |engine marzano(0.1)
        |language js
        |`console.log($msg)`"#
        .trim_margin()
        .unwrap();

    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let _pattern = src_to_problem(pattern, js_lang).unwrap();
}

#[test]
fn warning_rewrite_in_not() {
    let pattern = r#"
        |`async ($args) => { $body }` where {
        |    $body <: not contains `try` => ` try {
        |        $body
        |    } catch { }`
        |}"#
    .trim_margin()
    .unwrap();

    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let libs = BTreeMap::new();
    let cr = src_to_problem_libs(pattern, &libs, js_lang, None, None, None, None).unwrap();
    assert_debug_snapshot!(cr.compilation_warnings)
}

#[test]
fn should_log_variable() {
    let pattern = r#"
        |engine marzano(0.1)
        |language js
        |
        |`function ($args) { $body }` => `($args) => {
        |  $body
        |}` where {
        |    log(variable=$body)
        |}
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
    |var increment = function (i) {
    |    return i + 1;
    |};"#
        .trim_margin()
        .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results =
        pattern.execute_file(&RichFile::new(file.to_owned(), source.to_owned()), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn rewrite_dot() {
    let pattern = r#"
            engine marzano(0.1)
            language js

            `new Date($bob)` where {
                $bob <: . => `"fo"`
            }"#
    .to_owned();
    let source = r#"
            new Date()"#
        .to_owned();
    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results =
        pattern.execute_file(&RichFile::new(file.to_owned(), source.to_owned()), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn test_simple_log() {
    let pattern = r#"
        |language js
        |
        |program() as $x where {
        |    log(message="this is a message",variable=$x)
        |}
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
    |foo;bar;baz;
    |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn test_variable_message_log() {
    let pattern = r#"
        |language js
        |
        |program() as $x where {
        |    $message = "this is a message",
        |    log(message=$message,variable=$x),
        |}
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
    |foo;bar;baz;
    |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn test_shorthand_log() {
    let pattern = r#"
        |language js
        |
        |`baz` as $x where {
        |    $message = "this is a message",
        |    $variable = $x,
        |    log($message,$variable),
        |}
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
    |foo;bar;baz;
    |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn test_before_each_file() {
    let pattern = r#"
        |language js
        |
        |pattern before_each_file() {
        |  $p where $p <: contains `foo;` => raw`fooooo;`
        |}
        |
        |`baz` => `baaaz`
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
    |foo;bar;baz;
    |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";
    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn test_after_each_file() {
    let pattern = r#"
        |language js
        |
        |pattern after_each_file($) {
        |  $p where $p <: contains `bar;` => raw`baaaar;`
        |}
        |
        |`baz` => `baaaz`
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
    |foo;bar;baz;
    |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn test_before_and_after_each_file() {
    let pattern = r#"
        |language js
        |
        |pattern before_each_file($) {
        |  $p where $p <: contains `foo;` => `fooooo;`
        |}
        |
        |pattern after_each_file($) {
        |  $p where $p <: contains `bar;` => `baaaar;`
        |}
        |
        |`baz` => `baaaz`
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
    |foo;bar;baz;
    |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn std_lib_array_snippet() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language js
                |
                |`const stdlib = [$old] as const`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"const stdlib = [foo] as const"#.to_string(),
        }
    })
    .unwrap();
}

#[test]
fn simple_yaml_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language yaml
                |
                |`foo: bar` => `foo: baz`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |foo: bar
            |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |foo: baz
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn nested_yaml_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language yaml
                |
                |`name: job` => `name: boz`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |bob:
            |  name: job
            |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |bob:
            |  name: boz
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn yaml_object_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language yaml
                |
                |`foo: $obj` where { $obj <: contains `thing: bar` => `thing: baz` }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |list:
            |- item: one
            |- item: two
            |  foo:
            |    thing: bar
            |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |list:
            |- item: one
            |- item: two
            |  foo:
            |    thing: baz
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn simple_yaml_rewrite_sequence_items() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language yaml
                |
                |`- $foo` => `- quux`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |version: v1beta1
            |build:
            |  roots:
            |    - .
            |    - foo
            |    - bar
            |lint:
            |  ignore:
            |    - dependencies
            |  use:
            |    - DEFAULT
            |breaking:
            |  ignore:
            |  use:
            |    - PACKAGE
            |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |version: v1beta1
            |build:
            |  roots:
            |    - quux
            |    - quux
            |    - quux
            |lint:
            |  ignore:
            |    - quux
            |  use:
            |    - quux
            |breaking:
            |  ignore:
            |  use:
            |    - quux
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn std_lib_activities_snippet() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language js
                |
                |`const stdlib = { $activities }`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"const stdlib = { foo: 7 }"#.to_string(),
        }
    })
    .unwrap();
}

#[test]
fn import_snippet_variable() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`import $legacy_image from "next/legacy/image"` => `$legacy_image`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |import Image from "next/legacy/image";
                |import ImageNew from "next/image";
                |
                |return (
                |    <>
                |        <Image />
                |        <ImageNew />
                |    </>
                |)"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |Image
                |import ImageNew from "next/image";
                |
                |return (
                |    <>
                |        <Image />
                |        <ImageNew />
                |    </>
                |)"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn js_match_single_arrow_args_list() {
    run_test_match({
        TestArg {
            pattern: r#"
            language js
            `($args) => $body` where {
                $args <: [$x, ...]
            }"#
            .to_owned(),
            source: r#"
            const x = y => {
               return y;
            }"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn js_match_arrow_args_list() {
    run_test_match({
        TestArg {
            pattern: r#"
            language js
            `($args) => $body` where {
                $args <: [$x, ...]
            }"#
            .to_owned(),
            source: r#"
            const x = (y) => {
               return y;
            }"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn js_match_single_arrow_args_list_without_parentheses() {
    run_test_match({
        TestArg {
            pattern: r#"
            language js
            `$args => $body` where {
                $args <: [$x, ...]
            }"#
            .to_owned(),
            source: r#"
            const x = y => {
               return y;
            }"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn js_match_arrow_args_list_without_parentheses() {
    run_test_match({
        TestArg {
            pattern: r#"
            language js
            `$args => $body` where {
                $args <: [$x, ...]
            }"#
            .to_owned(),
            source: r#"
            const x = (y) => {
               return y;
            }"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn js_paren_params() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
            contains bubble arrow_function(parameters=$x, parenthesis = $y) where {
                if ($y <: .) {
                    $x => `()`
                } else {
                    $x => .
                }
            }"#
            .to_owned(),
            source: r#"
            const foo = (bar) => {}
            const baz = qux => {}"#
                .to_owned(),
            expected: r#"
            const foo = () => {}
            const baz = () => {}"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn array_destrcutring_snippet() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language js
                |
                |`const [$name, $_] = useState($_)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"const [foo, bar] = useState(5)"#.to_string(),
        }
    })
    .unwrap();
}

#[test]
fn equals_zero_snippet() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language js
                |
                |`$y = $x == -0`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"let bar = foo(5) + 3 == -0"#.to_string(),
        }
    })
    .unwrap();
}

#[test]
fn extends_component() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language js
                |
                |`extends Component`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |class Button extends Component {
                |  a = 1;
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn import_react_snippet_no_match() {
    run_test_no_match({
        TestArg {
            pattern: r#"
                |language js
                |
                |`import React from 'react'`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |import * as React from "react"
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_ranges() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language js(js_do_not_use)
                |$obj where {
                |    $obj <: object($properties),
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const a = { foo: 'bar' };"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn pattern_test_filename_match_and_use() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |file($name, body = contains `whatever` => `$name`)
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |whatever()
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |test-file.tsx()
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_conditional_snippet() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |`if ($cond) { $cond_true }` => `$cond`"#
                .trim_margin()
                .unwrap(),
            source: r#"
                if (!response) {
                  return false;
                }"#
            .to_owned(),
            expected: r#"!response"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn test_file_rename() {
    let pattern = r#"
        |language js
        |
        |file($name, $body) where {
        |    $body <: contains `rename_to($new_name)` => `we_renamed_to($new_name)`,
        |    $name => `$new_name`
        |}
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
    |rename_to(the_new_name)
    |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn test_json_file_match_snippet() {
    let pattern = r#"
        |language json
        |
        |file($name, $body) where {
        |    $name <: r"(?:.+)package.json",
        |    $body <: contains pair(key=`"name"`, $value),
        |    $value => `"bar"`
        |}"#
    .trim_margin()
    .unwrap();

    let source = r#"
        {
          "not-name": "foo",
          "name": "foo"
        }
        "#
    .to_owned();

    let file = "foo.package.json";

    let context = ExecutionContext::default();
    let json_lang: TargetLanguage = PatternLanguage::Json.try_into().unwrap();
    let pattern = src_to_problem(pattern, json_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn test_basic_python() {
    let pattern = r#"
        |language python
        |
        |`print($msg)` => `log($msg)`
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"|print("hello world!")
                    |log("this is python")"#
        .trim_margin()
        .unwrap();

    let file = "foo.py";

    let context = ExecutionContext::default();
    let language: TargetLanguage = PatternLanguage::Python.try_into().unwrap();
    let pattern = src_to_problem(pattern, language).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn test_json_file_match_string() {
    let pattern = r#"
        |language json
        |
        |file($name, $body) where {
        |    $name <: r"(?:.+)package.json",
        |    $body <: contains pair(key="\"name\"", $value),
        |    $value => `"bar"`
        |}"#
    .trim_margin()
    .unwrap();

    let source = r#"
        {
          "not-name": "foo",
          "name": "foo"
        }
        "#
    .to_owned();

    let file = "foo.package.json";

    let context = ExecutionContext::default();
    let json_lang: TargetLanguage = PatternLanguage::Json.try_into().unwrap();
    let pattern = src_to_problem(pattern, json_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn pattern_test_new_files() {
    run_test_expected_with_new_file({
        TestArgExpectedWithNewFile {
            pattern: r#"
                |language js
                |
                |`foo($name, $body)` => `bar()` where {
                |    $new_files += file($name, $body)
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |foo(the_new_file.tsx, whatever)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |bar()
                |"#
            .trim_margin()
            .unwrap(),
            new_file_name: "the_new_file.tsx".into(),
            new_file_body: r#"
                |whatever
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_saving_info() {
    let pattern = r#"
        |language js
        |
        |program() where {
        |  $l = [],
        |  $keys = [],
        |  $output = ``,
        |  $program <: contains bubble ($l, $keys, $output) `foo($key, $value)` as $x where {
        |    if ($l <: not some [$key, $value]) $l += [$key, $value],
        |    if ($keys <: not some $key) $keys += $key
        |  },
        |  $keys <: some bubble($output, $l) $key where {
        |    $values = [],
        |    $l <: some bubble($key, $values) [$key, $value] where $values += $value,
        |    $values_string = join(list = $values, separator = ", "),
        |    $output += `$key->($values_string)`
        |  },
        |  $program => `$output`
        |}
        |
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
    |foo(1, 11);
    |foo(1, 12);
    |foo(2, 21)
    |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn test_assoc_matcher() {
    let pattern = r#"
        |language js
        |
        |program() where {
        |  $l = [[`1`, `2`, `3`]],
        |  $x = `5`,
        |  $l <: some [$x, $y, $z], // should not match because of $x
        |  $program => `$x, $y, $z`
        |}
        |
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
        |whatever
        |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn add_to_set() {
    let pattern = r#"
          pattern add_to_set($set) {
            $element where {
              if ($set <: not some $element) {
                $set += $element
              } else {
                true
              }
            }
          }
          `end` as $end where {
            $big_set = [],
            $program <: contains bubble($big_set) `foo($x, $y)` where {
                // this should work too:
                // $element = [$x, $y],
                //$element <: add_to_set(set = $big_set)

                if ($big_set <: not some [$x, $y]) {
                    $big_set += [$x, $y]
                  } else {
                    true
                  }
            },
            $big_set_string = join(list = $big_set, separator = "||| "),
            $end => `$big_set_string`
          }
            "#;

    let source = r#"
        |foo(1, 11);
        |foo(1, 12);
        |foo(1, 12);
        |foo(2, 21);
        |end
        |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern.to_owned(), js_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn add_to_set_via_pattern() {
    let pattern = r#"
          pattern add_to_set($set) {
            $element where {
              if ($set <: not some $element) {
                $set += $element
              } else {
                true
              }
            }
          }
          `end` as $end where {
            $big_set = [],
            $program <: contains bubble($big_set) `foo($x, $y)` where {
                // this should work too:
                $element = [$x, $y],
                $element <: add_to_set(set = $big_set)
            },
            $big_set_string = join(list = $big_set, separator = "||| "),
            $end => `$big_set_string`
          }
            "#;

    let source = r#"
        |foo(1, 11);
        |foo(1, 12);
        |foo(1, 12);
        |foo(2, 21);
        |end
        |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern.to_owned(), js_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn test_assignment_in_method_call() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |pattern use_info($comp_name) {
                |    variable_declarator($name, $value, $type) where {
                |        $comp_name = `foo`
                |    }
                |}
                |
                |use_info($comp_name) => `$comp_name`
                |
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const _DateSelect = {
                |    parameters: {
                |      info: 'This is a Select date Component',
                |    },
                |  };
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const foo;
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_same_var_twice_in_snippet() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`<$name $props>$children</$name>` where {
                |    $name <: `Foo` => `Bar`,
                |}
                |
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"<Foo name='sam'>Test</Foo>"#.to_owned(),
            expected: r#"<Bar name='sam'>Test</Bar>"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn simple_predicate() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |
                |predicate is_foo($name) {
                |    $name <: `Foo`
                |}
                |predicate rewrite($name) {
                |    $name => `Bar`,
                |}
                |
                |`<$name $props>$children</$name>` where {
                |    if (is_foo($name)) {
                |          rewrite($name)
                |    }
                |}
                |
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                | <Foo name='sam'>Test</Foo>
                | <Baz name='sam'>Test</Baz>"#
                .trim_margin()
                .unwrap(),
            expected: r#"
                | <Bar name='sam'>Test</Bar>
                | <Baz name='sam'>Test</Baz>"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn simple_predicate_false() {
    let res = run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |
                |predicate is_foo($name) {
                |    $name <: `Foo`
                |}
                |predicate rewrite($name) {
                |    $name => `Bar`,
                |}
                |
                |`<$name $props>$children</$name>` where {
                |    if (is_foo($name)) {
                |          rewrite($name)
                |    }
                |}
                |
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"<Baz name='sam'>Test</Baz>"#.to_owned(),
            expected: r#"<Baz name='sam'>Test</Baz>"#.to_owned(),
        }
    });
    assert_eq!(res.unwrap_err().to_string(), "found a match but no rewrite");
}

#[test]
fn test_import_none() {
    let root = get_fixtures_root().unwrap();
    let import_patterns = format!("{}/test_patterns/imports.grit", root.display());
    let import_patterns = std::fs::read_to_string(import_patterns).unwrap();

    let pattern = r#"

        `foo`
        "#;

    let pattern = format!("{}\n{}", import_patterns, pattern);

    let source = r#"
        |import { foo, bar } from 'bar'
        |import { other } from 'other.file'
        |"#
    .trim_margin()
    .unwrap();

    let results = execute(pattern, source);
    assert_yaml_snapshot!(results);
}

#[test]
fn rewrite_or_bubble_pattern_argument() {
    let pattern = r#"
        engine marzano(1.0)
        language js

        `it($_, $body)` as $m where {
            $body <: contains  `foo` as $to_rewrite,

            $m <: bubble($to_rewrite) $_ where {
                $to_rewrite => `MARKER`
            }
        }

        "#;

    let source = r#"
            it('consolidates', async () => {
              foo;
              foo;
              foo;
            });
        "#;

    let results = execute(pattern.into(), source.into());
    assert_yaml_snapshot!(results);
}

#[test]
fn test_import_just_insert() {
    let root = get_fixtures_root().unwrap();
    let import_patterns = format!("{}/test_patterns/imports.grit", root.display());
    let import_patterns = std::fs::read_to_string(import_patterns).unwrap();

    let pattern = r#"

        `foo` as $f where {
            $name = `buz`,
            $buuuz_source = `'buuuz'`,
            $name <: ensure_import_from(source = $buuuz_source),
        }
        "#;

    let pattern = format!("{}\n{}", import_patterns, pattern);

    let source = r#"
        |import { foo, bar } from 'bar'
        |import { other } from 'other.file'
        |"#
    .trim_margin()
    .unwrap();

    let results = execute(pattern, source);
    assert_yaml_snapshot!(results);
}

#[test]
fn pattern_call_as_rhs() {
    let root = get_fixtures_root().unwrap();
    let import_patterns = format!("{}/test_patterns/imports.grit", root.display());
    let import_patterns = std::fs::read_to_string(import_patterns).unwrap();

    let pattern = r#"
        // does not work:
        `orderBy` as $ob where { $ob <: ensure_import_from(source=`"lodash"`) }
        // does work:
        // `orderBy` as $ob where { $source=`"lodash"`, $ob <: ensure_import_from($source) }
        "#;

    let pattern = format!("{}\n{}", import_patterns, pattern);

    let source = r#"
        |var increment = function (i) {
        |    orderBy();
        |  return i + 1;
        |};
        |"#
    .trim_margin()
    .unwrap();
    let expected = r#"
        |import { orderBy } from "lodash";
        |var increment = function (i) {
        |    orderBy();
        |  return i + 1;
        |};
        |"#
    .trim_margin()
    .unwrap();
    run_test_expected(TestArgExpected {
        pattern,
        source,
        expected,
    })
    .unwrap();
}

#[test]
fn pattern_call_as_lhs_and_rhs() {
    let root = get_fixtures_root().unwrap();
    let import_patterns = format!("{}/test_patterns/imports.grit", root.display());
    let import_patterns = std::fs::read_to_string(import_patterns).unwrap();

    let pattern = r#"
            $x where {
                $x <: js"makeStyles",
                $x <: remove_import(from=`'@material-ui/core'`)
            }"#;

    let pattern = format!("{}\n{}", import_patterns, pattern);

    let source = r#"import { Box, Paper, makeStyles } from "@material-ui/core";"#.to_owned();
    let expected = r#"import { Box, Paper  } from "@material-ui/core";"#.to_owned();
    run_test_expected(TestArgExpected {
        pattern,
        source,
        expected,
    })
    .unwrap();
}

#[test]
fn test_import_all_already_there() {
    let root = get_fixtures_root().unwrap();
    let import_patterns = format!("{}/test_patterns/imports.grit", root.display());
    let import_patterns = std::fs::read_to_string(import_patterns).unwrap();

    let pattern = r#"

        `foo` as $f where {
            $new_name = `bar`,
            $bar_source = `'bar'`,
            $new_name <: ensure_import_from(source = $bar_source),
            $new_name <: ensure_import_from(source = $bar_source),
            $other_name = `other`,
            $other_source = `'other.file'`,
            $other_name <: ensure_import_from(source = $other_source),
        }
        "#;

    let pattern = format!("{}\n{}", import_patterns, pattern);

    let source = r#"
        |import { foo, bar } from 'bar'
        |import { other } from 'other.file'
        |"#
    .trim_margin()
    .unwrap();

    let results = execute(pattern, source);
    assert_yaml_snapshot!(results);
}

#[test]
fn test_import_multiple() {
    let root = get_fixtures_root().unwrap();
    let import_patterns = format!("{}/test_patterns/imports.grit", root.display());
    let import_patterns = std::fs::read_to_string(import_patterns).unwrap();

    let pattern = r#"

        `foo` as $f where {
            $name = `buz`,
            $buuuz_source = `'buuuz'`,
            $name <: ensure_import_from(source = $buuuz_source),
            $name <: ensure_import_from(source = $buuuz_source),
            $new_name = `morefoo`,
            $bar_source = `'bar'`,
            $new_name <: ensure_import_from(source = $bar_source),
            $new_name <: ensure_import_from(source = $bar_source),
            $other_name = `otherfoo`,
            $other_source = `'other.file'`,
            $other_name <: ensure_import_from(source = $other_source),
        }
        "#;

    let pattern = format!("{}\n{}", import_patterns, pattern);

    let source = r#"
        |import { foo, bar } from 'bar'
        |import { other } from 'other.file'
        |"#
    .trim_margin()
    .unwrap();

    let results = execute(pattern, source);
    assert_yaml_snapshot!(results);
}

#[test]
fn test_replace_program() {
    run_test_expected(TestArgExpected {
        pattern: r#"
            program() as $p where {
                $p <: contains `foo` => `buz`,
                //$p <: contains `next` => `$p`,
                $p => `>>$p<<`
            }
            "#
        .to_owned(),
        source: r#"foo(bar);"#.to_owned(),
        expected: r#">>buz(bar);<<"#.trim_margin().unwrap(),
    })
    .unwrap();
}

#[test]
#[ignore = "special case we don't care about yet"]
// to handle we could replace our HashMap<CodeRanges, Option<String>>
// with a Hashmap<CodeRanges, String>, and HashSet<Effects> so that
// we can apply both effects in a specified order.
fn test_duplicate_rewrite() {
    run_test_expected(TestArgExpected {
        pattern: r#"
            program() as $p where {
                $p => `foo($p)`,
                $p => `bar($p)`
            }
            "#
        .to_owned(),
        source: r#"123"#.to_owned(),
        expected: r#"foo(bar(123))"#.trim_margin().unwrap(),
    })
    .unwrap();
}

fn execute(pattern: String, source: String) -> Vec<MatchResult> {
    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    pattern.execute_file(&RichFile::new(file.to_owned(), source), &context)
}

#[test]
fn clone_activites() {
    let pattern = r#"
language js

multifile {
    $names = [],
    bubble($names) file($name, $body) where {
        $name <: r".+sdk/src/stdlib/index.ts",
        $body <: contains `const stdlib = { $activities }` where {
            $activities <: some bubble($names) $activity where {
                $names += `'$activity'`
            }
        }
    },
    bubble($names) file($name, $body) where {
      $body <: contains `const stdlib = [$old] as const`,
      $name <: r".+__generated__/stdlib.ts",
      $new_names = join(list = $names, separator = ", "),
      $old => `$new_names`,
    },
}"#
    .into();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let content1 = r#"
            |import { findConfigs } from './configs';
            |// Remember: ./packages/timekeeper/src/__generated__/stdlib.ts
            |export const stdlib = {
            |  show,
            |  apply,
            |  sh,
            |};
            |
            |export type StdLibFunctions = typeof stdlib;"#
        .trim_margin()
        .unwrap();
    let content2 = r#"
            |export const stdlib = [
            |  'foo',
            |  'bar',
            |  'baz',
            |  'qux'
            |] as const;
            |
            |export type StdLibNamedFunctions = typeof stdlib;"#
        .trim_margin()
        .unwrap();
    let context = ExecutionContext::default();
    let results = pattern.execute_files(
        &[
            RichFile::new(
                "~/dev/rewriter/packages/sdk/src/stdlib/index.ts".to_string(),
                content1,
            ),
            RichFile::new(
                "~/dev/rewriter/packages/timekeeper/src/__generated__/stdlib.ts".to_string(),
                content2,
            ),
        ],
        &context,
    );

    assert_yaml_snapshot!(results);
}

#[test]
fn test_filename() {
    let pattern = r#"
        |language js
        |
        |pattern foo() {
        |  $_ where $filename => `the_new_name`
        |}
        |
        |file(name = $filename, body = $program) where $program <: foo()
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
    |whatever;
    |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn test_program() {
    let pattern = r#"
        |language js
        |
        |pattern foo() {
        |  $_ where $program <: contains `find_this` => `replace_with_this`
        |}
        |
        |file(body = $program) where {
        |  $program <: foo()
        |}
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
    |other_stuff;
    |find_this;
    |and_more;
    |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn pattern_test_new_files_complex() {
    run_test_expected_with_new_file({
        TestArgExpectedWithNewFile {
            pattern: r#"
                language js

                file($name, $body) where {
                    $body <: contains `theNameOfTheFileIs($new_name)` => `'$name'`,
                    $body <: contains bubble `createNewFile($name, $body)` where {
                        $new_files += file($name, $body)
                    },
                    $name => `$new_name`
                }
                "#
            .into(),
            source: r#"
                |theNameOfTheFileIs(the_newwww)
                |createNewFile(the_new_name, the_new_body)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |'test-file.tsx'
                |createNewFile(the_new_name, the_new_body)
                |"#
            .trim_margin()
            .unwrap(),
            new_file_name: "the_new_name".into(),
            new_file_body: r#"
                |the_new_body
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_filename_autoset() {
    let pattern = r#"
        |language js
        |
        |`whatever` => `changed` where $filename => `the_new_name`
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
    |whatever;
    |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn test_program_autoset() {
    let pattern = r#"
        |language js
        |
        |`foo` where $program <: contains `bar` => `baz`
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
    |foo(bar);
    |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn pattern_test_new_files_in_pattern_def() {
    run_test_expected_with_new_file({
        TestArgExpectedWithNewFile {
            pattern: r#"
                language js

                pattern make_new_file() {
                    `createNewFile($name, $body)` where {
                        $new_files += file($name, $body)
                    }
                }

                file($name, $body) where {
                    $body <: contains `theNameOfTheFileIs($new_name)` => `'$name'`,
                    $body <: contains make_new_file(),
                    $name => `$new_name`
                }
                "#
            .into(),
            source: r#"
                |theNameOfTheFileIs(the_newwww)
                |createNewFile(the_new_name, the_new_body)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |'test-file.tsx'
                |createNewFile(the_new_name, the_new_body)
                |"#
            .trim_margin()
            .unwrap(),
            new_file_name: "the_new_name".into(),
            new_file_body: r#"
                |the_new_body
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn pattern_def_with_bad_arg() {
    run_test_no_match({
        TestArg {
            pattern: r#"
                |language js
                |pattern MainPattern($supertype) {
                |    class_declaration(
                |       name = $className,
                |       heritage = contains $supertype,
                |       body = $body
                |   ) => `const $className = $body`
                |}
                |contains MainPattern(supertype = "NotComponent")"#
                .trim_margin()
                .unwrap(),
            source: r#"
                |class Button extends Component {
                |  a = 1;
                |}
                |
                |class AnotherButton extends Component {
                |  b = 2;
                |}"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn pattern_by_reference() {
    run_test_match({
        TestArg {
            pattern: r#"
                |pattern foo($bar) {
                |    class_declaration(
                |        name = $className,
                |        heritage = contains "Component",
                |        body = $body
                |    ) => `const $className = $bar`
                |}
                |and {
                |   $baz = "{ b = 2; }",
                |   foo(bar = $baz)
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |class Button extends Component {
                |  a = 1;
                |}
                |
                |class AnotherButton extends Component {
                |  b = 2;
                |}"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_args_syntactic_sugar() {
    run_test_match({
        TestArg {
            pattern: r#"
                |pattern foo($bar) {
                |    class_declaration(
                |        name = $className,
                |        heritage = contains "Component",
                |        $body
                |    ) => `const $className = $bar`
                |}
                |and {
                |   $bar = "{ b = 2; }",
                |   foo($bar)
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |class Button extends Component {
                |  a = 1;
                |}
                |
                |class AnotherButton extends Component {
                |  b = 2;
                |}"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn curly_metavar() {
    run_test_match({
        TestArg {
            pattern: r#"
                |pattern foo($bar) {
                |    class_declaration(
                |        name = $className,
                |        heritage = contains "Component",
                |        body = $body
                |    ) => `const $className = $[bar]`
                |}
                |and {
                |   $baz = "{ b = 2; }",
                |   foo(bar = $baz)
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |class Button extends Component {
                |  a = 1;
                |}
                |
                |class AnotherButton extends Component {
                |  b = 2;
                |}"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn pattern_append_string() {
    run_test_match({
        TestArg {
            pattern: r#"
                |pattern handlePairs($accum) {
                |    or {
                |       pair_pattern(key = $key, value = $value) where {
                |           $accum += `$key as $value, `
                |       }
                |   }
                |}
                |and {
                |   $result = "{ ",
                |   variable_declarator(name = contains handlePairs(accum = $result)),
                |   $result += "}",
                |   $x => `$result`
                |}"#
            .trim_margin()
            .unwrap(),
            source:
                r#"const { named1, alias2: named2, alias3: named3 } = require('path/to/file/1');"#
                    .to_string(),
        }
    })
    .unwrap();
}

#[test]
fn export_object() {
    run_test_match({
            TestArg {
                pattern: r#"
                |and {
                |   $program,
                |   contains $x where {
                |       $x <: shorthand_property_identifier(),
                |       $program <: contains lexical_declaration(declarations = [variable_declarator(name = $x, value = $val)]) => `export const $x = $val`,
                |       $x => ``
                |   }
                |}"#
                    .trim_margin()
                    .unwrap(),
                source: r#"
                |const king = '9';
                |
                |module.exports = {
                |  king,
                |  queen: '8',
                |};"#
                .trim_margin()
                .unwrap(),
            }
        })
        .unwrap();
}

#[test]
fn transform_test() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language js
                |
                |`$name` where {
                |    $name <: within `var $name = $_`
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |// This is a comment. Please ignore it.
                |// Really we should just call variables by their number
                |var a = 1;
                |var b = 2;
                |// This is a comment. Please ignore it.
                |var c = 3;
                |var bigLongName = 4;"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn json_snippet_simple() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language json
                |
                |`{ "foo": $x }` => `{ "foo": "baz" }`"#
                .trim_margin()
                .unwrap(),
            source: r#"
                |{
                |    "foo": "bar"
                |}"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn python_node_type_mismatch() {
    run_test_match({
        TestArg {
            pattern: r#"
            |engine marzano(0.1)
            |language python
            |
            |`def stuff(): $body` where {
            |    $body <: contains `$x = "9"`,
            |    $body <: contains `print($x)`
            |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
            |def stuff():
            |  foo = "9"
            |  print(foo)"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_json_swap_kv() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language json
                |`{ $x: $y }` => `{ $y: $x }`"#
                .trim_margin()
                .unwrap(),
            source: r#"
                |{
                |    "foo": "bar"
                |}"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |{ "bar": "foo" }"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_step1() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |contains `{ foo: $y }` => `{ bar: $y }`"#
                .trim_margin()
                .unwrap(),
            source: r#"
                |let x = {
                |    foo: "bar",
                |};"#
                .trim_margin()
                .unwrap(),
            expected: r#"
                |let x = { bar: "bar" };"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}
#[test]
fn test_step2() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |contains `{ bar: $y }` => `{ baz: $y }`"#
                .trim_margin()
                .unwrap(),
            source: r#"
                |let x = { bar: "bar" };"#
                .trim_margin()
                .unwrap(),
            expected: r#"
                |let x = { baz: "bar" };"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn no_export_class() {
    run_test_no_match({
        TestArg {
            pattern: r#"
                |language js
                |pattern js_class_export() {
                |    $class where {
                |        $class <: and {
                |            class_declaration(name = $name, body = $body),
                |            not within export_statement()
                |        }
                |    }
                |}
                |
                |js_class_export()"#
                .trim_margin()
                .unwrap(),
            source: r#"export class Pattern1b {}"#.trim_margin().unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_any_pattern() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |any {
                |    call_expression(function = "f3", arguments = [$args]) => `g($args)`,
                |    call_expression(function = "f", arguments = [$args]) => `g($args)`,
                |    call_expression(function = "f2", arguments = [$args]) => `g($args)`
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   f("hello")
                |   f2("hello")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   g("hello")
                |   g("hello")
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_any_predicate() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |$x where any {
                |    true,
                |    $x => `foo`,
                |    false,
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   f("hello")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   foo
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_after_pattern() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |call_expression(function = $f, arguments = [$args]) as $call => `g($args)` where {
                |   $call <: after contains call_expression()
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   f("hello")
                |   f2("hello")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   f("hello")
                |   g("hello")
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_before_pattern() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |call_expression(function = $f, arguments = [$args]) as $call => `g($args)` where {
                |   $call <: before contains call_expression()
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |f("hi")
                |f2("hi")
                |// this will not be touched
                |function() {
                |  f3("bob");
                |}
                |// this one will
                |function() {
                |  f4("cool");
                |  f5("school");
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |g("hi")
            |f2("hi")
            |// this will not be touched
            |function() {
            |  f3("bob");
            |}
            |// this one will
            |function() {
            |  g("cool");
            |  f5("school");
            |}
            |
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_after_pattern_rhs() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |
                |`const foo = 9;` as $foo where {
                |    $previous = after $foo,
                |    $foo => $previous
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |var increment = function (i) {
                |    const foo = 9;
                |    const x = 10;
                |};"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |var increment = function (i) {
            |    const x = 10;
            |    const x = 10;
            |};"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_before_pattern_rhs() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |
                |`const foo = 9;` as $foo where {
                |    $previous = before $foo,
                |    $foo => $previous
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |var increment = function (i) {
                |    const x = 10;
                |    const foo = 9;
                |};"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |var increment = function (i) {
            |    const x = 10;
            |    const x = 10;
            |};"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_global_variables() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |pattern the_foo() {
                |   `foo($GLOBAL_VAR)`
                |}
                |pattern the_bar() {
                |   `bar` => `baz($GLOBAL_VAR)`
                |}
                |
                |program(statements = and {
                |  contains the_foo(),
                |  contains the_bar()
                |})
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   foo(1)
                |   bar
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   foo(1)
                |   baz(1)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_pattern_call() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |pattern foo() {
                |    call_expression(function="f", arguments=[$args]) => `g($args)`
                |}
                |contains foo()"#
                .trim_margin()
                .unwrap(),
            source: r#"
                |   f("hello")
                |   f("hello2")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   g("hello")
                |   g("hello2")
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_bubble() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |call_expression(function="f", arguments=[$args]) => `g($args)`"#
                .trim_margin()
                .unwrap(),
            source: r#"
                |   f("hello")
                |   f("hello2")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   g("hello")
                |   g("hello2")
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_range_line() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |`const $x` where {
                |    $x <: within range(start_line=2, end_line=3),
                |    $x => `FOO = 5`
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   const foo = 5
                |   const foo = 5
                |   const foo = 5
                |   const foo = 5
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |   const foo = 5
            |   const FOO = 5
            |   const FOO = 5
            |   const foo = 5
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_lazy_join() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
engine marzano(0.1)
language js

`class $foo { $body }` where {
    $fns = [],
    $body <: contains bubble($fns) {
        `$name($_) { $body }` as $fn where {
                $body => `"foo"`,
                $fns += $fn
            }
        },
    $joined = join(list=$fns, separator=`\n`),
    $body => $joined
}"#
            .to_owned(),
            source: r#"
class Rectangle {
  height = 2;
  constructor(height, width) {
    this.height = height;
    this.width = width;
  }

  first() {
    return 1 + 1;
  }
}"#
            .to_owned(),
            expected: r#"
class Rectangle {
  constructor(height, width) {
    "foo"
  }
first() {
    "foo"
  }
}"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn contains_lazy_join() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
engine marzano(0.1)
language js

`class $foo { $body }` where {
    $fns = [],
    $body <: contains bubble($fns) {
        `$name($_) { $body }` as $fn where {
                $body => `"foo"`,
                $fns += $fn
            }
        },
    $joined = join(list=$fns, separator=`\n`),
    $joined <: contains bubble `$name($_) { $body }` where {
        $name => `hammering_time`
    },
    $body => $joined
}"#
            .to_owned(),
            source: r#"
class Rectangle {
  height = 2;
  constructor(height, width) {
    this.height = height;
    this.width = width;
  }

  first() {
    return 1 + 1;
  }
}"#
            .to_owned(),
            expected: r#"
class Rectangle {
  hammering_time(height, width) {
    "foo"
  }
hammering_time() {
    "foo"
  }
}"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn debug_imports_lazy_join() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
engine marzano(0.1)
language js

pattern before_each_file() {
    $_ where {
        $GLOBAL_IMPORTED_SOURCES = [],
        $GLOBAL_IMPORTED_NAMES = [],
    }
}

pattern the_import_statement($imports, $source) {
    import_statement(import = import_clause(name = named_imports($imports)), $source)
}

pattern imported_from($from) {
    $name where {
        $program <: program($statements),
        $statements <: some bubble($name, $from) the_import_statement($imports, source = $from) where {
            $imports <: some $name,
        }
    }
}

pattern ensure_import_from1($source) {
    $name where {
        if ($name <: not imported_from(from = $source)) {
            if ($GLOBAL_IMPORTED_SOURCES <: not some [$program, $source]) {
                $GLOBAL_IMPORTED_SOURCES += [$program, $source]
            } else {
                true
            },
            if ($GLOBAL_IMPORTED_NAMES <: not some [$program, $name, $source]) {
                $GLOBAL_IMPORTED_NAMES += [$program, $name, $source]
            } else {
                true
            }
        } else {
            true
        }
    }
}

pattern process_one_source($p, $all_imports) {
    [$p, $source] where {
        $imported_names = [],
        $GLOBAL_IMPORTED_NAMES <: some bubble($p, $source, $imported_names, $all_imports) [$p, $name, $source] where {
            $imported_names += $name,
        },
        $joined_imported_names = text(join(list = $imported_names, separator = ", ")),
        if ($p <: program(statements = some the_import_statement($imports, $source))) {
            $imports => `$imports, $joined_imported_names`
        } else {
            $all_imports += `import { $joined_imported_names } from $source;\n`
        }
    }
}

pattern insert_imports() {
    $p where {
        $all_imports = [],
        $GLOBAL_IMPORTED_SOURCES <: some process_one_source($p, $all_imports),
        if ($all_imports <: not []) {
            or {
              // Try to find a shebang and insert after that
              $p <: program(hash_bang=$h) where {
                $h <: hash_bang_line() += `\n$all_imports`
              },
              // Find an import statement to anchor on
              $p <: program($statements) where {
                $statements <: some $anchor where { $anchor <: import_statement() },
                $anchor += `\n$all_imports`
              },
              // Fall back to inserting the whole program
              $p => `$all_imports\n$p`
            }
        } else {
            true
        }
    }
}

pattern after_each_file() {
  file($body) where $body <: maybe insert_imports()
}

pattern remove_import($from) {
    $name where {
        // Handle named imports
        $program <: maybe contains bubble($name, $from) `import $clause from $raw_source` as $import where {
          $raw_source <: contains $from,
          $clause <: or {
            // Handle module import
            import_clause(default=$name) where {
                $import => .
            },
            // Handle named import
            import_clause($default, name=named_imports($imports)) as $clause where {
                $others = `false`,
                if ($imports <: [$name]) {
                    if ($default <: .) {
                        $import => .
                    } else {
                        $clause => $default
                    }
                } else {
                    $imports <: some $name => .
                }
            }
          }
        }
    }
}

pattern replace_import($old, $new) {
    $name where {
        $name <: remove_import(from=$old),
        $name <: ensure_import_from1(source=$new)
    }
}

`ThemeProvider` as $target where {
  $target <: replace_import(old= `'@mui/styles'`, new=`'@mui/material/styles'`),
}
"#
            .to_owned(),
            source: r#"
import { ThemeProvider, styles } from '@mui/styles';"#
            .to_owned(),
            expected: r#"
import {  styles } from '@mui/styles';
import { ThemeProvider } from '@mui/material/styles';"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn test_range_line_no_within() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |range(start_line=2, end_line=3) as $foo where {
                |    $foo <: `const $x` where {
                |        $x => `FOO = 5`
                |    }
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   const foo = 5
                |   const foo = 5
                |   const foo = 5
                |   const foo = 5
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |   const foo = 5
            |   const FOO = 5
            |   const FOO = 5
            |   const foo = 5
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_range_column() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |`const $x` where {
                |    $x <: within range(start_line=2, start_column=10, end_line=4, end_column=17),
                |    $x => `FOO = 5`
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   const foo = 5
                |   const foo = 5
                |   const foo = 5
                |   const foo = 5
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |   const foo = 5
            |   const FOO = 5
            |   const FOO = 5
            |   const FOO = 5
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_range_column_out_of_range() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |`const $x` where {
                |    $x <: within range(start_line=2, start_column=11, end_line=4, end_column=16),
                |    $x => `FOO = 5`
                |}"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   const foo = 5
                |   const foo = 5
                |   const foo = 5
                |   const foo = 5
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |   const foo = 5
            |   const foo = 5
            |   const FOO = 5
            |   const foo = 5
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

// Built-ins

#[test]
fn builtin_capitalize() {
    test_rewrite("built_ins", "capitalize", "object.js").unwrap();
}

#[test]
fn builtin_uppercase() {
    test_rewrite("built_ins", "uppercase", "object.js").unwrap();
}

#[test]
fn builtin_lowercase() {
    test_rewrite("built_ins", "lowercase", "object.js").unwrap();
}

#[test]
fn nested_builtins() {
    test_rewrite("built_ins", "nested", "object.js").unwrap();
}

#[test]
fn builtin_length() {
    test_rewrite("built_ins", "length", "main.js").unwrap();
}

#[test]
fn builtin_join() {
    test_rewrite("built_ins", "join", "main.js").unwrap();
}

#[test]
fn builtin_distinct() {
    test_rewrite("built_ins", "distinct", "main.js").unwrap();
}

// Other rewrites

#[test]
fn new_files() {
    test_rewrite("test_patterns", "new_files", "foo.tsx").unwrap();
}

#[test]
#[ignore = "this test is broken in CI"]
fn es6_imports() {
    test_rewrite("test_patterns", "es6Imports", "imports.js").unwrap();
}

#[test]
fn steps_simple() {
    test_rewrite("test_patterns", "steps", "simple.js").unwrap();
}

#[test]
fn does_include() {
    test_rewrite("test_patterns", "includes", "does_include.js").unwrap();
}

#[test]
fn does_not_include() {
    test_no_match("includes", "does_not_include.js").unwrap();
}

#[test]
fn match_empty_field() {
    test_match("empty_field", "class.js").unwrap();
}

#[test]
fn if_syncronous() {
    test_rewrite("test_patterns", "else", "if_class.js").unwrap();
}

#[test]
fn else_asyncronous() {
    test_rewrite("test_patterns", "else", "else_class.js").unwrap();
}

#[test]
fn does_equal() {
    test_rewrite("test_patterns", "equals", "does_equal.js").unwrap();
}

#[test]
fn does_not_equal() {
    test_no_match("equals", "does_not_equal.js").unwrap();
}

#[test]
fn underscore_react_to_hooks() {
    test_match("underscore", "react_to_hooks.js").unwrap();
}

#[test]
fn react_to_hooks1_first() {
    test_rewrite("test_patterns", "react_to_hooks", "first.tsx").unwrap();
}

#[test]
fn react_to_hooks_double_quote_snippet_first() {
    test_rewrite("test_patterns", "react_to_hooks_double_quote", "first.tsx").unwrap();
}

// missing arg list
#[test]
fn react_to_hooks2_observable() {
    test_rewrite("test_patterns", "react_to_hooks", "mobx_observable.tsx").unwrap();
}

// missing arg list
#[test]
fn react_to_hooks3_reactions() {
    test_rewrite("test_patterns", "react_to_hooks", "mobx_reactions.tsx").unwrap();
}

#[test]
fn react_to_hooks4_view_state() {
    test_rewrite("test_patterns", "react_to_hooks", "mobx_view_state.tsx").unwrap();
}

#[test]
fn react_to_hooks5_prop_types() {
    test_rewrite("test_patterns", "react_to_hooks", "prop_types.tsx").unwrap();
}

#[test]
fn react_to_hooks7_lifecycle() {
    test_rewrite("test_patterns", "react_to_hooks", "lifecycle.tsx").unwrap();
}

#[test]
fn react_to_hooks8_pure_javascript() {
    test_rewrite("test_patterns", "react_to_hooks", "pure_javascript.tsx").unwrap();
}

#[test]
fn react_to_hooks9_observable_null() {
    test_rewrite(
        "test_patterns",
        "react_to_hooks",
        "mobx_observable_null.tsx",
    )
    .unwrap();
}

#[test]
fn react_to_hooks10_props_wiring() {
    test_rewrite("test_patterns", "react_to_hooks", "props_wiring.tsx").unwrap();
}

#[test]
fn react_to_hooks_no_extra_observer() {
    test_rewrite("test_patterns", "react_to_hooks", "no_extra_observer.tsx").unwrap();
}

#[test]
fn react_to_hooks_props_only_when_used_after() {
    test_rewrite(
        "test_patterns",
        "react_to_hooks",
        "props_only_when_used_after.tsx",
    )
    .unwrap();
}

#[test]
fn react_to_hooks_typed_props_only_when_used_after() {
    test_rewrite(
        "test_patterns",
        "react_to_hooks",
        "typed_props_only_when_used_after.tsx",
    )
    .unwrap();
}

#[test]
fn react_to_hooks_import_star() {
    test_rewrite("test_patterns", "react_to_hooks", "import_star.tsx").unwrap();
}

#[test]
fn react_to_hooks_render_at_end() {
    test_rewrite("test_patterns", "react_to_hooks", "render_at_end.tsx").unwrap();
}

#[test]
fn react_to_hooks_avoid_unrelated() {
    test_rewrite("test_patterns", "react_to_hooks", "avoid_unrelated.tsx").unwrap();
}

#[test]
fn match_test() {
    test_rewrite("test_patterns", "match", "match.tsx").unwrap();
}

#[test]
fn class_declaration_test() {
    test_rewrite("test_patterns", "class_declaration", "class.js").unwrap();
}

#[test]
fn plus_equal_test() {
    test_rewrite("test_patterns", "plus_equal", "main.js").unwrap();
}

#[test]
fn test_list_first_arg() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |call_expression(function="f", arguments=[$x, $y]) => `g($x)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   f(1, 2)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   g(1)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn implement_length() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`const $x = $_` where { $target = [1, 2, 3], $length = length($target) } => `const $x = $length`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = [1, 2, 3]
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const x = 3
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn inline_list_length() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`const $x = $_` where { $length = length(target=[7, 8, 9]) } => `const $x = $length`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = [1, 2, 3]
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const x = 3
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn string_length() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`"world"` as $w where { $length = length(target="hello") } => `$length`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const hello = "world";
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const hello = 5;
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn dont_swallow_rewrites() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |js"export default $susan" => `bob\n$susan`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |export default const foo
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |bob
                |const foo
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_linearization_medium() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |program() where {
                |    $program <: contains {
                |        $x where {
                |           $x <: `bar`,
                |           $x => `qux`
                |        }
                |     },
                |    $program <: contains { `foo(bar)` => `baz()` },
                |    $program <: contains { `blo` => `$x` }
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   foo(bar)
                |   flo(blo)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   baz()
                |   flo(qux)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_linearization_basic() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |or {
                |    `foo($x)` => `changed($x)`,
                |    `bar` => `baz`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"foo(of(bar))"#.to_string(),
            expected: r#"changed(of(baz))"#.to_string(),
        }
    })
    .unwrap();
}

#[test]
fn test_eager_builtin() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |or {
                |    `foo($x)` => text(`changed($x)`),
                |    `bar` => `baz`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"foo(of(bar))"#.to_string(),
            expected: r#"changed(of(bar))"#.to_string(),
        }
    })
    .unwrap();
}

#[test]
fn quote_agnostic_string_snippet() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"`"foo"` => `bar` "#.to_string(),
            source: r#"'foo'"#.to_string(),
            expected: r#"bar"#.to_string(),
        }
    })
    .unwrap();
}

#[test]
fn add_string_fragment_metavariables() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"`import $FROM from '$SOURCE'` => `import $FROM from '../$SOURCE'`"#
                .to_string(),
            source: r#"import Stuff from '../../hell'"#.to_string(),
            expected: r#"import Stuff from '../../../hell'"#.to_string(),
        }
    })
    .unwrap();
}

#[test]
fn quote_agnostic_string_snippet_reverse() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"`'foo'` => `bar` "#.to_string(),
            source: r#""foo""#.to_string(),
            expected: r#"bar"#.to_string(),
        }
    })
    .unwrap();
}

#[test]
fn even_more_linearized_test() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |program() where {
                |    $program <: contains {
                |       `foo($x)` where {
                |           $x <: `bar($y)`,
                |           $y => `qux`
                |       }
                |    },
                |    $program <: contains {
                |        `flo($a)` => `flo($x, $y)`
                |    }
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |foo(bar(baz))
                |flo(blo, fly)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |foo(bar(qux))
                |flo(bar(qux), qux)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn matching_var_snippet_work() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |contains `foo($x)` => `zum` where {
                |    $x <: `bar($_)`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"foo(bar(baz));"#.to_string(),
            expected: r#"zum;"#.to_string(),
        }
    })
    .unwrap();
}

#[test]
fn test_list_other_args() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |call_expression(function="f", arguments=[$x, $y, $z]) => `g($z, $y)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   f(1, 2, 3)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   g(3, 2)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_list_dots_first_arg() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |call_expression(function="f", arguments=[$x, ...]) => `g($x)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   f(1, 2, 3)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   g(1)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_list_dots_last_arg() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |call_expression(function="f", arguments=[..., $x]) => `g($x)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   f(1, 2, 3)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   g(3)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn import_with_regex() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`import $_ from "$x"` where {
                |   $x <: r"../(?:.+)" => `../$x`
                | }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |import Image from "../react";
                |import Other from "world";
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |import Image from "../../react";
                |import Other from "world";
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_js_comments() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |comment() => .
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log("hello")
                |// comment
                |console.log("world")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log("hello")
                |console.log("world")
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_js_comments_with_text() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |comment($content) where {
                |    $content <: or {" match this", " match this "}
                |} => .
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log("match")
                |// match this
                |// do not match this
                |// match this
                |// do not even // match this
                |// not this
                |/* match this */
                |/* do not match this */
                |console.log("world")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log("match")
                |// do not match this
                |// do not even // match this
                |// not this
                |/* do not match this */
                |console.log("world")
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_comments_with_regex() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |comment($content) where {
                |    $content <: r"(?: )match this(?:.*)"
                |} => .
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |// match this
                |// do not match this
                |// match this even with more content
                |// do not even // match this
                |// not this
                |/* match this */
                |/* do not match this */
                |/* match this with stuff after it */
                |console.log("world")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |// do not match this
                |// do not even // match this
                |// not this
                |/* do not match this */
                |console.log("world")
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_other_regex() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |comment($content) => . where { $content <: "@ts-nocheck" }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |//@ts-nocheck
                |const fs = require('fs');
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |
                |const fs = require('fs');
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_regex_with_method_call_rewrite_inside() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |pattern use_info($comp_name) {
                |    variable_declarator(name=$comp, $value, $type) where {
                |        $comp <: r"_(.+)"($comp_name),
                |        $value <: object() => `$comp_name`
                |    }
                |}
                |
                |use_info($comp_name)
                |
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const _DateSelect = {
                |    parameters: {
                |      info: 'This is a Select date Component',
                |    },
                |  };
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const _DateSelect = DateSelect;
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rust_mod_snippet() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language rust
                |
                |`println!($_)` as $ptln  where {
                |   $ptln <: within `#[cfg(test)] mod $foo { $bar }`,
                |   $ptln => .
                |}
                |
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |#[cfg(test)]
                |mod tests {
                |    #[test]
                |    fn hello() {
                |        println!("HELLO: {}", port);
                |    }
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |#[cfg(test)]
            |mod tests {
            |    #[test]
            |    fn hello() {
            |    }
            |}
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_regex_with_method_call_rewrite_outside() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |pattern use_info($comp_name) {
                |    variable_declarator(name=$comp, $value, $type) where {
                |        $comp <: r"_(.+)"($comp_name),
                |        $value <: object()
                |    }
                |}
                |
                |use_info($comp_name) => `FOUND$[comp_name]BEFORE`
                |
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const _DateSelect = {
                |    parameters: {
                |      info: 'This is a Select date Component',
                |    },
                |  };
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const FOUNDDateSelectBEFORE;
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn please_bind() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language js
                |`$name = $obj` where {
                |    $obj <: object($properties),
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                var foo = { x: "y" }"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn recursive_function_with_params() {
    run_test_match({
        TestArg {
            pattern: r#"
            pattern foo($a, $b) {
                `function foo($a, $b) { $bar }`
                where {
                    $bar <: not contains foo($b, $a)
                }
            }
            foo("bar", "baz")"#
                .to_owned(),
            source: r#"
            function foo(bar, baz) {
                "hello"
            }"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn test_list_first_and_last_arg() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |call_expression(function="f", arguments=[$x, ..., $y]) => `g($y, $x)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   f(1, 2, 3)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   g(3, 1)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_list_first_after() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |call_expression(function="f", arguments=[..., `3`, $x, ...]) => `g($x)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   f(1, 2, 3, 4, 5)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   g(4)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_list_complex() {
    run_test_expected({
            TestArgExpected {
                pattern: r#"
                |language js
                |
                |contains call_expression(function="f", arguments=[$x, ..., `3`, $y, ..., `7`, $z, ...]) => `g($x, $y, $z)`
                |"#
                .trim_margin()
                .unwrap(),
                source: r#"
                |   f(1, 2, 3, 4, 5, 6, 7, 8, 9)
                |"#
                    .trim_margin()
                    .unwrap(),
                expected: r#"
                |   g(1, 4, 8)
                |"#
                    .trim_margin()
                    .unwrap(),
            }
        })
        .unwrap();
}

#[test]
fn test_list_just_dots() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |call_expression(function="f", $arguments) where {
                |    $arguments <: [...],
                |    $arguments => `g($arguments)`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   f(1, 4, 8)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   f(g(1, 4, 8))
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_list_just_empty() {
    test_no_match("last_arg", "no_match.js").unwrap();
}

#[test]
fn test_list_bind_to_list() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |call_expression(function="f", arguments=$args) => `g($args)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   f(1, 2, 3)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   g(1, 2, 3)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_random() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`console.log($x)` where {
                |   $float = random(),
                |   $list = ["a", "a", "a"],
                |   $length = length($list),
                |   $max = $length - 1,
                |   $int = random(0, $max),
                |   $x => $list[$int]
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   console.log("fill")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   console.log(a)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_list_bind_to_empty_list() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |call_expression(function="f", arguments=$args) => `g($args)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   f()
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   g()
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_list_accumulator() {
    run_test_expected({
            TestArgExpected {
                pattern: r#"
                |language js
                |
                |pattern Foo() {
                |    $all = [],
                |    call_expression(function="f", arguments = some bubble($all) $x where $all += $x) => `g($all)`
                |}
                |
                |Foo()
                |"#
                .trim_margin()
                .unwrap(),
                source: r#"
                |   f(1, 2, 3)
                |"#
                    .trim_margin()
                    .unwrap(),
                expected: r#"
                |   g(1 2 3)
                |"#
                    .trim_margin()
                    .unwrap(),
            }
        })
        .unwrap();
}

#[test]
fn test_match_snippet_to_snippet() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |
                |`foo($x)` => `buz($x)` where {
                |  $y = `bar()`,
                |  $y <: $x
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   foo(bar())
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   buz(bar())
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_list_join() {
    run_test_expected({
            TestArgExpected {
                pattern: r#"
                |language js
                |
                |pattern Foo() {
                |    $all = [],
                |    call_expression(function="f", arguments = some bubble($all) $x where $all += $x) as $call where {
                |       $merged = join(list = $all, separator = ", "),
                |       $call => `g($merged)`
                |    }
                |}
                |
                |Foo()
                |"#
                .trim_margin()
                .unwrap(),
                source: r#"
                |   f(1, 2, 3)
                |"#
                    .trim_margin()
                    .unwrap(),
                expected: r#"
                |   g(1, 2, 3)
                |"#
                    .trim_margin()
                    .unwrap(),
            }
        })
        .unwrap();
}

#[test]
fn test_constructed_list_join() {
    run_test_expected({
            TestArgExpected {
                pattern: r#"
                |language js
                |
                |pattern Foo() {
                |    $all = [],
                |    call_expression(function="f", arguments = some bubble($all) $x where $all += $x) as $call where {
                |       $list = ["a string", `andSomeCode`],
                |       $merged = join($list, separator = ", "),
                |       $call => `g($merged)`
                |    }
                |}
                |
                |Foo()
                |"#
                .trim_margin()
                .unwrap(),
                source: r#"
                |   f(1, 2, 3)
                |"#
                    .trim_margin()
                    .unwrap(),
                expected: r#"
                |   g(a string, andSomeCode)
                |"#
                    .trim_margin()
                    .unwrap(),
            }
        })
        .unwrap();
}

#[test]
fn simple_until() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |contains bubble `foo($x)` => `bar($x)` until `foo($_)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   foo(another(foo(x)))
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   bar(another(foo(x)))
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn capitalize_no_args() {
    let pattern = r#"
        |language js
        |
        |`$foo: $bar` where {
        |   $Foo = capitalize(),
        |   $foo => `$Foo`
        |}
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
        |   let foo = { bar: 'baz' };
        |"#
    .trim_margin()
    .unwrap();

    let context = ExecutionContext::default();
    let tsx: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let res = src_to_problem(pattern, tsx).unwrap();
    let execution_result =
        res.execute_file(&RichFile::new("test-file.tsx".to_owned(), source), &context);
    let message = "No argument provided for capitalize function";
    assert!(execution_result.iter().any(|m| has_error(m, message)));
}

fn has_error(mr: &MatchResult, message: &str) -> bool {
    if let MatchResult::AnalysisLog(l) = mr {
        l.message == message
    } else {
        false
    }
}

#[test]
fn test_regex() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |identifier() as $i where {
                |    $i <: r"(foo)(bar)"($x, $y),
                |    $i => `$x, $y`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   foobar
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   foo, bar
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_lhs_regex() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |identifier() as $i where {
                |    $i <: r"(foo)(bar)"($x, $y),
                |    $x => `baz`,
                |    $y => `qux`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   foobar
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   bazqux
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_regex_no_submatch() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |identifier() as $i where {
                |    if ($i <: r"(foo)(bar)"($x, $y)) {
                |       $i => `$x, $y`
                |    } else {
                |       $i => `no_match`
                |    }
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   foobarbuz
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   no_match
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_some() {
    run_test_expected({
            TestArgExpected {
                pattern: r#"
                |language js
                |and {
                |    $all = [],
                |    bubble($all) $foo where {
                |        $foo <: call_expression(function="f", arguments = some bubble($all) $x where $all += $x),
                |        $res = join(list = $all, separator = ", "),
                |        $foo => `g($res)`
                |    }
                |}
                |"#
                .trim_margin()
                .unwrap(),
                source: r#"
                |   f(1, 2, 3)
                |"#
                    .trim_margin()
                    .unwrap(),
                expected: r#"
                |   g(1, 2, 3)
                |"#
                    .trim_margin()
                    .unwrap(),
            }
        })
        .unwrap();
}

#[test]
fn test_every() {
    run_test_expected(TestArgExpected {
        pattern: r#"
            |language js
            |`$f($args)` where {
            |    $args <: every bubble {
            |        $arg where {
            |            $arg => `5`,
            |            not $arg <: 4
            |        }
            |    }
            |}
            |"#
        .trim_margin()
        .unwrap(),
        source: r#"
            |   f(1, 2, 3)
            |   g(1, 2, 4, 3)
            |"#
        .trim_margin()
        .unwrap(),
        expected: r#"
            |   f(5, 5, 5)
            |   g(1, 2, 4, 3)
            |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap()
}

#[test]
fn test_pattern_match_constructed_list_some() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |pattern Foo() {
                |    $all = [],
                |    call_expression(function="foo", $arguments) as $call where {
                |       $arguments <: some bubble($all) $x where $all += $x,
                |       $all <: some `buz($the_buz)`,
                |       $call => `found($the_buz)`
                |    }
                |}
                |
                |Foo()
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   foo(bar(1), buz(2), bar(3))
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |  found(2)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_pattern_match_constructed_list_contains() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |pattern Foo() {
                |    $all = [],
                |    call_expression(function="foo", $arguments) as $call where {
                |       $arguments <: some bubble($all) $x where $all += $x,
                |       $all <: contains `buz($the_buz)`,
                |       $call => `found($the_buz)` //`found($the_buz)`
                |    }
                |}
                |
                |Foo()
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   foo(bar(1), bar(buz(2)), bar(3))
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |  found(2)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_css_stdlib() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language css
                |
                |`a { $props }` where {
                |  $props <: contains `aspect-ratio: $x` => `aspect-ratio: 3;`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |a {
                |  width: calc(100% - 80px);
                |  aspect-ratio: 1/2;
                |  font-size: calc(10px + (56 - 10) * ((100vw - 320px) / (1920 - 320)));
                |}
                |
                |#some-id {
                |  some-property: 5px;
                |}
                |
                |a.b ~ c.d {}
                |.e.f + .g.h {}
                |
                |@font-face {
                |  font-family: "Open Sans";
                |  src: url("/a") format("woff2"), url("/b/c") format("woff");
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |a {
                |  width: calc(100% - 80px);
                |  aspect-ratio: 3;
                |  font-size: calc(10px + (56 - 10) * ((100vw - 320px) / (1920 - 320)));
                |}
                |
                |#some-id {
                |  some-property: 5px;
                |}
                |
                |a.b ~ c.d {}
                |.e.f + .g.h {}
                |
                |@font-face {
                |  font-family: "Open Sans";
                |  src: url("/a") format("woff2"), url("/b/c") format("woff");
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_rewrite_predicate() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`$key: $value` where { $value => `MARKER` }
                |
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"x = { foo: 1 }"#.to_owned(),
            expected: r#"x = { foo: MARKER }"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn test_undefined_match() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |contains $x where {
                |    if ($y <: undefined) {
                |        $program => `MARKER`
                |    } else {
                |        $program => `NO_MATCH`
                |    }
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"x = { foo: 1 }"#.to_owned(),
            expected: r#"MARKER"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn test_undefined_no_match() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |contains $x where {
                |    if ($x <: undefined) {
                |        $program => `MARKER`
                |    } else {
                |        $program => `NO_MATCH`
                |    }
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"x = { foo: 1 }"#.to_owned(),
            expected: r#"NO_MATCH"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn string_with_comment() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |`console.log($arg);` => `// Some comment` where {
                |   $arg <: not within catch_clause()
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"console.log('foo');"#.to_owned(),
            expected: r#"// Some comment"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn no_console_log_lib() {
    run_test_expected_libs({
        TestArgExpectedWithLibs {
            pattern: r#"
                |language js
                |
                |no_console_log()
                |"#
            .trim_margin()
            .unwrap(),
            lib_patterns: vec![r#"
                |language js
                |pattern no_console_log() {
                |   `console.log($arg)` => `logger.log($arg)` where {
                |       $arg <: not within catch_clause()
                |   }
                |}
                |"#
            .trim_margin()
            .unwrap()],
            source: r#"console.log('foo');"#.to_owned(),
            expected: r#"logger.log('foo');"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn multiple_libs() {
    run_test_expected_libs({
        TestArgExpectedWithLibs {
            pattern: r#"
                |language js
                |
                |no_console_log()
                |"#
            .trim_margin()
            .unwrap(),
            lib_patterns: vec![
                r#"
                |language js
                |pattern no_console_log() {
                |   change_console_log()
                |}
                |"#
                .trim_margin()
                .unwrap(),
                r#"
                |language js
                |pattern change_console_log() {
                |   `console.log($arg)` => `logger.log($arg)` where {
                |       $arg <: not within catch_clause()
                |   }
                |}
                |"#
                .trim_margin()
                .unwrap(),
            ],
            source: r#"console.log('foo');"#.to_owned(),
            expected: r#"logger.log('foo');"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn add_operation_assignment() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"theme.spacing($x)" where {
                |   $y = $x + 8,
                |   $x => js"$y",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"theme.spacing(2)"#.to_owned(),
            expected: r#"theme.spacing(10)"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn add_operation_matching() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"fun_with_numbers($x, $y)" where {
                |   $y <: $x + 8,
                |   $x => js"$y",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"fun_with_numbers(-4, 4)"#.to_owned(),
            expected: r#"fun_with_numbers(4, 4)"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn subtract_operation_assignment() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"theme.spacing($x)" where {
                |   $y = $x - 8,
                |   $x => js"$y",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"theme.spacing(2)"#.to_owned(),
            expected: r#"theme.spacing(-6)"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn subtract_operation_matching() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"fun_with_numbers($x, $y)" where {
                |   $y <: $x - 5,
                |   $x => js"$y",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"fun_with_numbers(9, 4)"#.to_owned(),
            expected: r#"fun_with_numbers(4, 4)"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn mul_operation_assignment() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"theme.spacing($x)" where {
                |   $y = $x * 2,
                |   $x => js"$y",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"theme.spacing(3)"#.to_owned(),
            expected: r#"theme.spacing(6)"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn mul_operation_matching() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"fun_with_numbers($x, $y)" where {
                |   $y <: $x * 3,
                |   $x => js"$y",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"fun_with_numbers(4, 12)"#.to_owned(),
            expected: r#"fun_with_numbers(12, 12)"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn div_operation_assignment() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"theme.spacing($x)" where {
                |   $y = $x / 2,
                |   $x => js"$y",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"theme.spacing(-8)"#.to_owned(),
            expected: r#"theme.spacing(-4)"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn div_operation_matching() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"fun_with_numbers($x, $y)" where {
                |   $y <: $x / 3,
                |   $x => js"$y",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"fun_with_numbers(6, 2)"#.to_owned(),
            expected: r#"fun_with_numbers(2, 2)"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn mod_operation_assignment() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"theme.spacing($x)" where {
                |   $y = $x % 3,
                |   $x => js"$y",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"theme.spacing(8)"#.to_owned(),
            expected: r#"theme.spacing(2)"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn mod_operation_matching() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"fun_with_numbers($x, $y)" where {
                |   $x <: $y % 2,
                |   $x => js"$y",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"fun_with_numbers(1, 11)"#.to_owned(),
            expected: r#"fun_with_numbers(11, 11)"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn add_decimals() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"theme.spacing($x)" where {
                |   $y = $x + 8,
                |   $x => js"$y",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"theme.spacing(2.5)"#.to_owned(),
            expected: r#"theme.spacing(10.5)"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn divide_decimals() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"theme.spacing($x)" where {
                |   $y = $x / 2.5,
                |   $x => js"$y",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"theme.spacing(7.5)"#.to_owned(),
            expected: r#"theme.spacing(3)"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn subtract_decimals() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"fun_with_numbers($x, $y)" where {
                |   $y <: $x - 1.30,
                |   $x => js"$y",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"fun_with_numbers(2.44, 1.14)"#.to_owned(),
            expected: r#"fun_with_numbers(1.14, 1.14)"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn multiply_decimals() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"fun_with_numbers($x, $y)" where {
                |   $y <: $x * 1.5,
                |   $x => js"$y",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"fun_with_numbers(2, 3)"#.to_owned(),
            expected: r#"fun_with_numbers(3, 3)"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn js_repair_orphaned_arrow() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`console.$_($_)` => .
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const fn = () => console.log();
                |const fnTwo = () => { console.log(); };
                |const fnBob = () => { alert(); }
                |"#
            .trim_margin()
            .unwrap(),
            // Biome will handle formatting
            expected: r#"
                |const fn = () =>{} ;
                |const fnTwo = () => {  };
                |const fnBob = () => { alert(); }
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn python_removes_orphaned_type_arrow() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language python
                |
                |function_definition($name, $return_type) where {
                |  $name <: `foo`,
                |  $return_type => .
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |def foo() -> None:
                |    print('hi')
                |
                |def bar() -> SomeType:
                |    print('hello')
                |    return SomeType
                |"#
            .trim_margin()
            .unwrap(),
            // The whitespace is fine, because Ruff will remove it
            expected: r#"
                |def foo()  :
                |    print('hi')
                |
                |def bar() -> SomeType:
                |    print('hello')
                |    return SomeType
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn removes_orphaned_semicolon() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`console.log($x)` => .
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log('hi');
                |console.log('hello');
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#""#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn removes_orphaned_import_declaration() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |import_specifier($name) where or {
                |   $name <: `sumBy` => .,
                |   $name <: `orderBy` => .,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"import { orderBy, sumBy } from 'lodash';"#.to_owned(),
            expected: r#""#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn removes_orphans_sequential() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |pattern test_semicolon_1() {
                |   contains bubble lexical_declaration($kind, $declarations) where {
                |       $kind => .,
                |       $declarations => .,
                |   }
                |}
                |
                |pattern test_semicolon_2() {
                |   contains bubble `console.log($x)` => .
                |}
                |
                |pattern test_import_orphan() {
                |   contains bubble import_specifier($name) where or {
                |       $name <: `sumBy` => .,
                |       $name <: `orderBy` => .,
                |   }
                |}
                |
                |sequential {
                |   test_semicolon_1(),
                |   test_semicolon_2(),
                |   test_import_orphan(),
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |import { orderBy, sumBy } from 'lodash';
                |
                |const x = 6;
                |console.log('hi');
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |
                |
                |
                |
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn sequential_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
            engine marzano(0.1)
            language js

            pattern transform_test_modifier($from, $to) {
                `$testlike.$from` => `$testlike.$to` where {
                    $testlike <: or {
                        `describe`,
                        `it`,
                        `test`
                    }
                }
            }

            sequential {
                contains transform_test_modifier(from=`only`, to=`grit_only`),
                contains transform_test_modifier(from=`grit_only`, to=`skip`)
            }"#
            .to_owned(),
            source: r#"
                |test.only('Something to check', async () => {
                |    console.log('wow');
                |    console.error('hello world');
                |    expect(true).toBe(true);
                |})"#
                .trim_margin()
                .unwrap(),
            expected: r#"
            |test.skip('Something to check', async () => {
            |    console.log('wow');
            |    console.error('hello world');
            |    expect(true).toBe(true);
            |})"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn sequential_shared_state() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
            engine marzano(0.1)
            language js

            sequential {
                contains `$foo = "$bar"` => `$bar = "$foo"\nbaz = "qux"`,
                contains `$bar` => `$foo$foo`
            }"#
            .to_owned(),
            source: r#"
                |foo = "bar""#
                .trim_margin()
                .unwrap(),
            expected: r#"
                |foofoo = "foo"
                |baz = "qux""#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn test_multifile_pattern() {
    let pattern = "
engine marzano(0.1)
language js

pattern step1($x) {
    bubble($x) file($name, $body) where {
        $body <: contains `foo($x)`,
        $body <: contains `bar($x)` => `baz($x)`,
    }
}

pattern step2($x) {
    contains `baz($x)` => `qux($x)`
}

multifile {
    step1($x),
    step2($x)
}";
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let context = ExecutionContext::default();
    let pattern = src_to_problem(pattern.to_owned(), js_lang).unwrap();
    let results = pattern.execute_files(
        &[
            RichFile::new(
                "file1.tsx".to_string(),
                "foo(1)\nbar(1)\nbar(2)\nbaz(1)".to_string(),
            ),
            RichFile::new(
                "file2.tsx".to_string(),
                "foo(2)\nfoo(1)\nbar(1)\nbar(2)\nbaz(1)".to_string(),
            ),
        ],
        &context,
    );

    assert_yaml_snapshot!(results);
}

#[test]
fn multifile_propagates_scope_between_steps() {
    let pattern = "
engine marzano(0.1)
language js

multifile {
    bubble($x) file($name, $body) where $body <: contains `foo($x)`,
    bubble($x) file($name, $body) where $body <: contains `bar($x)` => `baz($x)`
}";
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let context = ExecutionContext::default();
    let pattern = src_to_problem(pattern.to_owned(), js_lang).unwrap();
    let results = pattern.execute_files(
        &[
            RichFile::new("file1.tsx".to_string(), "foo(1)".to_string()),
            RichFile::new("file2.tsx".to_string(), "bar(1)\nbar(3)".to_string()),
        ],
        &context,
    );

    assert_yaml_snapshot!(results);
}

#[test]
fn removes_orphans_sequential_pattern() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |pattern test_semicolon_1() {
                |   contains bubble lexical_declaration($kind, $declarations) where {
                |       $kind => .,
                |       $declarations => .,
                |   }
                |}
                |
                |pattern test_semicolon_2() {
                |   contains bubble `console.log($x)` => .
                |}
                |
                |pattern test_import_orphan() {
                |   contains bubble import_specifier($name) where or {
                |       $name <: `sumBy` => .,
                |       $name <: `orderBy` => .,
                |   }
                |}
                |
                |pattern remove_orphans() {
                |    sequential {
                |       test_semicolon_1(),
                |       test_semicolon_2(),
                |       test_import_orphan(),
                |    }
                |}
                |remove_orphans()
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |import { orderBy, sumBy } from 'lodash';
                |
                |const x = 6;
                |console.log('hi');
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |
                |
                |
                |
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn distinct_on_primitives() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |`const wow = [$yay]` where {
                |   $list = [1, 2, 3, 3, 2, 1],
                |   $distinct_list = distinct($list),
                |   $yay => `$distinct_list`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"const wow = [1];"#.to_owned(),
            expected: r#"const wow = [1,2,3];"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn correct_variable_index() {
    let pattern = "`function () { $body }`".to_owned();
    let tsx: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let _pattern =
        src_to_problem_libs(pattern, &BTreeMap::new(), tsx, None, None, None, None).unwrap();
}

#[test]
fn distinct_on_list_of_bindings() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |`const numbers = [$numbers]` where {
                |   $list = [],
                |   $numbers <: some bubble($list) $num where {
                |       $num <: number(),
                |       $list += $num,
                |   },
                |   $distinct = distinct($list),
                |   $joined = join(list=$distinct, separator=`,`),
                |   $numbers => $joined,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"const numbers = [1, 2, 3, 3, 2, 1];"#.to_owned(),
            expected: r#"const numbers = [1,2,3];"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn distinct_on_statements() {
    run_test_expected({
        TestArgExpected {
            pattern: "
                language js
                program($statements) where {
                   $list = [],
                   $statements <: some bubble($list) $s where {
                       $list += `$s`,
                   },
                   $distinct = distinct($list),
                   $joined = join(list=$distinct, separator=`\n`),
                   $statements => $joined,
                }
                "
            .to_owned(),
            source: r#"
                |const a = 6;
                |const b = 7;
                |const a = 6;
                |console.log(a + b);
                |console.log(a + b);
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const a = 6;
                |const b = 7;
                |console.log(a + b);
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn distinct_on_binding_of_list() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`const $foo = [$args]` where {
                |   $de = distinct(list=$args),
                |   $args => `$de`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"const foo = [1, 2, 3, 1]"#.to_owned(),
            expected: r#"const foo = [1,2,3]"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn rewrite_to_variable() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |js"remove_this_wrapper($x)" => $x
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"remove_this_wrapper('hello')"#.to_owned(),
            expected: r#"'hello'"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn test_basic_md() {
    let pattern = r#"
    |language markdown
    |
    |`[text](link)` => `[changed](link)`
    |"#
    .trim_margin()
    .unwrap();

    let source = r#"[text](link)"#.trim_margin().unwrap();
    let file = "foo.md";

    let context = ExecutionContext::default();
    let language: TargetLanguage = PatternLanguage::MarkdownInline.try_into().unwrap();
    let pattern = src_to_problem(pattern, language).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn md_link_metavariable() {
    let pattern = r#"
    |language markdown
    |
    |`[$text]($link)` => `[changed]($link)`
    |"#
    .trim_margin()
    .unwrap();

    let source = r#"[grit](https://app.grit.io)"#.trim_margin().unwrap();
    let file = "foo.md";

    let context = ExecutionContext::default();
    let language: TargetLanguage = PatternLanguage::MarkdownInline.try_into().unwrap();
    let pattern = src_to_problem(pattern, language).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn md_link_node() {
    let pattern = r#"
    |language markdown
    |
    |inline_link(identifier=link_text(text=link_text_non_empty($name)), $destination) => `[$destination]($name)`
    |"#
    .trim_margin()
    .unwrap();

    let source = r#"[grit](https://app.grit.io)"#.trim_margin().unwrap();
    let file = "foo.md";

    let context = ExecutionContext::default();
    let language: TargetLanguage = PatternLanguage::MarkdownInline.try_into().unwrap();
    let pattern = src_to_problem(pattern, language).unwrap();
    let results = pattern.execute_file(&RichFile::new(file.to_owned(), source), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn code_span() {
    let pattern = r#"
    |language markdown
    |
    |code_span($source) => $source
    |"#
    .trim_margin()
    .unwrap();

    let source = r#"This is `code`"#.trim_margin().unwrap();
    let file = "foo.md";

    let context = ExecutionContext::default();
    let language: TargetLanguage = PatternLanguage::MarkdownInline.try_into().unwrap();
    let pattern = src_to_problem(pattern, language).unwrap();
    let results =
        pattern.execute_file(&RichFile::new(file.to_owned(), source.to_owned()), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn markdown_heading_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: "
                language markdown(block)

                atx_heading($heading_content, $level) where {
                    $level <: or {
                        \"##\",
                        atx_h4_marker()
                    }
                } => `HEADING: $heading_content\n`
                "
            .to_owned(),
            source: r#"
                |# File with two secionds
                |Some content
                |## subheading
                |More content
                |## Subheading two
                |Even more content
                |### Subheading three
                |Even more content
                |#### Subheading four
                |Even more content
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |# File with two secionds
                |Some content
                |HEADING:  subheading
                |More content
                |HEADING:  Subheading two
                |Even more content
                |### Subheading three
                |Even more content
                |HEADING:  Subheading four
                |Even more content
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn markdown_sections() {
    run_test_expected({
        TestArgExpected {
            pattern: "
                language markdown(block)

                section($heading, $content) where {
                    $heading <: atx_heading(level=atx_h2_marker()),
                    $content <: not includes \"skip\"
                } => raw`---
                SECTION 2 HEADER: $heading`
                "
            .to_owned(),
            source: r#"
                |# File with two secionds
                |Some content
                |## level 2
                |More content
                |## level 2 (do not skip)
                |Even more content
                |### level 3
                |Even more content
                |#### level 4
                |Even more content
                |## Now we go back to section two
                |Content only here
                |## Skip this one...
                |skip me
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |# File with two secionds
                |Some content
                |---
                |                SECTION 2 HEADER: ## level 2
                |---
                |                SECTION 2 HEADER: ## level 2 (do not skip)
                |---
                |                SECTION 2 HEADER: ## Now we go back to section two
                |## Skip this one...
                |skip me
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn parses_java_constructor() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language java
                |
                |`new Seren($x)` => .
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"Seren thing = new Seren(1);"#.to_owned(),
            expected: r#"Seren thing = ;"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn parses_method_modifier() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language java
                |
                |modifier() as $m where {
                |   $m <: `public`,
                |   $m => `private`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"public void cool() { }"#.to_owned(),
            expected: r#"private void cool() { }"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn parses_method_body_metavariable() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language java
                |
                |`public void compute(int a) { $x }` where $x => .
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |class MyClass {
                |   public void compute(int a) {
                |       return a * bar + 42;
                |   }
                |}"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |class MyClass {
                |   public void compute(int a) {
                |   }
                |}"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn parses_class_body_metavariable() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language java
                |
                |`public class MyClass { $x }` where $x <: contains `foo` => `buz`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |public class MyClass {
                |   private int foo = 42;
                |}"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |public class MyClass {
                |   private int buz = 42;
                |}"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn allows_where_insert() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`new Thing({$params})` where { $params += `, nice: "bar"` }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"new Thing({ good: "foo" })"#.to_owned(),
            expected: r#"new Thing({ good: "foo", nice: "bar" })"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn allows_where_insert_twice() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`new Thing({$params})` where {
                |   $params += `, nice: "bar"`,
                |   $params += `, great: "baz"`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"new Thing({ good: "foo" })"#.to_owned(),
            expected: r#"new Thing({ good: "foo", nice: "bar", great: "baz" })"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn linearizes_rewrite_insertion_when_insert_after_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`new Thing({$params})` where {
                |   $params => `$params, great: "baz"`,
                |   $params += `, nice: "bar"`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"new Thing({ good: "foo" })"#.to_owned(),
            expected: r#"new Thing({ good: "foo", great: "baz", nice: "bar" })"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn linearizes_rewrite_insertion_when_insert_before_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`new Thing({$params})` where {
                |   $params += `, nice: "bar"`,
                |   $params => `$params, great: "baz"`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"new Thing({ good: "foo" })"#.to_owned(),
            expected: r#"new Thing({ good: "foo", great: "baz", nice: "bar" })"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn linearizes_multiple_insertions_with_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`new Thing({$params})` where {
                |   $params += `, nice: "bar"`,
                |   $params => `$params, great: "baz"`,
                |   $params += `, amazing: "biz"`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"new Thing({ good: "foo" })"#.to_owned(),
            expected: r#"new Thing({ good: "foo", great: "baz", nice: "bar", amazing: "biz" })"#
                .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn allows_pattern_insert() {
    run_test_expected({
        TestArgExpected {
            pattern: "
                language js

                `import $anything from 'foo'` += `\nimport { mylib } from 'bar'`
                "
            .to_owned(),
            source: r#"import { stuff } from 'foo'"#.to_owned(),
            expected: r#"
                |import { stuff } from 'foo'
                |import { mylib } from 'bar'
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn multiple_pattern_inserts() {
    run_test_expected({
        TestArgExpected {
            pattern: "
                language js

                and {
                   `import $anything from 'foo'` += `\nimport { mylib } from 'bar'`,
                   `import $more from 'foo'` += `\nimport { goodies } from 'baz'`
                }
                "
            .to_owned(),
            source: r#"import { stuff } from 'foo'"#.to_owned(),
            expected: r#"
                |import { stuff } from 'foo'
                |import { mylib } from 'bar'
                |import { goodies } from 'baz'
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn pattern_inserts_with_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: "
                language js

                and {
                   `import $anything from 'foo'` += `\nimport { mylib } from 'bar'`,
                   `import $more from 'foo'` += `\nimport { goodies } from 'baz'`,
                   `import $more from 'foo'` where {
                       $more => `{ forward }`
                   },
                }
                "
            .to_owned(),
            source: r#"import { stuff } from 'foo'"#.to_owned(),
            expected: r#"
                |import { forward } from 'foo'
                |import { mylib } from 'bar'
                |import { goodies } from 'baz'
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn inserts_with_contains() {
    run_test_expected({
        TestArgExpected {
            pattern: "
                language js

                function() as $f where $f <: contains lexical_declaration() += `\nconst y = 7;`
                "
            .to_owned(),
            source: r#"
                |function (x, y) {
                |   const x = 6;
                |   return new Value(x + y);
                |}"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |function (x, y) {
                |   const x = 6;
                |const y = 7;
                |   return new Value(x + y);
                |}"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn call_built_in_uppercase_on_rhs() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`i` as $i => uppercase(string=$i)
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"i"#.trim_margin().unwrap(),
            expected: r#"I"#.trim_margin().unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn call_built_in_length_on_rhs() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`i` as $i => length(target=$i)
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"i"#.trim_margin().unwrap(),
            expected: r#"1"#.trim_margin().unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn call_built_in_distinct_on_rhs() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |`const numbers = [$numbers]` where {
                |   $numbers => distinct(list=$numbers),
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"const numbers = [1, 2, 3, 3, 2, 1];"#.to_owned(),
            expected: r#"const numbers = [1,2,3];"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn join_on_binding_of_list() {
    run_test_expected({
        TestArgExpected {
            pattern: "
                language js

                program($statements) => join(list=$statements, separator=`\n\n`)
                "
            .to_owned(),
            source: r#"
                |const x = 6;
                |const y = 7;
                |const add = (a, b) => a + b;
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const x = 6;
                |
                |const y = 7;
                |
                |const add = (a, b) => a + b;
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn length_on_binding_of_list() {
    run_test_expected({
        TestArgExpected {
            pattern: "
                language js

                program($statements) where {
                   $length = length(target=$statements),
                   $statements += `\nconst length = $length;`,
                }
                "
            .to_owned(),
            source: r#"
                |const x = 6;
                |const y = 7;
                |const add = (a, b) => a + b;
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const x = 6;
                |const y = 7;
                |const add = (a, b) => a + b;
                |const length = 3;
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn parses_terraform_module() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language hcl
                |
                |`module "foo" {
                |    $args
                |}` => `module "custom" {
                |   source = "bar"
                |}`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |module "foo" {
                |   source = "bar"
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |module "custom" {
                |   source = "bar"
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rewrite_block_arg() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language hcl
                |
                |`module $mod {
                |    $args
                |}` where {
                |   $args <: contains `source = "terraform-aws-modules/rds/aws"` => `source = "better-place"`,
                |   $args <: contains `version = $_` => .
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |module "my_module" {
                |  for_each = local.instances
                |  source   = "terraform-aws-modules/rds/aws"
                |  version  = "2.0.0"
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |module "my_module" {
                |  for_each = local.instances
                |  source = "better-place"
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rewrite_arrow_function() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                engine marzano(0.1)
                language js
                `$args => $body` => `($args) => $body`"#
                .to_owned(),
            source: "(x) => x".to_owned(),
            expected: "(x) => x".to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn rewrite_to_function() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |function lines($string) {
                |    return split($string, separator=`\n`)
                |}
                |
                |function my_todo($target, $message) {
                |   if($message <: undefined) {
                |       $message = "This requires manual intervention."
                |   },
                |   $lines = lines(string = $message),
                |   $lines <: some bubble($result) $x where {
                |       if ($result <: undefined) {
                |            $result = `// TODO: $x`
                |        } else {
                |            $result += `\n// $x`
                |        }
                |   },
                |   $target_lines = lines(string = $target),
                |   $target_lines <: some bubble($result) $x where { $result += `\n// $x` },
                |   return $result,
                |}
                |
                |`module.exports = $_` as $x => my_todo(target=$x, message=`Fix this\nAnd that`)
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |module.exports = {
                |  king,
                |  queen: '8',
                |};
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |// TODO: Fix this
                |// And that
                |// module.exports = {
                |//   king,
                |//   queen: '8',
                |// };
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn assignment_with_function_value() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |function my_todo($target, $message) {
                |   if($message <: undefined) {
                |       $message = "This requires manual intervention."
                |   },
                |   return `// TODO: $message\n// $target`
                |}
                |
                |`console.log($x)` where {
                |   $ret_val = my_todo(target=$x),
                |   $x => `\n$ret_val\n`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log("foo")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log(
                |// TODO: This requires manual intervention.
                |// "foo"
                |)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn function_with_nested_conditions() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |function greet($person) {
                |   or {
                |       $person <: string($fragment) where { $you = $fragment },
                |       $you = "world"
                |   },
                |   return `"Hello $you"`
                |}
                |
                |`console.log($x)` where {
                |   $x => greet(person=$x),
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log("seren")
                |console.log(wow())
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log("Hello seren")
                |console.log("Hello world")
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn accumulate_with_function() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |function is_cool() {
                |   return ` is cool`,
                |}
                |
                |`console.log($x)` where {
                |   $x <: string($fragment) where {
                |       $fragment += is_cool(),
                |   },
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log("seren")
                |console.log(wow())
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log("seren is cool")
                |console.log(wow())
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn if_else_early_return() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |function print_boolean($value) {
                |   if ($value <: `true`) {
                |       return `"True"`,
                |   },
                |   return `"False"`,
                |}
                |
                |`console.log($x)` where {
                |   $x => print_boolean(value=$x),
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log(true)
                |console.log(false)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log("True")
                |console.log("False")
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn or_early_return() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |function print_boolean($value) {
                |   or {
                |       $value <: `false`,
                |       return `"True"`,
                |   },
                |   return `"False"`,
                |}
                |
                |`console.log($x)` where {
                |   $x => print_boolean(value=$x),
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log(true)
                |console.log(false)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log("True")
                |console.log("False")
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn and_early_return() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |function print_boolean($value) {
                |   and {
                |       $x = 3,
                |       log(variable=$x),
                |       return `"True"`,
                |   },
                |   return `"False"`,
                |}
                |
                |`console.log($x)` where {
                |   $x => print_boolean(value=$x),
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log(true)
                |console.log(false)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log("True")
                |console.log("True")
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn helper_function_in_ret_val() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |function print_true($value) {
                |   return `"$value is True"`,
                |}
                |
                |function print_boolean($value) {
                |   $x = 3,
                |   log(variable=$x),
                |   return print_true($value),
                |}
                |
                |`console.log($x)` where {
                |   $x => print_boolean(value=$x),
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log(true)
                |console.log(false)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log("true is True")
                |console.log("false is True")
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn helper_function_in_body() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |function print_true($value) {
                |   log(variable=$x),
                |   return `"$value is True"`,
                |}
                |
                |function print_boolean($value) {
                |   $x = print_true($value),
                |   print_true($value),
                |   return $x,
                |}
                |
                |`console.log($x)` where {
                |   $x => print_boolean(value=$x),
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log(true)
                |console.log(false)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log("true is True")
                |console.log("false is True")
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn empty_variable_matches_false() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |arrow_function($parenthesis) as $f where {
                |   $parenthesis <: false,
                |   $y <: false,
                |   $z = false,
                |   $z <: false,
                |   $f => .,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |x => {
                |   return x * y;
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#""#.trim_margin().unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn bound_variable_matches_true() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |arrow_function($parenthesis) as $f where {
                |   $parenthesis <: true,
                |   $y = "false",
                |   $y <: true,
                |   $f => .,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |(x) => {
                |   return x * y;
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#""#.trim_margin().unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn zero_matches_false() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |js"fun_with_numbers($x)" => . where {
                |   $z = $x - 8,
                |   $z <: false,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"fun_with_numbers(8)"#.to_owned(),
            expected: r#""#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn empty_list_and_string_match_false() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |js"fun_with_numbers($x)" => . where {
                |   $z = [],
                |   $z <: false,
                |   $y = "",
                |   $y <: false,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"fun_with_numbers(8)"#.to_owned(),
            expected: r#""#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn non_empty_list_matches_true() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |call_expression($arguments) as $c where {
                |   $numbers = [],
                |   $arguments <: contains bubble($numbers) number() as $arg where {
                |       $numbers += $arg,
                |   },
                |   $numbers <: true,
                |   $c => .,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"fun_with_numbers(8, 7, 6, 5)"#.to_owned(),
            expected: r#""#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn matches_arrow_function_parameters() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |js"createParamDecorator($factory)" where {
                |   $factory <: contains arrow_function($parameters, $body) where {
                |       $parameters => .
                |   },
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"createParamDecorator(async (data: Options, name: string) => { console.log(name); })"#.to_owned(),
            expected: r#"createParamDecorator(async () => { console.log(name); })"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn call_foreign_js_function() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language python
                |
                |function custom_adder() js {
                |  // heres a comment {}"'`
                |  /* and heres another {} "'` */
                |  return 9
                |}
                |
                |`print($x)` where { $x => custom_adder() }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |print("foo")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |print(9)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn return_a_string_from_foreign_js_function() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language python
                |
                |function custom_adder() js {
                |  // heres a comment {}"'`
                |  /* and heres another {} "'` */
                |  return "\"{}\""
                |}
                |
                |`print($x)` where { $x => custom_adder() }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |print("foo")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |print("{}")
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn return_a_single_quote_string_from_foreign_js_function() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language python
                |
                |function custom_adder() js {
                |  // heres a comment {}"'`
                |  /* and heres another {} "'` */
                |  return '\'{}\''
                |}
                |
                |`print($x)` where { $x => custom_adder() }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |print("foo")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |print('{}')
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn return_a_template_string_from_foreign_js_function() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language python
                |
                |function custom_adder() js {
                |  // heres a comment {}"'`
                |  /* and heres another {} "'` */
                |  return `\`{}\``
                |}
                |
                |`print($x)` where { $x => custom_adder() }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |print("foo")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |print(`{}`)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn call_foreign_js_function_with_bracket_in_comment() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |function custom_func() js {
                |  console.log('hello')
                |  // {}
                |  /*
                |   {} */ return "Hi!"
                |}
                |
                |`console.log($x)` where { $x => custom_func() }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log(foo)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log(Hi!)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn call_foreign_js_function_with_nested_brackets() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language python
                |
                |function custom_adder() js {
                |    total = 0
                |    for (i = 0; i < 10; i++) {
                |        for (j = 0; j < 10; j++) {
                |            total += 1;
                |        }
                |    }
                |    return total
                |}
                |
                |`print($x)` where { $x => custom_adder() }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |print("foo")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |print(100)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn call_foreign_js_function_with_nested_brackets_and_arg() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |function add_prefixes($description) js {
                |   const words = $description.text.split(' ');
                |   const firstWith = words.findIndex((w) => w.startsWith('@'));
                |   for (let x = firstWith + 1; x < words.length; x++) {
                |       if (!words[x].startsWith('@')) {
                |           words[x] = '@' + words[x]
                |       }
                |   }
                |   return words.join(' ')
                |}
                |
                |`test($description, $_)` where {
                |   $description => add_prefixes($description),
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |test('this is a nice test @1 Project Studio Nice @hey', () => {});
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |test('this is a nice test @1 @Project @Studio @Nice @hey', () => {});
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn call_foreign_js_function_with_args() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language python
                |
                |function custom_func($x) js {
                |  return "Hi!" + $x.text
                |}
                |
                |`print($x)` where { $x => custom_func($x) }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |print(foo)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |print(Hi!foo)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn call_foreign_js_function_with_args_in_any_order() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language python
                |
                |function custom_func($x, $msg) js {
                |  return $msg.text + " ... " + $x.text
                |}
                |
                |`print($x)` where { $msg = `hello`, $x => custom_func($msg, $x) }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |print(foo)
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |print(hello ... foo)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn yaml_automatically_adds_newline() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language yaml
                |
                |`resource_types: $x` where { $x += `- wow: cool
                |  nice: ok` }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |resource_types:
                |- name: pull-request
                |  source:
                |    repository: thing
                |  type: registry-image
                |- name: mend"#
                .trim_margin()
                .unwrap(),
            expected: r#"
                |resource_types:
                |- name: pull-request
                |  source:
                |    repository: thing
                |  type: registry-image
                |- name: mend
                |- wow: cool
                |  nice: ok"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn yaml_multiple_insert() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language yaml
                |
                |`resource_types: $x` where { $x += `- wow: cool
                |  nice: ok`, $x += `- yay: great
                |  nice: ok` }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |resource_types:
                |- name: pull-request
                |  source:
                |    repository: thing
                |  type: registry-image
                |- name: mend"#
                .trim_margin()
                .unwrap(),
            expected: r#"
                |resource_types:
                |- name: pull-request
                |  source:
                |    repository: thing
                |  type: registry-image
                |- name: mend
                |- wow: cool
                |  nice: ok
                |- yay: great
                |  nice: ok"#
                .trim_margin()
                .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn yaml_no_extra_newlines() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language yaml
                |
                |`resource_types: $x` where { $x += `- wow: cool
                |  nice: ok` }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |resource_types:
                |- name: pull-request
                |  source:
                |    repository: thing
                |  type: registry-image
                |- name: mend
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |resource_types:
                |- name: pull-request
                |  source:
                |    repository: thing
                |  type: registry-image
                |- name: mend
                |- wow: cool
                |  nice: ok
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn js_automatically_adds_newline() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |program($statements) where {
                |  $length = length(target=$statements),
                |  $statements += `const length = $length;`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = 6;
                |const y = 7;
                |const add = (a, b) => a + b;
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const x = 6;
                |const y = 7;
                |const add = (a, b) => a + b;
                |const length = 3;
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn js_automatically_adds_newline_for_single_statement() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |or {
                |  `const $new = $val` += `console.log("variable $new initialized with value $val");`,
                |  `someDict.lookup` += `.super`,
                |  `$x = $y ` += ` + 1`
                |  }
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const y = 7;
                |someDict.lookup(hello);
                |y = 4;
                |for(let x = 0; x < 10; x++ ) { }
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const y = 7 + 1;
                |console.log("variable y initialized with value 7");
                |someDict.lookup.super(hello);
                |y = 4 + 1;
                |for(let x = 0 + 1; x < 10; x++ ) { }
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn adds_newline_for_matcher_insert() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |program($statements) where {
                |   $length = length(target=$statements),
                |   $statements <: true += `const length = $length;`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |diffs.push({
                |   before: allCodeBlocks[i],
                |   after: allCodeBlocks[i + 1],
                |});
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |diffs.push({
                |   before: allCodeBlocks[i],
                |   after: allCodeBlocks[i + 1],
                |});
                |const length = 1;
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn adds_newlines_for_matcher_multiple_insert() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |program($statements) where {
                |   $length = length(target=$statements),
                |   $statements <: true += `const length = $length;`,
                |   $statements <: true += `console.log(length)`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |diffs.push({
                |   before: allCodeBlocks[i],
                |   after: allCodeBlocks[i + 1],
                |});
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |diffs.push({
                |   before: allCodeBlocks[i],
                |   after: allCodeBlocks[i + 1],
                |});
                |const length = 1;
                |console.log(length)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn does_not_add_newline_for_single_multiline_node() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |call_expression() as $x += `;`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |diffs.push({
                |   before: allCodeBlocks[i],
                |   after: allCodeBlocks[i + 1],
                |})
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |diffs.push({
                |   before: allCodeBlocks[i],
                |   after: allCodeBlocks[i + 1],
                |});
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn js_automatically_adds_comma() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`($x) => { $_ }` where {
                |   $x += `a`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |var times = (x, y, z) => {
                |   return x * y * z;
                |};
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |var times = (x, y, z, a) => {
                |   return x * y * z;
                |};
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn js_multiple_insert_with_trailing_comma() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`($x) => { $_ }` where {
                |   $x += `a`,
                |   $x += `b`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |var times = (x, y, z,) => {
                |   return x * y * z;
                |};
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |var times = (x, y, z, a, b,) => {
                |   return x * y * z;
                |};
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn json_smart_insert_newline_and_comma() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language json
                |
                |`{ $properties }` where {
                |   $properties += `"hey": "wow"`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |{
                |   "extends": "./tsconfig.json",
                |   "exclude": ["**/*.spec.ts"],
                |   "include": ["**/*.ts"]
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |{
                |   "extends": "./tsconfig.json",
                |   "exclude": ["**/*.spec.ts"],
                |   "include": ["**/*.ts"],
                |   "hey": "wow"
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn js_respects_blanket_grit_disable() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`console.log($x)` => .
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |// grit-ignore
                |console.log('hello');
                |console.log('hi');
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |// grit-ignore
                |console.log('hello');
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rust_respects_blanket_grit_disable() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language rust
                |
                |`println!($_)` => .
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
fn main() {
    if cfg!(target_os = "linux") {
        // grit-ignore
        println!("cargo:rustc-link-lib=stdc++");
        // grit-ignore
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=c");
    } else if cfg!(target_os = "windows") {
        // Does not work yet
        // grit-ignore
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}"#
            .to_owned(),
            expected: r#"
fn main() {
    if cfg!(target_os = "linux") {
        // grit-ignore
        println!("cargo:rustc-link-lib=stdc++");
        // grit-ignore
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if cfg!(target_os = "windows") {
        // Does not work yet
        // grit-ignore
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}"#
            .to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn jsx_respects_blanket_grit_disable() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`this.foo` => `this.bar`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const Component = () => {
                |   return (
                |   <div>
                |       {/* grit-ignore */}
                |       <button>this.foo</button>
                |       <p>this.foo</p>
                |   </div>)
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const Component = () => {
                |   return (
                |   <div>
                |       {/* grit-ignore */}
                |       <button>this.foo</button>
                |       <p>this.bar</p>
                |   </div>)
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn python_respects_blanket_grit_disable() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language python
                |
                |`print($x)` => `log($x)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |print('hello')
                |print('hi') #grit-ignore
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |log('hello')
                |print('hi') #grit-ignore
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn inline_ignore_does_not_disable_next_line() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language python
                |
                |`print($x)` => `log($x)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |print('hello') #grit-ignore
                |print('hi')
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |print('hello') #grit-ignore
                |log('hi')
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn jsx_respects_stacked_ignores() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`this.foo` => `this.bar`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const Component = () => {
                |   // grit-ignore
                |   // eslint-disable-next-line
                |   this.foo = 3;
                |   return (
                |   <div>
                |       {/* grit-ignore */}
                |       {/* eslint-disable-next-line */}
                |       <button>this.foo</button>
                |       <p>this.foo</p>
                |   </div>)
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const Component = () => {
                |   // grit-ignore
                |   // eslint-disable-next-line
                |   this.foo = 3;
                |   return (
                |   <div>
                |       {/* grit-ignore */}
                |       {/* eslint-disable-next-line */}
                |       <button>this.foo</button>
                |       <p>this.bar</p>
                |   </div>)
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn python_respects_stacked_ignores() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language python
                |
                |`print($x)` => `log($x)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |print('hello')
                |# grit-ignore
                |# pylint: disable=missing-docstring
                |print('hi')
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |log('hello')
                |# grit-ignore
                |# pylint: disable=missing-docstring
                |print('hi')
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn hcl_respects_suppression() {
    run_test_no_match({
        TestArg {
            pattern: r#"
                |language hcl
                |
                |`module "foo" {
                |    $args
                |}` => `module "custom" {
                |   source = "bar"
                |}`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |/*
                |grit-ignore: We don't want this to match
                |*/
                |module "foo" {
                |   source = "bar"
                |}
                |# grit-ignore: Ignore this too
                |module "foo" {
                |   source = "bar"
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn css_respects_suppression() {
    run_test_no_match({
        TestArg {
            pattern: r#"
                |language css
                |
                |`a { $props }`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |/* grit-ignore */
                |a {
                |  width: calc(100% - 80px);
                |  aspect-ratio: 1/2;
                |  font-size: calc(10px + (56 - 10) * ((100vw - 320px) / (1920 - 320)));
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn java_respects_suppression() {
    run_test_no_match({
        TestArg {
            pattern: r#"
                |language java
                |
                |modifier() as $m where {
                |   $m <: `public`,
                |   $m => `private`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |/* grit-ignore */
                |public class Grit {
                |   // grit-ignore: Do nothing
                |   public static void main(String[] args) {
                |       System.out.println("Hello, World!");
                |   }
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn solidity_respects_suppression() {
    run_test_no_match({
        TestArg {
            pattern: r#"
                |language sol
                |pattern Loop($body) {
                |    or {
                |        `while($_) { $body }`,
                |        `for ($_; $_; $_) { $body }`
                |    }
                |}
                |
                |Loop($body)
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |pragma solidity ^0.8.9;
                |contract HelloWorld {
                |    string public greet = "Hello World!";
                |
                |    function foo(string memory _greet) public {
                |        // grit-ignore     :  leave some whitespace too
                |        while(other) {
                |            greet = foo(bar);
                |            while(foo) {  /** grit-ignore: not this either **/
                |                greet = foo(bar);
                |            }
                |        }
                |    }
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn does_not_delete_comma_after_type_annotation() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |type_annotation() => .
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |async function doStuff(
                |   _parent: unknown,
                |   { channelId, provider }: RequestParams,
                |   { group }: Context
                |) {}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |async function doStuff(
                |   _parent,
                |   { channelId, provider },
                |   { group }
                |) {}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn adjusts_padding_with_carriage_return() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language python
                |
                |module($statements) where $statements <: maybe contains bubble `openai.$field = $val` as $s where {
                |   $res = `raise Exception("The 'openai.$field' option isn't read in the client API")`,
                |   $s => $res,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: "thanksgiving = cool\r
# 🦃🦃🦃\r
hey = nice\r
# The next line says This is the code\r
# 這是程式碼\r
nice = things\r
\r
# The next line says another test\r
# 另一個測試\r
openai.a = a\r
\r
🥑🥑🥑\r
🦃🦃\r
# 🥑🥑🥑\r
key_array = openai_apikey.split(',')\r
                "
            .to_string(),
            expected: "thanksgiving = cool\r
# 🦃🦃🦃\r
hey = nice\r
# The next line says This is the code\r
# 這是程式碼\r
nice = things\r
\r
# The next line says another test\r
# 另一個測試\r
raise Exception(\"The 'openai.a' option isn't read in the client API\")\r
\r
🥑🥑🥑\r
🦃🦃\r
# 🥑🥑🥑\r
key_array = openai_apikey.split(',')\r
                "
            .to_string(),
        }
    })
    .unwrap();
}

#[test]
fn built_in_allows_unnamed_ordered_args() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |`const numbers = [$numbers]` where {
                |   $our_list = [],
                |   $numbers <: some bubble($our_list) $num where {
                |       $num <: number(),
                |       $our_list += $num,
                |   },
                |   $distinct = distinct($our_list),
                |   $joined = join($distinct, `,`),
                |   $numbers => $joined,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"const numbers = [1, 2, 3, 3, 2, 1];"#.to_owned(),
            expected: r#"const numbers = [1,2,3];"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn built_in_allows_named_unordered_args() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |`const numbers = [$numbers]` where {
                |   $our_list = [],
                |   $numbers <: some bubble($our_list) $num where {
                |       $num <: number(),
                |       $our_list += $num,
                |   },
                |   $distinct = distinct($our_list),
                |   $joined = join(separator=`,`, list=$distinct),
                |   $numbers => $joined,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"const numbers = [1, 2, 3, 3, 2, 1];"#.to_owned(),
            expected: r#"const numbers = [1,2,3];"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn user_defined_allows_unnamed_ordered_args() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |function my_todo($target, $message) {
                |   if($message <: undefined) {
                |       $message = "This requires manual intervention."
                |   },
                |   return `// TODO: $message\n// $target`
                |}
                |
                |`console.log($x)` where {
                |   $ret_val = my_todo($x, `this is cool`),
                |   $x => `\n$ret_val\n`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log("foo")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log(
                |// TODO: this is cool
                |// "foo"
                |)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn user_defined_allows_named_unordered_args() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |function my_todo($target, $message) {
                |   if($message <: undefined) {
                |       $message = "This requires manual intervention."
                |   },
                |   return `// TODO: $message\n// $target`
                |}
                |
                |`console.log($x)` where {
                |   $ret_val = my_todo(message=`this is cool`, target=$x),
                |   $x => `\n$ret_val\n`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log("foo")
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |console.log(
                |// TODO: this is cool
                |// "foo"
                |)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn removes_escape_backslash_from_snippet() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |or {
                |   `console.log($arg)` => `// \`Do not console log\`` where {
                |      $arg <: not within catch_clause(),
                |   },
                |   `console.error($arg)` => js"\" Do not console error",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |// Do not remove this
                |console.error('foo');
                |console.log('foo');
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |// Do not remove this
                |" Do not console error;
                |// `Do not console log`;
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn processes_escaped_escape_character() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |or {
                |   `console.log($arg)` => `\\\\ Do not console log` where {
                |      $arg <: not within catch_clause(),
                |   },
                |   `console.error($arg)` => js"// \\ Do not console error",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |// Do not remove this
                |console.error('foo');
                |console.log('foo');
                |"#
            .trim_margin()
            .unwrap(),
            expected: r"
                |// Do not remove this
                |// \ Do not console error;
                |\\ Do not console log;
                |"
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn escapes_dollar_sign_in_snippet() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |or {
                |   `console.log($arg)` => `console.log(\$$arg)`,
                |   `console.error($arg)` => js"console.error(\$$arg)",
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log(hey);
                |console.error(hey);
                |"#
            .trim_margin()
            .unwrap(),
            expected: r"
                |console.log($hey);
                |console.error($hey);
                |"
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn removes_error_commas_in_class_body() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`export default { $properties }` => `export default class MyClass {
                |   $properties
                |}`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |export default {
                |   sayHi() {
                |       console.log('hi');
                |   },
                |   async doSomething() {
                |       console.log('hello');
                |   },
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |export default class MyClass {
                |   sayHi() {
                |       console.log('hi');
                |   }
                |   async doSomething() {
                |       console.log('hello');
                |   }
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn resolves_map() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`const x = $foo` where {
                |   $bar = `"bar"`,
                |   $my_x = {
                |       foo: $bar,
                |   },
                |   $foo => $my_x,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = "hi";
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const x = {"foo": "bar"};
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn access_empty_map() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`const x = $foo` as $stmt where {
                |   $false = false,
                |   $false <: {}.gascony,
                |   $albania = {}.albania,
                |   $albania <: undefined,
                |   $stmt => $albania,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = right;
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#""#.trim_margin().unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn access_populated_map() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`const $x = $foo` where {
                |   $capitals = { england: "london", france: `paris` },
                |   $english = $capitals.england,
                |   $x => capitalize($english),
                |   $french = $capitals.france,
                |   $french += { city: `marseilles` }.city,
                |   $foo => $french,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = hello;
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const London = parismarseilles;
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn match_on_falsy_accessor() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`const x = $foo` as $stmt where {
                |   {}.gascony <: false,
                |   {}.albania <: undefined,
                |   $capitals = { italy: `"rome"` },
                |   $capitals.austria <: undefined,
                |   $stmt => .,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = right;
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#""#.trim_margin().unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn match_on_truthy_accessor() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`const x = $foo` where {
                |   { hi: $foo }.hi <: `right`,
                |   { clyde: 6, wales: 5 }.clyde <: 3 * 2,
                |   $kiel = `"kiel"`,
                |   $cities = { germany: $kiel, italy: `"venice"` },
                |   $cities.germany <: $kiel,
                |   $cities.italy <: `"venice"`,
                |   $foo => $kiel,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = right;
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const x = "kiel";
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rewrite_to_accessor() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`const $x = $foo` where {
                |   $kiel = `kiel`,
                |   $cities = { germany: $kiel, italy: `"venice"` },
                |   $x => capitalize($cities.germany),
                |   $foo => $cities.italy,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = right;
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const Kiel = "venice";
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rewrite_to_accessor_mut() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`const $x = $foo` where {
                |   $cities = {},
                |   $kiel = `kiel`,
                |   $cities.germany = $kiel,
                |   $cities.italy = `"venice"`,
                |   $x => capitalize($cities.germany),
                |   $foo => $cities.italy,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = right;
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const Kiel = "venice";
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn variable_accessor() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language yaml
                |pattern distribute_variables() {
                |    $vars_map = {},
                |    `across: $vars` as $across where {
                |        $vars <: contains bubble($vars_map) `var: $name
                |values: $vals` where {
                |            $val_list = [],
                |            $vals <: some bubble($val_list, $vars_map) `- $name` where {
                |                $val_list += $name
                |            },
                |            $vars_map.$name = $val_list
                |        },
                |        $across => $vars_map
                |    }
                |}
                |
                |distribute_variables()
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |jobs:
                |  - across:
                |    - var: qux
                |      values:
                |      - foo
                |      - bar
                |      - baz
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |jobs:
                |  - {"qux": foo,bar,baz}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn some_every_over_map() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |$foo where {
                |    $bar = { a: "1",  b: "2", c: "3" },
                |    $baz = { d: "1",  e: "2", f: "4" },
                |    $list = [],
                |    $bar <: some bubble($list) [$x, $y] where {
                |        not $y <: `3`,
                |        $list += $x
                |    },
                |    maybe $bar <: every bubble($list) [$x, $y] where {
                |        not $y <: `3`,
                |        $list += $x
                |    },
                |    $baz <: every bubble($list) [$x, $y] where {
                |        not $y <: `3`,
                |        $list += $x
                |    },
                |    $foo => $list
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |foo
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |a,b,d,e,f
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn some_every_over_map_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |`$a + $b` where {
                |    $map = {},
                |    $map.$a = $b,
                |    $map <: some bubble [$x, $y] where { $y => `2`}
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |4 + 5
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |4 + 2
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn some_every_errors_on_invalid_rewrite() {
    let pattern = r#"
        |language js
        |`$a + $b` where {
        |    $map = {},
        |    $map.$a = $b,
        |    $map <: some bubble [$x, $y] where { $x => `1`, $y => `2`}
        |}
        |"#
    .trim_margin()
    .unwrap();

    let source = r#"
        |4 + 5
        |"#
    .trim_margin()
    .unwrap();

    let file = "test-file.tsx";

    let context = ExecutionContext::default();
    let js_lang: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
    let pattern = src_to_problem(pattern, js_lang).unwrap();
    let results =
        pattern.execute_file(&RichFile::new(file.to_owned(), source.to_owned()), &context);
    assert_yaml_snapshot!(results);
}

#[test]
fn access_empty_list() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`const x = $foo` as $stmt where {
                |   $false = false,
                |   $false <: [][0],
                |   $foo = [][1],
                |   $foo <: undefined,
                |   $stmt => $foo,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = right;
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#""#.trim_margin().unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn access_populated_list() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`const $x = $foo` where {
                |   $capitals = ["london", `paris`],
                |   $english = $capitals[0],
                |   $x => capitalize($english),
                |   $french = $capitals[-1],
                |   $french += [`marseilles`][-1],
                |   $foo => $french,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = hello;
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const London = parismarseilles;
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn match_on_falsy_list_index() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`const x = $foo` as $stmt where {
                |   [][0] <: false,
                |   [][1] <: undefined,
                |   $capitals = [`"rome"`],
                |   $capitals[-2] <: undefined,
                |   $stmt => .,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = right;
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#""#.trim_margin().unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn before_list_index() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`[ $list ]` where {
                |    $new = before $list[1]
                |} => $new
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = ["one", "two", "three"]
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"const x = "one""#.trim_margin().unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn after_list_index() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`[ $list ]` where {
                |    $new = after $list[1]
                |} => $new
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = ["one", "two", "three"]
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"const x = "three""#.trim_margin().unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn after_map_index() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |`"two"` as $a where {
                |   $map = { a: $a },
                |   $new = after $map.a
                |} => $new
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = ["one", "two", "three"]
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"const x = ["one", "three", "three"]"#.trim_margin().unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn match_on_truthy_index() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`const x = $foo` where {
                |   [$foo][-1] <: `right`,
                |   [6, 5][-2] <: 3 * 2,
                |   $kiel = `"kiel"`,
                |   $cities = [$kiel, `"venice"`],
                |   $cities[0] <: $kiel,
                |   $cities[1] <: `"venice"`,
                |   $foo => $kiel,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = right;
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const x = "kiel";
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rewrite_to_index() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`const $x = $foo` where {
                |   $kiel = `kiel`,
                |   $cities = [$kiel, `"venice"`],
                |   $x => capitalize($cities[0]),
                |   $foo => $cities[1],
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = right;
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const Kiel = "venice";
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn accesses_binding_list() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`const $_ = new Data([$data])` where {
                |   $data[0] <: `"apple"`,
                |   $data => `"pear"`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = new Data(["apple", "mango", "watermelon"]);
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const x = new Data(["pear"]);
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn accesses_binding_list_with_negative_index() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`const $_ = new Data([$data])` where {
                |   $data[-1] <: `"watermelon"`,
                |   $data => `"pear"`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = new Data(["apple", "mango", "watermelon"]);
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const x = new Data(["pear"]);
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn accesses_binding_list_undefined() {
    run_test_no_match({
        TestArg {
            pattern: r#"
                |language js
                |
                |`const $_ = new Data([$data])` where {
                |   $data[-4] <: `"watermelon"`,
                |   $data => `"pear"`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const x = new Data(["apple", "mango", "watermelon"]);
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn parses_generic_within_type_query() {
    run_test_no_match({
        TestArg {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |
                |member_expression(property = $p) where $p => $p
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |export function MyComponent({
                |   why,
                |   wow,
                |   ...options
                |}: Pick<
                |   Parameters<typeof useHook<SomeType>>[0],
                |   'why' | 'wow'
                |>): JSX.Element {
                |   const { something } = useHook();
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn binds_python_decorator_argument() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language python
                |
                |`@pytest.mark.skip($_)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |@pytest.mark.skip(reason="no way of currently testing this")
                |def test_the_unknown():
                |   pass
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn trailing_comma_import() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`import $clause from $_` where {
                |    $clause <:
                |        import_clause($name) where {
                |        $name => .
                |    }
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |import ShouldNotBeRemoved, { fetch } from 'node-fetch';
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |import ShouldNotBeRemoved  from 'node-fetch';
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn yaml_string() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language yaml
                |file(body = and {
                |    contains `runs-on: ubuntu-latest`,
                |    contains `runs-on: "ubuntu-old"`
                |})"#
                .trim_margin()
                .unwrap(),
            source: r#"
                |foo:
                |    runs-on: "ubuntu-latest"
                |    runs-on: ubuntu-old
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn yaml_list_indentation() {
    run_test_expected({
        TestArgExpected {
            pattern: r"
                |engine marzano(0.1)
                |language yaml
                |
                |`- $task` where {
                |    $task <: block_mapping($items),
                |    $subtasks = [],
                |    $items <: some block_mapping_pair(key=contains `across`, $value),
                |    $value <: contains `values: $values`,
                |    $values <: some bubble($items, $subtasks) `- $val` where {
                |        $new_task = [],
                |        $items <: some bubble($new_task) $_ where {
                |                $new_task += `foo: other`,
                |            },
                |        $bar = join(list=$new_task, separator=`\n `),
                |        $subtasks += `- $bar`
                |    },
                |    $foo = join(list=$subtasks, separator=`\n\n`),
                |    $task => `in_parallel:
                |$foo`
                |}
                |
                |"
            .trim_margin()
            .unwrap(),
            source: r#"
                |  - across:
                |    - var: name
                |      values:
                |      - file1
                |      - file2
                |      - file3
                |    task: create-file
                |    config:
                |      platform: linux
                |      image_resource:
                |        type: registry-image
                |        source: {repository: busybox}
                |      run:
                |        path: touch
                |        args:
                |        - manifests/((.:name))
                |      outputs:
                |      - name: manifests
                |    file: input.yaml
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |  - in_parallel:
                |    - foo: other
                |      foo: other
                |      foo: other
                |      foo: other
                |
                |    - foo: other
                |      foo: other
                |      foo: other
                |      foo: other
                |
                |    - foo: other
                |      foo: other
                |      foo: other
                |      foo: other
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn linearizes_overlapping_rewrite_and_insert() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
language js

pattern before_each_file_prep_imports() {
    $_ where {
        $GLOBAL_IMPORTED_SOURCES = [],
        $GLOBAL_IMPORTED_NAMES = [],
    }
}

private pattern the_import_statement($imports, $source) {
    import_statement(import = import_clause(name = named_imports($imports)), $source)
}

pattern imported_from($from) {
    $name where {
        $program <: program($statements),
        $statements <: some bubble($name, $from) the_import_statement($imports, source = $from) where {
            $imports <: some $name,
        }
    }
}

pattern ensure_import_from($source) {
    $name where {
        if ($name <: not imported_from(from = $source)) {
            if ($GLOBAL_IMPORTED_SOURCES <: not some [$program, $source]) {
                $GLOBAL_IMPORTED_SOURCES += [$program, $source]
            } else {
                true
            },
            if ($GLOBAL_IMPORTED_NAMES <: not some [$program, $name, $source]) {
                $GLOBAL_IMPORTED_NAMES += [$program, $name, $source]
            } else {
                true
            }
        } else {
            true
        }
    }
}

private pattern process_one_source($p, $all_imports) {
    [$p, $source] where {
        $imported_names = [],
        $GLOBAL_IMPORTED_NAMES <: some bubble($p, $source, $imported_names, $all_imports) [$p, $name, $source] where {
            $imported_names += $name,
        },
        $joined_imported_names = text(string=join(list = $imported_names, separator = ", ")),
        if ($p <: program(statements = some the_import_statement($imports, $source))) {
            $imports => `$imports, $joined_imported_names`
        } else {
            $all_imports += `import { $joined_imported_names } from $source;\n`
        }
    }
}

private pattern insert_imports() {
    $p where {
        $all_imports = [],
        $GLOBAL_IMPORTED_SOURCES <: some process_one_source($p, $all_imports),
        if ($all_imports <: not []) {
            or {
              // Try to find a shebang and insert after that
              $p <: program(hash_bang=$h) where {
                $h <: hash_bang_line() += `\n$all_imports`
              },
              // Find an import statement to anchor on
              $p <: program($statements) where {
                $statements <: some $anchor where { $anchor <: import_statement() },
                $anchor += `\n$all_imports`
              },
              // Fall back to inserting the whole program
              $p => `$all_imports\n$p`
            }
        } else {
            true
        }
    }
}

pattern after_each_file_handle_imports() {
  file($body) where $body <: maybe insert_imports()
}

pattern remove_import($from) {
    $name where {
        // Handle named imports
        $program <: maybe contains bubble($name, $from) `import $clause from $raw_source` as $import where {
          $raw_source <: contains $from,
          $clause <: or {
            // Handle module import
            import_clause(default=$name) where {
                $import => .
            },
            // Handle named import
            import_clause($default, name=named_imports($imports)) as $clause where {
                $others = `false`,
                if ($imports <: [$name]) {
                    if ($default <: .) {
                        $import => .
                    } else {
                        $clause => $default
                    }
                } else {
                    $imports <: some $name => .
                }
            }
          }
        }
    }
}

pattern replace_import($old, $new) {
    $name where {
        $name <: remove_import(from=$old),
        $name <: ensure_import_from(source=$new)
    }
}

pattern literal_value() {
  or { number(), string(), `null`, `undefined`}
}

pattern function_like($name, $args, $statements) {
  or {
    `function $name($args) { $statements }`,
    `($args) => { $statements }`,
    `($args) => $statements`
  }
}

// All core stdlib functions can be done here
private pattern before_each_file_stdlib() {
  before_each_file_prep_imports()
}

private pattern after_each_file_stdlib() {
  after_each_file_handle_imports()
}


// These could be redefined in the future (not presently supported)
pattern before_each_file() {
  before_each_file_stdlib()
}

pattern after_each_file() {
  after_each_file_stdlib()
}

import_specifier($name) where {
   $x = `wow`,
   $x <: ensure_import_from(`'react'`),
   $program => `// hello\n$program`,
}"#
            .to_owned(),
            source: r#"
                |import { Component } from 'base';
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |// hello
                |import { Component } from 'base';
                |import { wow } from 'react';
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn metavariable_regex() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |variable_declarator($name, $value) where {
                |   $suffix = "Handler",
                |   $name <: r`([a-zA-Z]*)$suffix`($kind) where {
                |       $name => $kind,
                |   }
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const helloHandler = () => console.log("hello");
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const hello = () => console.log("hello");
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn bound_metavariable_regex() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |variable_declarator($name, $value) where {
                |   $name <: r`([a-zA-Z]*)$value`($kind) where {
                |       $name => $kind,
                |   }
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const helloHandler = handler;
                |const hellohandler = handler;
                |const x = y;
                |const xy = y;
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const helloHandler = handler;
                |const hello = handler;
                |const x = y;
                |const x = y;
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn can_assign_using_reserved_metavariable() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |variable_declarator($name, $value) where {
                |   $new_val = `"$filename"`,
                |   $value => $new_val,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const mango = "mango";
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |const mango = "test-file.tsx";
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn empty_lists_should_match() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |
                |file($body) where {
                |    $body => `"hello"`,
                |    $foo = [],
                |    $foo <: []
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |const mango = "mango";
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |"hello"
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn object_type_metavariable() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |
                |`type BLAH = { $a }` where { $a => `foo`}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |import React from 'react';
                |
                |type BLAH = {
                |  foo: FOO<Bar>;
                |  bar: FOO<Baz>;
                |  baz: boolean;
                |  qux: QUX;
                |};
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
            |import React from 'react';
            |
            |type BLAH = {
            |  foo
            |};
            |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn preserves_comments_at_bracketing_function_body_variable() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |
                |`function ($args) { $body }` => `($args) => { $body }`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |var increment = function (i) {
                |   // some comment
                |   // sweet potato
                |   return i + 1;
                |   /* another comment
                |   kale */
                |};
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |var increment = (i) => { // some comment
                |   // sweet potato
                |   return i + 1;
                |   /* another comment
                |   kale */ };
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn preserves_comments_bracketing_class_body_variable() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |
                |`class Cashew { $stuff }` => `class Peanut { $stuff }`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |class Cashew {
                |   // some comment
                |   // sweet potato
                |   nuts() {
                |       // just for good measure
                |       return i + 1;
                |   }
                |   /* another comment
                |   kale */
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |class Peanut { // some comment
                |   // sweet potato
                |   nuts() {
                |       // just for good measure
                |       return i + 1;
                |   }
                |   /* another comment
                |   kale */ }
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn match_against_regex_metavariable() {
    run_test_match({
        TestArg {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |
                |`console.$log` as $logger where {
                |   $logger <: r"console\.([a-zA-Z]*)"($regex_log),
                |   $log <: $regex_log,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log(cantaloupe)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn match_on_regex_metavariable() {
    run_test_match({
        TestArg {
            pattern: r#"
                |engine marzano(0.1)
                |language js
                |
                |`console.$log` as $logger where {
                |   $logger <: r"console\.([a-zA-Z]*)"($regex_log),
                |   $regex_log <: $log,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log(cantaloupe)
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn basic_rust_snippet() {
    run_test_match({
        TestArg {
            pattern: r#"
                |engine marzano(0.1)
                |language rust
                |
                |`$str.len()`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |let hi = "hello".len();
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn struct_name_and_body_metavariable() {
    run_test_match({
        TestArg {
            pattern: r#"
                |engine marzano(0.1)
                |language rust
                |
                |`struct $name {
                |   $fields
                |}`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |#[derive(Deserialize, Debug)]
                |pub struct PackagedWorkflowOutcome {
                |   pub message: Option<String>,
                |   pub success: bool,
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rust_string_literal_metavariable() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language rust
                |
                |`let $name = "$my_string".to_string();` => `let $my_string = "$my_string".to_string();`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |let raisin = "grape".to_string();
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |let grape = "grape".to_string();
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rust_macro_token_metavariable() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language rust
                |
                |`format!($stuff)` where {
                |   $stuff <: string_literal()
                |} => .
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |let raisin = format!("raisin");
                |let prune = format!("prune {}", 1);
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |let raisin = ;
                |let prune = format!("prune {}", 1);
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn enum_name_and_body_metavariable() {
    run_test_match({
        TestArg {
            pattern: r#"
                |engine marzano(0.1)
                |language rust
                |
                |`enum $name {
                |   $variants
                |}`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |pub enum AuthCommands {
                |   Login(LoginArgs),
                |   Logout(LogoutArgs),
                |   GetToken(GetTokenArgs),
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rust_return_expression_metavariable() {
    run_test_match({
        TestArg {
            pattern: r#"
                |engine marzano(0.1)
                |language rust
                |
                |`return $x;`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |fn mango(x: usize) -> usize {
                |   return x;
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rust_unary_expression_metavariable() {
    run_test_match({
        TestArg {
            pattern: r#"
                |engine marzano(0.1)
                |language rust
                |
                |`*$derefed`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |fn mango(x: &Vec<usize>) -> usize {
                |   return *x[0];
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rust_parameter_metavariable() {
    run_test_match({
        TestArg {
            pattern: r#"
                |engine marzano(0.1)
                |language rust
                |
                |`$x: &HashMap<String, usize>`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |fn mango(x: &HashMap<String, usize>) -> usize {
                |   return x["guava"];
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rust_type_argument_metavariable() {
    run_test_match({
        TestArg {
            pattern: r#"
                |engine marzano(0.1)
                |language rust
                |
                |`&HashMap<$key, $val>`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |fn mango(x: &HashMap<String, usize>) -> usize {
                |   return x["guava"];
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn closure_call_expression_metavariable() {
    run_test_match({
        TestArg {
            pattern: r#"
                |engine marzano(0.1)
                |language rust
                |
                |`$iter.filter($filter)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |let filtered = (0_i32..10)
                |   .filter(|n| n.checked_add(1).is_some());
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn match_expression_snippet() {
    run_test_match({
        TestArg {
            pattern: r#"
                |engine marzano(0.1)
                |language rust
                |
                |`match opt {
                |   Some(n) => n,
                |   None => return,
                |}`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |fn func() {
                |   let n = match opt {
                |       Some(n) => n,
                |       None => return,
                |   };
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn keyword_argument_metavariable() {
    run_test_match({
        TestArg {
            pattern: r#"
                |engine marzano(0.1)
                |language python
                |
                |`mango=Mango(taste=sweet, rating=10)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |fruit = make_fruit(mango=Mango(taste=sweet, rating=10))
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn match_block_metavariable() {
    run_test_match({
        TestArg {
            pattern: r#"
                |engine marzano(0.1)
                |language rust
                |
                |`match $var { $_ }`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |fn func(opt: Option<Result<u64, String>>) {
                |   let n = match opt {
                |       Some(n) => match n {
                |           Ok(n) => n,
                |           _ => return,
                |       }
                |       None => return,
                |   };
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn collapsible_match_block() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language rust
                |
                |`match $var {
                |   $outer($_) => match $_ {
                |       $inner($inner_var) => $matched,
                |       _ => $fallthrough,
                |   }
                |   $_ => $fallthrough,
                |}` => `match $var {
                |   $outer($inner($inner_var)) => $matched,
                |   _ => $fallthrough,
                |}`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |fn func(opt: Option<Result<u64, String>>) {
                |   let n = match opt {
                |       Some(n) => match n {
                |           Ok(n) => n,
                |           _ => return,
                |       }
                |       None => return,
                |   };
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |fn func(opt: Option<Result<u64, String>>) {
                |   let n = match opt {
                |   Some(Ok(n)) => n,
                |   _ => return,
                |};
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rust_match_fn_params() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language rust
                |
                |`fn execute(
                |        $_
                |    ) -> $_ { $_ }` as $fn => `async $fn`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |impl Matcher for FilePattern {
                |    fn execute<'a>(
                |        &'a self,
                |        resolved_pattern: &Q::ResolvedPattern<'a>,
                |        state: &mut State<'a>,
                |        context: &Context<'a>,
                |        logs: &mut AnalysisLogs,
                |    ) -> Result<bool> {
                |        false
                |    }
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |impl Matcher for FilePattern {
                |    async fn execute<'a>(
                |        &'a self,
                |        resolved_pattern: &Q::ResolvedPattern<'a>,
                |        state: &mut State<'a>,
                |        context: &Context<'a>,
                |        logs: &mut AnalysisLogs,
                |    ) -> Result<bool> {
                |        false
                |    }
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn visibility_modifier_metavariable() {
    run_test_match({
        TestArg {
            pattern: r#"
                |engine marzano(0.1)
                |language rust
                |
                |`$vis struct $_ { $_ }`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |pub struct Apple {}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn visibility_modifier_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language rust
                |
                |`$vis fn $_() { $_ }` where {
                |   $vis => `pub`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |pub mod outer_mod {
                |   pub mod inner_mod {
                |       // This function is visible within `outer_mod`
                |       pub(in crate::outer_mod) fn outer_mod_visible_fn() {}
                |       // Same as above, this is only valid in the 2015 edition.
                |       pub(in outer_mod) fn outer_mod_visible_fn_2015() {}
                |
                |       // This function is visible to the entire crate
                |       pub(crate) fn crate_visible_fn() {}
                |
                |       // This function is visible within `outer_mod`
                |       pub(super) fn super_mod_visible_fn() {
                |           // This function is visible since we're in the same `mod`
                |           inner_mod_visible_fn();
                |       }
                |
                |       // This function is visible only within `inner_mod`,
                |       // which is the same as leaving it private.
                |       pub(self) fn inner_mod_visible_fn() {}
                |   }
                |}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |pub mod outer_mod {
                |   pub mod inner_mod {
                |       // This function is visible within `outer_mod`
                |       pub fn outer_mod_visible_fn() {}
                |       // Same as above, this is only valid in the 2015 edition.
                |       pub fn outer_mod_visible_fn_2015() {}
                |
                |       // This function is visible to the entire crate
                |       pub fn crate_visible_fn() {}
                |
                |       // This function is visible within `outer_mod`
                |       pub fn super_mod_visible_fn() {
                |           // This function is visible since we're in the same `mod`
                |           inner_mod_visible_fn();
                |       }
                |
                |       // This function is visible only within `inner_mod`,
                |       // which is the same as leaving it private.
                |       pub fn inner_mod_visible_fn() {}
                |   }
                |}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn raw_identifier_snippet() {
    run_test_match({
        TestArg {
            pattern: r#"
                |language rust
                |
                |`r#match`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |use super::r#match::Match;
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn limit_applies_to_file_by_default() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |`console.log($_)` limit 2 => .
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log('apple');
                |console.log('mango');
                |console.log('banana');
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#""#.trim_margin().unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn explicit_limit_within_file() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |contains bubble `console.log($_)` limit 2 => .
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log('banana');
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#""#.trim_margin().unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn limit_over_rewrite() {
    run_test_no_match({
        TestArg {
            pattern: r#"
                |language js
                |
                |`console.log($_)` => . limit 0
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |console.log('apple');
                |console.log('mango');
                |console.log('banana');
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rewrites_metavariable_bound_multiple_times_across_definition_boundary() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |pattern a($a) {
                |   `$a + $a`
                |}
                |
                |a(a=$b) where {
                |   $b => `10`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |i + i
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |10 + 10
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn rewrites_react_node() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language js
                |
                |pattern ReactNode($name, $props, $children) {
                |   or {
                |       `<$name $props>$children</$name>`,
                |       `<$name $props />`
                |   }
                |}
                |
                |ReactNode(name=$call) where $call => `div`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |<Box>
                |   <p>Hi</p>
                |</Box>
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |<div>
                |   <div>Hi</div>
                |</div>
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn regex_snippet_matches_entire_node() {
    run_test_no_match(TestArg {
        pattern: r#"
            |language js
            |
            |comment() as $c where {
            |   $c <: r`TODO`
            |}
            |"#
        .trim_margin()
        .unwrap(),
        source: r#"
            |const a = true;
            |
            |// TODO something
            |const b = true;
            |
            |const c = true;
            |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn go_import_list_metavariable() {
    run_test_match(TestArg {
        pattern: r#"
            |language go
            |
            |`import $import`
            |"#
        .trim_margin()
        .unwrap(),
        source: r#"
            |import (
            |   "fmt"
            |   "path"
            |   "path/filepath"
            |)
            |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn go_import_not_list() {
    run_test_expected(TestArgExpected {
        pattern: r#"
            |language go
            |
            |`import $import "$path"` => `import $path "$import"`
            |"#
        .trim_margin()
        .unwrap(),
        source: r#"
            |import foo "bar"
            |"#
        .trim_margin()
        .unwrap(),
        expected: r#"
            |import bar "foo"
            |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn go_import_metavariable() {
    run_test_match(TestArg {
        pattern: r#"
            |language go
            |
            |`import ($import)`
            |"#
        .trim_margin()
        .unwrap(),
        source: r#"
            |import (
            |   "fmt"
            |   "path"
            |   "path/filepath"
            |)
            |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn go_field_identifier() {
    run_test_match(TestArg {
        pattern: r#"
            |language go
            |
            |`ReadFile`
            |"#
        .trim_margin()
        .unwrap(),
        source: r#"
            |import (
            |   "io/ioutil"
            |)
            |
            |file := ioutil.ReadFile("file.txt")
            |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn go_package_identifier() {
    run_test_match(TestArg {
        pattern: r#"
            |language go
            |
            |`mypackage`
            |"#
        .trim_margin()
        .unwrap(),
        source: r#"
            |func ZoneLockdown(api *mypackage.API) (resp interface{}, err error) {
            |   resp, err = api.Call()
            |   return
            |}
            |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn go_package_type() {
    run_test_match(TestArg {
        pattern: r#"
            |language go
            |
            |`API`
            |"#
        .trim_margin()
        .unwrap(),
        source: r#"
            |func ZoneLockdown(api *mypackage.API) (resp interface{}, err error) {
            |   resp, err = api.Call()
            |   return
            |}
            |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn php_no_match() {
    run_test_no_match({
        TestArg {
            pattern: r#"
                |language php(only)
                |
                |`TEST`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |echo "hello world"
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn php_simple_match() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language php(only)
                |
                |`echo ^x;` => `^x + ^x;`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |echo "duplicate this message";
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |"duplicate this message" + "duplicate this message";
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn php_html_simple_match() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language php
                |
                |`<?php
                |   echo ^x;
                |?>` where {
                |   ^x => `^x + ^x`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |<?php
                |   echo "duplicate this message";
                |?>
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |<?php
                |   echo "duplicate this message" + "duplicate this message";
                |?>
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn php_html_multi_arg() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language php
                |
                |`<?php
                |   $cost = Array(^x);
                |?>` where {
                |   ^x => `100, 399, 249`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |<?php
                |   $cost = Array(20, 10);
                |?>
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |<?php
                |   $cost = Array(100, 399, 249);
                |?>
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn php_until() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language php(only)
                |
                |contains bubble `foo(^x)` => `bar(^x)` until `foo(^_)`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |   foo(another(foo(x)));
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |   bar(another(foo(x)));
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn php_quote_snippet_rewrite() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language php(only)
                |php"foo" => php"bar"
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"$a = $foo;"#.to_owned(),
            expected: r#"$a = $bar;"#.to_owned(),
        }
    })
    .unwrap();
}

#[test]
fn php_if_statement() {
    run_test_expected(TestArgExpected {
        pattern: r#"
                |language php(only)
                |
                |`$a = 12;` => `$b=24;`
                |"#
        .trim_margin()
        .unwrap(),
        source: r#"
                |#
                |if (!$foo = $bar) {
                |   $a = 12;
                |}
                |"#
        .trim_margin()
        .unwrap(),
        expected: r#"
                |#
                |if (!$foo = $bar) {
                |   $b=24;
                |}
                |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn php_delete_include() {
    run_test_expected(TestArgExpected {
        pattern: r#"
                |language php(only)
                |
                |`include ^package;` => .
                |"#
        .trim_margin()
        .unwrap(),
        source: r#"
                |include 'test.php';
                |$test = "";
                |"#
        .trim_margin()
        .unwrap(),
        expected: r#"
                |
                |$test = "";
                |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn php_function_modifier() {
    run_test_expected(TestArgExpected {
        pattern: r#"
                |language php(only)
                |
                |`class ^_ { ^mod function ^name(){ ^_ } }` where {
                |   ^mod => `private`,
                |   ^name => `modified`,
                |}
                |"#
        .trim_margin()
        .unwrap(),
        source: r#"
            |class Log {
            |   public function printHello()
            |   {
            |       echo $this->public;
            |       echo $this->protected;
            |       echo $this->private;
            |   }
            |}
            |"#
        .trim_margin()
        .unwrap(),
        expected: r#"
            |class Log {
            |   private function modified()
            |   {
            |       echo $this->public;
            |       echo $this->protected;
            |       echo $this->private;
            |   }
            |}
            |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn php_rewrite_arrow_function() {
    run_test_expected(TestArgExpected {
        pattern: r#"
                |language php(only)
                |
                |`fn(^a) => ^_` => `fn(^a) => $x * $x`
                |"#
        .trim_margin()
        .unwrap(),
        source: "$fn1 = fn($x) => $x + $y;".trim_margin().unwrap(),
        expected: "$fn1 = fn($x) => $x * $x;".trim_margin().unwrap(),
    })
    .unwrap();
}

#[test]
fn php_array() {
    run_test_expected(TestArgExpected {
        pattern: r#"
                |language php(only)
                |
                |`^a=>^_` => `^a=>24`
                |"#
        .trim_margin()
        .unwrap(),
        source: r#"$fn1 = array("a"=>1, "b"=>2, "c"=>3);"#.trim_margin().unwrap(),
        expected: r#"$fn1 = array("a"=>24, "b"=>24, "c"=>24);"#.trim_margin().unwrap(),
    })
    .unwrap();
}

#[test]
fn php_html_foreach() {
    run_test_expected(TestArgExpected {
        pattern: r#"
                |language php
                |
                |`foreach(^x as ^y){^_}` where {
                |   ^x => `$x`,
                |   ^y => `$y`,
                |}
                |"#
        .trim_margin()
        .unwrap(),
        source: r#"
                |<?php
                |    $arr = array(1, 2, 3, 4);
                |    foreach($arr as &$value) {
                |        $value = $value * 2;
                |    }
                |?>"#
            .trim_margin()
            .unwrap(),
        expected: r#"
                |<?php
                |    $arr = array(1, 2, 3, 4);
                |    foreach($x as $y) {
                |        $value = $value * 2;
                |    }
                |?>"#
            .trim_margin()
            .unwrap(),
    })
    .unwrap();
}

#[test]
fn php_echo() {
    run_test_expected(TestArgExpected {
        pattern: r#"
                |language php(only)
                |
                |`echo ^_;` => `print "modified";`
                |"#
        .trim_margin()
        .unwrap(),
        source: r#"
                |$arr = array(1, 2, 3, 4);
                |foreach($arr as &$value) {
                |    echo $value * 2;
                |}
                |"#
        .trim_margin()
        .unwrap(),
        expected: r#"
                |$arr = array(1, 2, 3, 4);
                |foreach($arr as &$value) {
                |    print "modified";
                |}
                |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn php_cast() {
    run_test_expected(TestArgExpected {
        pattern: r#"
                |language php(only)
                |
                |`(^x) $a` => `(string) $a`
                |"#
        .trim_margin()
        .unwrap(),
        source: r#"
                |$i = (int) $a;
                |$f = (float) $a;
                |"#
        .trim_margin()
        .unwrap(),
        expected: r#"
                |$i = (string) $a;
                |$f = (string) $a;
                |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn php_if() {
    run_test_expected(TestArgExpected {
        pattern: r#"
                |language php(only)
                |
                |`if(^a){^_}` where {
                |   ^a => `$a == $b`,
                |}
                |"#
        .trim_margin()
        .unwrap(),
        source: r#"
                |$a = 1;
                |$b = 1;
                |if($a != $b){
                |   echo "pass";
                |}
                |"#
        .trim_margin()
        .unwrap(),
        expected: r#"
                |$a = 1;
                |$b = 1;
                |if($a == $b){
                |   echo "pass";
                |}
                |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn php_class() {
    run_test_expected(TestArgExpected {
        pattern: r#"
                |language php(only)
                |
                |`class ^name {
                |    function ^fname() {
                |        if (^a) { ^_ } else { ^_ }
                |    }
                |}` where {
                |   ^name => `Mod`,
                |   ^fname => `mod_method`,
                |   ^a => `$a == $b`,
                |}
                |"#
        .trim_margin()
        .unwrap(),
        source: r#"
                |class A {
                |    function foo()
                |    {
                |        if (isset($this)) {
                |            echo '$this is defined (';
                |            echo get_class($this);
                |            echo ")\n";
                |        } else {
                |            echo "\$this is not defined.\n";
                |        }
                |    }
                |}
                |
                |class B {
                |    function bar()
                |    {
                |        A::foo();
                |    }
                |}
                |"#
        .trim_margin()
        .unwrap(),
        expected: r#"
                |class Mod {
                |    function mod_method()
                |    {
                |        if ($a == $b) {
                |            echo '$this is defined (';
                |            echo get_class($this);
                |            echo ")\n";
                |        } else {
                |            echo "\$this is not defined.\n";
                |        }
                |    }
                |}
                |
                |class B {
                |    function bar()
                |    {
                |        A::foo();
                |    }
                |}
                |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn css_property_value() {
    run_test_match(TestArg {
        pattern: r#"
            |language css
            |
            |`var($a)`
            |"#
        .trim_margin()
        .unwrap(),
        source: r#"
            |#some-id {
            |    some-property: 5px;
            |    color: var(--red)
            |  }
            |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn json_empty_string_should_not_match_everything() {
    run_test_no_match(TestArg {
        pattern: r#"
            |engine marzano(0.1)
            |language json
            |
            |`"x": ""`
            |"#
        .trim_margin()
        .unwrap(),
        source: r#"
            |{
            |  "x": "foo"
            |}
            |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn json_empty_string_should_match_self() {
    run_test_match(TestArg {
        pattern: r#"
            |engine marzano(0.1)
            |language json
            |
            |`"x": ""`
            |"#
        .trim_margin()
        .unwrap(),
        source: r#"
            |{
            |  "x": ""
            |}
            |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn limit_export_default_match() {
    run_test_expected(TestArgExpected {
        pattern: r#"
            |language js
            |
            |`export default function $name() {}` where $name => `foo`
            |"#
        .trim_margin()
        .unwrap(),
        source: r#"
            |export async function loader() {}
            |export default function main() {}
            |"#
        .trim_margin()
        .unwrap(),
        expected: r#"
        |export async function loader() {}
        |export default function foo() {}
        |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn python_support_empty_line() {
    run_test_expected(TestArgExpected {
        pattern: r#"
            |engine marzano(0.1)
            |language python
            |`class $name: $body` => $body
            |"#
        .trim_margin()
        .unwrap(),
        source: r#"
            |class MyClass:
            |    def function(self):
            |        result = 1 + 1
            |
            |        return result
            |"#
        .trim_margin()
        .unwrap(),
        expected: r#"
        |def function(self):
        |    result = 1 + 1
        |
        |    return result
        |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn delete_import_clause() {
    run_test_expected(TestArgExpected {
        pattern: r#"
            |engine marzano(0.1)
            |language js
            |`import $_` => .
            |"#
        .trim_margin()
        .unwrap(),
        source: r#"
            |import * as React from "react";
            |
            |<div>Hi</div>;
            |"#
        .trim_margin()
        .unwrap(),
        expected: r#"
        |<div>Hi</div>;
        |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
fn remove_unused_import() {
    run_test_expected(TestArgExpected {
        pattern: r#"
language js

pattern remove_unused_imports($src) {
or {
    `import * as $import_clause from $src`,
    `import $import_clause, { $named_imports } from $src` where {
        $named_imports <: maybe some bubble($keep_named_import_list) or {`type $import`, `$import`} as $full where {
            if($program <: contains `$import` until `import $_`) {
                $keep_named_import_list = true
             } else {
                $full => .
            }
        }
    },
    `import $import_clause from $src` where { $import_clause <: not contains `{$_}`},
} as $import_line where {
    $import_clause <: or {`type $module_name`, `$module_name`},
    $program <: not contains $module_name until `import $_`,
    if($keep_named_import_list <: undefined) {
        $import_line => .
    } else {
        $import_clause => .
    }
  }
}

remove_unused_imports()"#
        .to_owned(),
        source: r#"
            |import * as React from "react";
            |
            |<div>Hi</div>;
            |"#
        .trim_margin()
        .unwrap(),
        expected: r#"
        |<div>Hi</div>;
        |"#
        .trim_margin()
        .unwrap(),
    })
    .unwrap();
}

#[test]
#[ignore = "this test will be fixed in a followup pr"]
fn yaml_list_add_indentation() {
    run_test_expected({
        TestArgExpected {
            pattern: r"
                |engine marzano(0.1)
                |language yaml
                |
                |`- $task` where {
                |    $task <: block_mapping($items),
                |    $subtasks = [],
                |    $items <: some block_mapping_pair(key=contains `across`, $value),
                |    $value <: contains `values: $values`,
                |    $values <: some bubble($items, $subtasks) `- $val` where {
                |        $new_task = [],
                |        $items <: some bubble($new_task) $_ where {
                |                $new_task += `foo: other`,
                |            },
                |        $bar = join(list=$new_task, separator=`\n `),
                |        $subtasks += `- $bar`
                |    },
                |    $foo = join(list=$subtasks, separator=`\n\n`),
                |    $task => `in_parallel:
                |  nested:
                |    $foo`
                |}
                |
                |"
            .trim_margin()
            .unwrap(),
            source: r#"
                |  - across:
                |    - var: name
                |      values:
                |      - file1
                |      - file2
                |      - file3
                |    task: create-file
                |    config:
                |      platform: linux
                |      image_resource:
                |        type: registry-image
                |        source: {repository: busybox}
                |      run:
                |        path: touch
                |        args:
                |        - manifests/((.:name))
                |      outputs:
                |      - name: manifests
                |    file: input.yaml
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |  - in_parallel:
                |      nested:
                |        - foo: other
                |          foo: other
                |          foo: other
                |          foo: other
                |
                |        - foo: other
                |          foo: other
                |          foo: other
                |          foo: other
                |
                |        - foo: other
                |          foo: other
                |          foo: other
                |          foo: other
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn yaml_indents() {
    run_test_expected({
        TestArgExpected {
            pattern: r"
                |engine marzano(0.1)
                |language yaml
                |
                |`- $foo` where {
                |    $joined = `foo: bar
                |baz: qux`,
                |    $foo => `baz:
                |  $joined`
                |}
                |"
            .trim_margin()
            .unwrap(),
            source: r#"
                |  - across:
                |    - var: name
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |  - baz:
                |      foo: bar
                |      baz: qux
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn yaml_indents_join() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |engine marzano(0.1)
                |language yaml
                |
                |`- $foo` where {
                |    $list = ["a", "b", "c"],
                |    $accumulator = [],
                |    $list <: some bubble($accumulator) {
                |        $accumulator += `foo: bar`
                |    },
                |    $joined = join(list=$accumulator, separator=`\n`),
                |    $foo => `baz:
                |  $joined`
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |  - across:
                |    - var: name
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |  - baz:
                |      foo: bar
                |      foo: bar
                |      foo: bar
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn ruby_hello_world() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language ruby
                |
                |`puts $string` => `puts $string + " modified"`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |puts "hello world"
                |puts "hello again"
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |puts "hello world" + " modified"
                |puts "hello again" + " modified"
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn ruby_if() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language ruby
                |
                |`if $x
                |   puts $success
                |end` where {
                |   $x => `y > 2`,
                |   $success => `"y is greater than 2"`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |if x == 0
                |   puts "pass"
                |end
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |if y > 2
                |   puts "y is greater than 2"
                |end
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn ruby_class() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language ruby
                |
                |`class $x < $superclass
                |   def $name($y,h)
                |      $yinstance, @height = $y, h
                |   end
                |   $_
                |end` where {
                |   $x => `Foo`,
                |   $superclass => `Bar`,
                |   $name => `init`,
                |   $y => `w`,
                |   $yinstance => `@w`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |class Box < Foo
                |   # constructor method
                |   def initialize(h,h)
                |      @width, @height = h, h
                |   end
                |   # instance method
                |   def getArea
                |      @width * @height
                |   end
                |end
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |class Foo < Bar
                |   # constructor method
                |   def init(w,h)
                |      @w, @height = w, h
                |   end
                |   # instance method
                |   def getArea
                |      @width * @height
                |   end
                |end
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn ruby_class_2() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language ruby
                |
                |`$yinstance, @height = $y, h` where {
                |   $y => `w`,
                |   $yinstance => `@w`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |class Box < Foo
                |   # constructor method
                |   def initialize(w,h)
                |      @width, @height = h, h
                |   end
                |   # instance method
                |   def getArea
                |      @width * @height
                |   end
                |end
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |class Box < Foo
                |   # constructor method
                |   def initialize(w,h)
                |      @w, @height = w, h
                |   end
                |   # instance method
                |   def getArea
                |      @width * @height
                |   end
                |end
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn ruby_class_method() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language ruby
                |
                |`class Box
                |   $methods
                |end` => `$methods`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |class Box
                |   def init(w,h)
                |      @width, @height = w, h
                |   end
                |end
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |def init(w,h)
                |      @width, @height = w, h
                |   end
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn ruby_each() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language ruby
                |
                |`[a, b, c].each do |$a|
                |   puts $b
                |end` where {
                |   $a => `x`,
                |   $b => `x`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |[a, b, c].each do |a1|
                |   puts abc::ABC
                |end
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |[a, b, c].each do |x|
                |   puts x
                |end
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn ruby_scope() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language ruby
                |
                |`puts abc::$a` where {
                |   $a => `X`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |[a, b, c].each do |n|
                |   puts abc::ABC
                |end
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |[a, b, c].each do |n|
                |   puts abc::X
                |end
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn ruby_lambda() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language ruby
                |
                |`lambda {|$a| $a**$b }` where {
                |   $b => `3`,
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |square = lambda {|val| val**2 }
                |three_squared = square.call(3)
                |puts "Three squared is #{three_squared}"
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |square = lambda {|val| val**3 }
                |three_squared = square.call(3)
                |puts "Three squared is #{three_squared}"
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn ruby_case() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language ruby
                |
                |`when $a
                |  print('It is a string')` => `when Integer
                |  print('It is an integer')`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |obj = 'hello'
                |case obj.class
                |when String
                |  print('It is a string')
                |when Fixnum
                |  print('It is a number')
                |else
                |  print('It is not a string or number')
                |end
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |obj = 'hello'
                |case obj.class
                |when Integer
                |  print('It is an integer')
                |when Fixnum
                |  print('It is a number')
                |else
                |  print('It is not a string or number')
                |end
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn ruby_nested_module() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language ruby
                |
                |`module Foo
                |   module $foo_child
                |   end
                |end` where {
                |   $foo_child => `Child`    
                |}
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |module Foo
                |   module Bar
                |   end
                |end
                |
                |module Foo
                |  module Baz
                |  end
                |end
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |module Foo
                |   module Child
                |   end
                |end
                |
                |module Foo
                |  module Child
                |  end
                |end
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn ruby_hash() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language ruby
                |
                |`$prop: $key` => `$key: $prop`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |person = {name: "Alice", age: 25, city: "New York"}
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |person = {"Alice": name, 25: age, "New York": city}
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}

#[test]
fn ruby_array() {
    run_test_expected({
        TestArgExpected {
            pattern: r#"
                |language ruby
                |
                |`$a, 2, 3` => `$a`
                |"#
            .trim_margin()
            .unwrap(),
            source: r#"
                |person = [1, 2, 3]
                |"#
            .trim_margin()
            .unwrap(),
            expected: r#"
                |person = [1]
                |"#
            .trim_margin()
            .unwrap(),
        }
    })
    .unwrap();
}