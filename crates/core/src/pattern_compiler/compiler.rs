use super::{
    auto_wrap::auto_wrap_pattern,
    function_definition_compiler::{
        ForeignFunctionDefinitionCompiler, GritFunctionDefinitionCompiler,
    },
    pattern_compiler::PatternCompiler,
    pattern_definition_compiler::PatternDefinitionCompiler,
    predicate_definition_compiler::PredicateDefinitionCompiler,
    NodeCompiler,
};
use crate::{
    analysis::{has_limit, is_multifile},
    parse::make_grit_parser,
    pattern::{
        built_in_functions::BuiltIns,
        constants::{
            ABSOLUTE_PATH_INDEX, DEFAULT_FILE_NAME, FILENAME_INDEX, NEW_FILES_INDEX, PROGRAM_INDEX,
        },
        function_definition::{ForeignFunctionDefinition, GritFunctionDefinition},
        pattern_definition::PatternDefinition,
        predicate_definition::PredicateDefinition,
        variable::VariableSourceLocations,
        VariableLocations,
    },
    problem::{MarzanoQueryContext, Problem},
};
use anyhow::{anyhow, bail, Result};
use grit_util::{traverse, AnalysisLogBuilder, AnalysisLogs, AstNode, FileRange, Order, Range};
use itertools::Itertools;
use marzano_language::{self, target_language::TargetLanguage};
use marzano_util::{cursor_wrapper::CursorWrapper, node_with_source::NodeWithSource};
use regex::Regex;
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsStr,
    path::Path,
    str::Utf8Error,
    vec,
};
use tree_sitter::{Node, Parser, Tree};

#[cfg(feature = "grit_tracing")]
use tracing::instrument;

pub(crate) struct CompilationContext<'a> {
    pub file: &'a str,
    pub built_ins: &'a BuiltIns,
    pub lang: &'a TargetLanguage,
    pub pattern_definition_info: &'a BTreeMap<String, DefinitionInfo>,
    pub predicate_definition_info: &'a BTreeMap<String, DefinitionInfo>,
    pub function_definition_info: &'a BTreeMap<String, DefinitionInfo>,
    pub foreign_function_definition_info: &'a BTreeMap<String, DefinitionInfo>,
}

pub(crate) struct NodeCompilationContext<'a> {
    pub compilation: &'a CompilationContext<'a>,

    /// Used to lookup local variables in the `vars_array`.
    pub vars: &'a mut BTreeMap<String, usize>,

    /// Storage for variable information.
    ///
    /// The outer vector can be index using `scope_index`, while the individual
    /// variables in a scope can be indexed using the indices stored in `vars`
    /// and `global_vars`.
    pub vars_array: &'a mut Vec<Vec<VariableSourceLocations>>,

    /// Index of the local scope.
    ///
    /// Corresponds to the index in the outer vector of `vars_array`.
    pub scope_index: usize,

    /// Used to lookup global variables in the `vars_array`.
    ///
    /// Global variables are always at scope 0.
    pub global_vars: &'a mut BTreeMap<String, usize>,

    pub logs: &'a mut AnalysisLogs,
}

fn grit_parsing_errors(tree: &Tree, src: &str, file_name: &str) -> Result<AnalysisLogs> {
    let mut errors = vec![];
    let cursor = tree.walk();
    let mut log_builder = AnalysisLogBuilder::default();
    let level: u16 = if file_name == DEFAULT_FILE_NAME {
        299
    } else {
        300
    };
    log_builder
        .level(level)
        .engine_id("marzano(0.1)".to_owned())
        .file(file_name.to_owned());

    for n in traverse(CursorWrapper::new(cursor, src), Order::Pre) {
        if n.node.is_error() || n.node.is_missing() {
            let position = n.range().start;

            let error_node = n.text()?;
            let identifier_regex = Regex::new(r"^([A-Za-z0-9_]*)\(\)$")?;
            let message = if let Some(found) = identifier_regex.find(&error_node) {
                format!(
                    "{} is a reserved keyword in Grit. Try renaming your pattern.",
                    found.as_str().trim_end_matches("()")
                )
            } else {
                let file_locations_str = if file_name == DEFAULT_FILE_NAME {
                    String::new()
                } else {
                    format!(" in {}", file_name)
                };
                format!(
                    "Pattern syntax error at {position}{file_locations_str}. \
                        If you hit this error while running grit apply on a \
                        pattern from the Grit standard library, try running \
                        grit init. If you are running a custom pattern, check \
                        out the docs at https://docs.grit.io/ for help with \
                        writing patterns.",
                )
            };

            let log = log_builder
                .clone()
                .message(message)
                .position(position)
                .build()?;
            errors.push(log);
        }
    }
    Ok(errors.into())
}

// this code looks wrong. Todo test to see if we correctly find duplicate
// parameter names, if not fix.
fn get_duplicates(list: &[(String, Range)]) -> Vec<&String> {
    let mut dups = BTreeSet::new();
    let unique: BTreeSet<String> = list.iter().map(|s| s.0.to_owned()).collect();
    for s in list {
        if !unique.contains(&s.0) {
            dups.insert(&s.0);
        }
    }
    dups.into_iter().collect()
}

// errors only refer to pattern, but could also be predicate
fn insert_definition_index(
    indices: &mut BTreeMap<String, DefinitionInfo>,
    definition: NodeWithSource,
    index: &mut usize,
) -> Result<()> {
    let name = definition
        .child_by_field_name("name")
        .ok_or_else(|| anyhow!("missing name of patternDefinition"))?;
    let name = name.text()?;
    let name = name.trim();
    let parameters: Vec<_> = definition
        .named_children_by_field_name("args")
        .map(|n| Ok::<(String, Range), Utf8Error>((n.text()?.trim().to_string(), n.range())))
        .collect::<Result<Vec<_>, Utf8Error>>()?;
    let duplicates = get_duplicates(&parameters);
    if !duplicates.is_empty() {
        bail!(
            "Pattern parameters must be unique,
            but {} had repeated parameters {:?}.",
            name,
            duplicates
        )
    }
    let info = DefinitionInfo {
        index: *index,
        parameters,
    };
    match indices.insert(name.to_owned(), info) {
        Some(_) => bail!("cannot have repeated definition of pattern {}", name),
        None => {
            *index += 1;
            Ok(())
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn node_to_definition_info(
    node: &NodeWithSource,
    pattern_indices: &mut BTreeMap<String, DefinitionInfo>,
    pattern_index: &mut usize,
    predicate_indices: &mut BTreeMap<String, DefinitionInfo>,
    predicate_index: &mut usize,
    function_indices: &mut BTreeMap<String, DefinitionInfo>,
    function_index: &mut usize,
    foreign_function_indices: &mut BTreeMap<String, DefinitionInfo>,
    foreign_function_index: &mut usize,
) -> Result<()> {
    for definition in node.named_children_by_field_name("definitions") {
        if let Some(pattern_definition) = definition.child_by_field_name("pattern") {
            insert_definition_index(pattern_indices, pattern_definition, pattern_index)?;
        } else if let Some(predicate_definition) = definition.child_by_field_name("predicate") {
            insert_definition_index(predicate_indices, predicate_definition, predicate_index)?;
        } else if let Some(function_definition) = definition.child_by_field_name("function") {
            insert_definition_index(function_indices, function_definition, function_index)?;
        } else if let Some(foreign_definition) = definition.child_by_field_name("foreign") {
            insert_definition_index(
                foreign_function_indices,
                foreign_definition,
                foreign_function_index,
            )?;
        } else {
            bail!("definition must be either a pattern, a predicate or a function");
        }
    }
    Ok(())
}

pub(crate) struct DefinitionInfo {
    pub(crate) index: usize,
    pub(crate) parameters: Vec<(String, Range)>,
}

struct DefinitionInfoKinds {
    pattern_indices: BTreeMap<String, DefinitionInfo>,
    predicate_indices: BTreeMap<String, DefinitionInfo>,
    function_indices: BTreeMap<String, DefinitionInfo>,
    foreign_function_indices: BTreeMap<String, DefinitionInfo>,
}

fn get_definition_info(
    libs: &[(String, String)],
    root: &NodeWithSource,
    parser: &mut Parser,
) -> Result<DefinitionInfoKinds> {
    let mut pattern_indices: BTreeMap<String, DefinitionInfo> = BTreeMap::new();
    let mut pattern_index = 0;
    let mut predicate_indices: BTreeMap<String, DefinitionInfo> = BTreeMap::new();
    let mut predicate_index = 0;
    let mut function_indices: BTreeMap<String, DefinitionInfo> = BTreeMap::new();
    let mut function_index = 0;
    let mut foreign_function_indices: BTreeMap<String, DefinitionInfo> = BTreeMap::new();
    let mut foreign_function_index = 0;
    for (file, pattern) in libs.iter() {
        let tree = parse_one(parser, pattern, file)?;
        let root = NodeWithSource::new(tree.root_node(), pattern);
        node_to_definition_info(
            &root,
            &mut pattern_indices,
            &mut pattern_index,
            &mut predicate_indices,
            &mut predicate_index,
            &mut function_indices,
            &mut function_index,
            &mut foreign_function_indices,
            &mut foreign_function_index,
        )?;
        if root.child_by_field_name("pattern").is_some() {
            let path = Path::new(file);
            if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                let info = DefinitionInfo {
                    index: pattern_index,
                    parameters: vec![],
                };
                match pattern_indices.insert(name.to_owned(), info) {
                    Some(_) => bail!("cannot have repeated definition of pattern {}", name),
                    None => pattern_index += 1,
                };
            } else {
                bail!(
                    "failed to get pattern name from definition in file {}",
                    file
                )
            }
        }
    }
    node_to_definition_info(
        root,
        &mut pattern_indices,
        &mut pattern_index,
        &mut predicate_indices,
        &mut predicate_index,
        &mut function_indices,
        &mut function_index,
        &mut foreign_function_indices,
        &mut foreign_function_index,
    )?;
    Ok(DefinitionInfoKinds {
        pattern_indices,
        predicate_indices,
        function_indices,
        foreign_function_indices,
    })
}

fn node_to_definitions(
    node: NodeWithSource,
    context: &mut NodeCompilationContext,
    pattern_definitions: &mut Vec<PatternDefinition<MarzanoQueryContext>>,
    predicate_definitions: &mut Vec<PredicateDefinition<MarzanoQueryContext>>,
    function_definitions: &mut Vec<GritFunctionDefinition<MarzanoQueryContext>>,
    foreign_function_definitions: &mut Vec<ForeignFunctionDefinition>,
) -> Result<()> {
    for definition in node.named_children_by_field_name("definitions") {
        if let Some(pattern_definition) = definition.child_by_field_name("pattern") {
            // todo check for duplicate names
            pattern_definitions.push(PatternDefinitionCompiler::from_node(
                &pattern_definition,
                context,
            )?);
        } else if let Some(predicate_definition) = definition.child_by_field_name("predicate") {
            // todo check for duplicate names
            predicate_definitions.push(PredicateDefinitionCompiler::from_node(
                &predicate_definition,
                context,
            )?);
        } else if let Some(function_definition) = definition.child_by_field_name("function") {
            function_definitions.push(GritFunctionDefinitionCompiler::from_node(
                &function_definition,
                context,
            )?);
        } else if let Some(function_definition) = definition.child_by_field_name("foreign") {
            foreign_function_definitions.push(ForeignFunctionDefinitionCompiler::from_node(
                &function_definition,
                context,
            )?);
        } else {
            bail!("definition must be either a pattern, a predicate or a function");
        }
    }
    Ok(())
}

struct DefinitionOutput {
    vars_array: Vec<Vec<VariableSourceLocations>>,
    pattern_definitions: Vec<PatternDefinition<MarzanoQueryContext>>,
    predicate_definitions: Vec<PredicateDefinition<MarzanoQueryContext>>,
    function_definitions: Vec<GritFunctionDefinition<MarzanoQueryContext>>,
    foreign_function_definitions: Vec<ForeignFunctionDefinition>,
}

fn get_definitions(
    libs: &[(String, String)],
    source_file: &NodeWithSource,
    parser: &mut Parser,
    context: &CompilationContext,
    global_vars: &mut BTreeMap<String, usize>,
    logs: &mut AnalysisLogs,
) -> Result<DefinitionOutput> {
    let mut pattern_definitions = vec![];
    let mut predicate_definitions = vec![];
    let mut function_definitions = vec![];
    let mut foreign_function_definitions = vec![];
    let mut vars_array = vec![];
    vars_array.push(
        global_vars
            .iter()
            .sorted_by(|x, y| Ord::cmp(x.1, y.1))
            .map(|x| VariableSourceLocations {
                name: x.0.clone(),
                file: context.file.to_owned(),
                locations: BTreeSet::new(),
            })
            .collect(),
    );

    for (file, pattern) in libs.iter() {
        let mut node_context = NodeCompilationContext {
            compilation: &CompilationContext { file, ..*context },
            // We're not in a local scope yet, so this map is kinda useless.
            // It's just there because all node compilers expect one.
            vars: &mut BTreeMap::new(),
            vars_array: &mut vars_array,
            scope_index: 0,
            global_vars,
            logs,
        };

        let tree = parse_one(parser, pattern, file)?;
        let source_file = tree.root_node();
        node_to_definitions(
            NodeWithSource::new(source_file.clone(), pattern),
            &mut node_context,
            &mut pattern_definitions,
            &mut predicate_definitions,
            &mut function_definitions,
            &mut foreign_function_definitions,
        )?;

        if let Some(bare_pattern) = source_file.child_by_field_name("pattern") {
            let mut local_vars = BTreeMap::new();
            let (scope_index, mut local_context) = create_scope!(node_context, local_vars);
            let path = Path::new(file);
            let Some(name) = path.file_stem().and_then(OsStr::to_str) else {
                bail!("failed to get pattern name from definition in file {file}");
            };

            let body = PatternCompiler::from_node(
                &NodeWithSource::new(bare_pattern, pattern),
                &mut local_context,
            )?;
            let pattern_def = PatternDefinition::new(
                name.to_owned(),
                scope_index,
                vec![],
                local_vars.values().cloned().collect(),
                body,
            );
            pattern_definitions.push(pattern_def);
        }
    }
    node_to_definitions(
        source_file.clone(),
        &mut NodeCompilationContext {
            compilation: context,
            // We're not in a local scope yet, so this map is kinda useless.
            // It's just there because all node compilers expect one.
            vars: &mut BTreeMap::new(),
            vars_array: &mut vars_array,
            scope_index: 0,
            global_vars,
            logs,
        },
        &mut pattern_definitions,
        &mut predicate_definitions,
        &mut function_definitions,
        &mut foreign_function_definitions,
    )?;
    Ok(DefinitionOutput {
        vars_array,
        pattern_definitions,
        predicate_definitions,
        function_definitions,
        foreign_function_definitions,
    })
}
struct DefsToFilenames {
    patterns: BTreeMap<String, String>,
    predicates: BTreeMap<String, String>,
    functions: BTreeMap<String, String>,
    foreign_functions: BTreeMap<String, String>,
}

fn defs_to_filenames(
    libs: &BTreeMap<String, String>,
    parser: &mut Parser,
    root: Node,
    src: &str,
) -> Result<DefsToFilenames> {
    let mut patterns = BTreeMap::new();
    let mut predicates = BTreeMap::new();
    let mut functions = BTreeMap::new();
    let mut foreign_functions = BTreeMap::new();
    for (file, pattern) in libs.iter() {
        let tree = parse_one(parser, pattern, file)?;
        let node = tree.root_node();
        let mut cursor = node.walk();
        for definition in node
            .children_by_field_name("definitions", &mut cursor)
            .filter(|n| n.is_named())
        {
            if let Some(pattern_definition) = definition.child_by_field_name("pattern") {
                let name = pattern_definition
                    .child_by_field_name("name")
                    .ok_or_else(|| anyhow!("missing name of pattern definition"))?;
                let name = name.utf8_text(pattern.as_bytes())?;
                let name = name.trim();
                // todo check for duplicates?
                patterns.insert(name.to_owned(), file.to_owned());
            } else if let Some(predicate_definition) = definition.child_by_field_name("predicate") {
                let name = predicate_definition
                    .child_by_field_name("name")
                    .ok_or_else(|| anyhow!("missing name of pattern definition"))?;
                let name = name.utf8_text(pattern.as_bytes())?;
                let name = name.trim();
                // todo check for duplicates?
                predicates.insert(name.to_owned(), file.to_owned());
            } else if let Some(function_definition) = definition.child_by_field_name("function") {
                let name = function_definition
                    .child_by_field_name("name")
                    .ok_or_else(|| anyhow!("missing name of function definition"))?;
                let name = name.utf8_text(pattern.as_bytes())?;
                let name = name.trim();
                functions.insert(name.to_owned(), file.to_owned());
            } else if let Some(foreign_definition) = definition.child_by_field_name("foreign") {
                let name = foreign_definition
                    .child_by_field_name("name")
                    .ok_or_else(|| anyhow!("missing name of function definition"))?;
                let name = name.utf8_text(pattern.as_bytes())?;
                let name = name.trim();
                foreign_functions.insert(name.to_owned(), file.to_owned());
            } else {
                return Err(anyhow!(
                    "definition must be either a pattern, a predicate or a function"
                ));
            }
        }
        if node.child_by_field_name("pattern").is_some() {
            let name = file.strip_suffix(".grit").unwrap_or(file);
            patterns.insert(name.to_owned(), file.to_owned());
        }
    }
    let mut cursor = root.walk();
    for definition in root
        .children_by_field_name("definitions", &mut cursor)
        .filter(|n| n.is_named())
    {
        if let Some(pattern_definition) = definition.child_by_field_name("pattern") {
            let name = pattern_definition
                .child_by_field_name("name")
                .ok_or_else(|| anyhow!("missing name of pattern definition"))?;
            let name = name.utf8_text(src.as_bytes())?;
            let name = name.trim();
            // todo check for duplicates?
            patterns.remove(name);
        } else if let Some(predicate_definition) = definition.child_by_field_name("predicate") {
            let name = predicate_definition
                .child_by_field_name("name")
                .ok_or_else(|| anyhow!("missing name of pattern definition"))?;
            let name = name.utf8_text(src.as_bytes())?;
            let name = name.trim();
            // todo check for duplicates?
            predicates.remove(name);
        } else if let Some(function_definition) = definition.child_by_field_name("function") {
            let name = function_definition
                .child_by_field_name("name")
                .ok_or_else(|| anyhow!("missing name of function definition"))?;
            let name = name.utf8_text(src.as_bytes())?;
            let name = name.trim();
            functions.remove(name);
        } else if let Some(foreign_definition) = definition.child_by_field_name("foreign") {
            let name = foreign_definition
                .child_by_field_name("name")
                .ok_or_else(|| anyhow!("missing name of function definition"))?;
            let name = name.utf8_text(src.as_bytes())?;
            let name = name.trim();
            foreign_functions.remove(name);
        } else {
            return Err(anyhow!(
                "definition must be either a pattern, a predicate or a function"
            ));
        }
    }
    Ok(DefsToFilenames {
        patterns,
        predicates,
        functions,
        foreign_functions,
    })
}

fn filter_libs(
    libs: &BTreeMap<String, String>,
    src: &str,
    parser: &mut Parser,
    will_autowrap: bool,
) -> Result<Vec<(String, String)>> {
    let node_like = "nodeLike";
    let predicate_call = "predicateCall";
    let tree = parse_one(parser, src, DEFAULT_FILE_NAME)?;
    let DefsToFilenames {
        patterns: pattern_file,
        predicates: predicate_file,
        functions: function_file,
        foreign_functions: foreign_file,
    } = defs_to_filenames(libs, parser, tree.root_node(), src)?;
    let mut filtered: BTreeMap<String, String> = BTreeMap::new();
    // gross but necessary due to running these patterns befor and after each file

    let mut stack: Vec<(Tree, &str)> = if will_autowrap {
        let before_each_file = "before_each_file()";
        let before_tree = parse_one(parser, before_each_file, DEFAULT_FILE_NAME)?;
        let after_each_file = "after_each_file()";
        let after_tree = parse_one(parser, after_each_file, DEFAULT_FILE_NAME)?;

        vec![
            (tree, src),
            (before_tree, before_each_file),
            (after_tree, after_each_file),
        ]
    } else {
        vec![(tree, src)]
    };
    while let Some((tree, source)) = stack.pop() {
        let cursor = tree.walk();
        for n in traverse(CursorWrapper::new(cursor, source), Order::Pre)
            .map(|n| n.node)
            .filter(|n| n.is_named() && (n.kind() == node_like || n.kind() == predicate_call))
        {
            let name = n
                .child_by_field_name("name")
                .ok_or_else(|| anyhow!("missing name of nodeLike"))?;
            let name = name.utf8_text(source.as_bytes())?;
            let name = name.trim();
            if n.kind() == node_like {
                if let Some((tree, file_body)) =
                    find_definition_if_exists(&pattern_file, parser, libs, &mut filtered, name)?
                {
                    stack.push((tree, file_body));
                }
                if let Some((tree, file_body)) =
                    find_definition_if_exists(&function_file, parser, libs, &mut filtered, name)?
                {
                    stack.push((tree, file_body));
                }
                if let Some((tree, file_body)) =
                    find_definition_if_exists(&foreign_file, parser, libs, &mut filtered, name)?
                {
                    stack.push((tree, file_body));
                }
            } else if n.kind() == predicate_call {
                if let Some((tree, file_body)) =
                    find_definition_if_exists(&predicate_file, parser, libs, &mut filtered, name)?
                {
                    stack.push((tree, file_body));
                }
            }
        }
    }
    Ok(filtered.into_iter().collect_vec())
}

fn find_definition_if_exists<'a>(
    files: &BTreeMap<String, String>,
    parser: &mut Parser,
    libs: &'a BTreeMap<String, String>,
    filtered: &mut BTreeMap<String, String>,
    name: &str,
) -> Result<Option<(Tree, &'a String)>> {
    if let Some(file_name) = files.get(name) {
        if !filtered.contains_key(file_name) {
            if let Some(file_body) = libs.get(file_name) {
                filtered.insert(file_name.to_owned(), file_body.to_owned());
                let tree = parse_one(parser, file_body, file_name)?;
                return Ok(Some((tree, file_body)));
            }
        }
    };
    Ok(None)
}

pub struct CompilationResult {
    pub compilation_warnings: AnalysisLogs,
    pub problem: Problem,
}

#[cfg_attr(
    feature = "grit_tracing",
    instrument(name = "compile_pattern", skip(libs, default_lang, name, file_ranges))
)]
pub fn src_to_problem_libs(
    src: String,
    libs: &BTreeMap<String, String>,
    default_lang: TargetLanguage,
    name: Option<String>,
    file_ranges: Option<Vec<FileRange>>,
    custom_built_ins: Option<BuiltIns>,
    injected_limit: Option<usize>,
) -> Result<CompilationResult> {
    let mut parser = make_grit_parser()?;
    let src_tree = parse_one(&mut parser, &src, DEFAULT_FILE_NAME)?;
    let lang = TargetLanguage::from_tree(&src_tree, &src).unwrap_or(default_lang);
    src_to_problem_libs_for_language(
        src,
        libs,
        lang,
        name,
        file_ranges,
        &mut parser,
        custom_built_ins,
        injected_limit,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn src_to_problem_libs_for_language(
    src: String,
    libs: &BTreeMap<String, String>,
    lang: TargetLanguage,
    name: Option<String>,
    file_ranges: Option<Vec<FileRange>>,
    grit_parser: &mut Parser,
    custom_built_ins: Option<BuiltIns>,
    injected_limit: Option<usize>,
) -> Result<CompilationResult> {
    if src == "." {
        let error = ". never matches and should not be used as a pattern. Did you mean to run 'grit apply <pattern> .'?";
        bail!(error);
    }
    let src_tree = parse_one(grit_parser, &src, DEFAULT_FILE_NAME)?;

    let root = NodeWithSource::new(src_tree.root_node(), &src);
    let mut built_ins = BuiltIns::get_built_in_functions();
    if let Some(custom_built_ins) = custom_built_ins {
        built_ins.extend_builtins(custom_built_ins)?;
    }
    let mut logs: AnalysisLogs = vec![].into();
    let mut global_vars = BTreeMap::from([
        ("$new_files".to_owned(), NEW_FILES_INDEX),
        ("$filename".to_owned(), FILENAME_INDEX),
        ("$program".to_owned(), PROGRAM_INDEX),
        ("$absolute_filename".to_owned(), ABSOLUTE_PATH_INDEX),
    ]);
    let is_multifile = is_multifile(&root, libs, grit_parser)?;
    let has_limit = has_limit(&root, libs, grit_parser)?;
    let libs = filter_libs(libs, &src, grit_parser, !is_multifile)?;
    let DefinitionInfoKinds {
        pattern_indices: pattern_definition_indices,
        predicate_indices: predicate_definition_indices,
        function_indices: function_definition_indices,
        foreign_function_indices,
    } = get_definition_info(&libs, &root, grit_parser)?;

    let context = CompilationContext {
        file: DEFAULT_FILE_NAME,
        built_ins: &built_ins,
        lang: &lang,
        pattern_definition_info: &pattern_definition_indices,
        predicate_definition_info: &predicate_definition_indices,
        function_definition_info: &function_definition_indices,
        foreign_function_definition_info: &foreign_function_indices,
    };

    let DefinitionOutput {
        mut vars_array,
        mut pattern_definitions,
        predicate_definitions,
        function_definitions,
        foreign_function_definitions,
    } = get_definitions(
        &libs,
        &root,
        grit_parser,
        &context,
        &mut global_vars,
        &mut logs,
    )?;
    let scope_index = vars_array.len();
    vars_array.push(vec![]);
    let mut vars = BTreeMap::new();

    let mut node_context = NodeCompilationContext {
        compilation: &context,
        vars: &mut vars,
        vars_array: &mut vars_array,
        scope_index,
        global_vars: &mut global_vars,
        logs: &mut logs,
    };

    let pattern = if let Some(node) = root.child_by_field_name("pattern") {
        PatternCompiler::from_node(&node, &mut node_context)?
    } else {
        let long_message = "No pattern found.
        If you have written a pattern definition in the form `pattern myPattern() {{ }}`,
        try calling it by adding `myPattern()` to the end of your file.
        Check out the docs at https://docs.grit.io for help with writing patterns.";
        bail!("{}", long_message);
    };

    let pattern = auto_wrap_pattern(
        pattern,
        &mut pattern_definitions,
        !is_multifile,
        file_ranges,
        &mut node_context,
        injected_limit,
    )?;

    let problem = Problem::new(
        src,
        src_tree,
        pattern,
        lang,
        built_ins,
        is_multifile,
        has_limit,
        name,
        VariableLocations::new(vars_array),
        pattern_definitions,
        predicate_definitions,
        function_definitions,
        foreign_function_definitions,
    );
    let result = CompilationResult {
        compilation_warnings: logs,
        problem,
    };
    Ok(result)
}

pub fn parse_one(parser: &mut Parser, src: &str, file_name: &str) -> Result<Tree> {
    let tree = parser
        .parse(src, None)?
        .ok_or_else(|| anyhow!("parse error"))?;
    let parse_errors = grit_parsing_errors(&tree, src, file_name)?;
    if !parse_errors.is_empty() {
        let error = parse_errors[0].clone();
        bail!(error);
    }
    Ok(tree)
}

#[cfg(test)]
mod tests {
    use super::*;
    use grit_util::Language;
    use marzano_language::target_language::PatternLanguage;

    #[test]
    fn test_typescript_flavor() {
        let libs = BTreeMap::new();
        let pattern = r#"
            language js (typescript)
            `foo`
        "#
        .to_owned();
        let pattern = src_to_problem_libs(
            pattern,
            &libs,
            PatternLanguage::JavaScript.try_into().unwrap(),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let language = pattern.problem.language.language_name();
        assert_eq!(language, "TypeScript");
    }

    #[test]
    fn language_parsing() {
        let pattern_javascript = "language js(js_do_not_use)";
        let pattern_typescript = "language js(typescript)";
        let pattern_tsx = "language js(jsx)";
        let pattern_default = "language js";
        let pattern_default_fall_through = "language js(block)";
        let js: TargetLanguage = PatternLanguage::JavaScript.try_into().unwrap();
        let ts: TargetLanguage = PatternLanguage::TypeScript.try_into().unwrap();
        let tsx: TargetLanguage = PatternLanguage::Tsx.try_into().unwrap();
        assert_eq!(
            TargetLanguage::get_language(pattern_javascript)
                .unwrap()
                .language_name(),
            js.language_name()
        );
        assert_eq!(
            TargetLanguage::get_language(pattern_typescript)
                .unwrap()
                .language_name(),
            ts.language_name()
        );
        assert_eq!(
            TargetLanguage::get_language(pattern_tsx)
                .unwrap()
                .language_name(),
            tsx.language_name()
        );
        assert_eq!(
            TargetLanguage::get_language(pattern_default)
                .unwrap()
                .language_name(),
            tsx.language_name()
        );
        assert_eq!(
            TargetLanguage::get_language(pattern_default_fall_through)
                .unwrap()
                .language_name(),
            tsx.language_name()
        );
    }
}
