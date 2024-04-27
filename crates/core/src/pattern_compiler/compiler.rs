use super::{
    builder::PatternBuilder,
    function_definition_compiler::{
        ForeignFunctionDefinitionCompiler, GritFunctionDefinitionCompiler,
    },
    pattern_compiler::PatternCompiler,
    pattern_definition_compiler::PatternDefinitionCompiler,
    predicate_definition_compiler::PredicateDefinitionCompiler,
    NodeCompiler,
};
use crate::{
    built_in_functions::BuiltIns,
    foreign_function_definition::ForeignFunctionDefinition,
    problem::{MarzanoQueryContext, Problem},
};
use anyhow::{anyhow, bail, Result};
use grit_pattern_matcher::{
    constants::{
        DEFAULT_FILE_NAME,
    },
    pattern::{
        GritFunctionDefinition, PatternDefinition, PredicateDefinition,
        VariableSourceLocations,
    },
};
use grit_util::{traverse, AnalysisLogs, Ast, AstNode, FileRange, Order, Range, VariableMatch};
use itertools::Itertools;
use marzano_language::{
    self, grit_parser::MarzanoGritParser, language::Tree, target_language::TargetLanguage,
};
use marzano_util::node_with_source::NodeWithSource;
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsStr,
    path::Path,
    str::Utf8Error,
    vec,
};

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

pub(crate) struct DefinitionInfoKinds {
    pub(crate) pattern_indices: BTreeMap<String, DefinitionInfo>,
    pub(crate) predicate_indices: BTreeMap<String, DefinitionInfo>,
    pub(crate) function_indices: BTreeMap<String, DefinitionInfo>,
    pub(crate) foreign_function_indices: BTreeMap<String, DefinitionInfo>,
}

pub(crate) fn get_definition_info(
    libs: &[(String, String)],
    root: &NodeWithSource,
    parser: &mut MarzanoGritParser,
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
        let tree = parser.parse_file(pattern, Some(Path::new(file)))?;
        let root = tree.root_node();
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
    node: &NodeWithSource,
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

pub(crate) struct DefinitionOutput {
    pub(crate) vars_array: Vec<Vec<VariableSourceLocations>>,
    pub(crate) pattern_definitions: Vec<PatternDefinition<MarzanoQueryContext>>,
    pub(crate) predicate_definitions: Vec<PredicateDefinition<MarzanoQueryContext>>,
    pub(crate) function_definitions: Vec<GritFunctionDefinition<MarzanoQueryContext>>,
    pub(crate) foreign_function_definitions: Vec<ForeignFunctionDefinition>,
}

pub(crate) fn get_definitions(
    libs: &[(String, String)],
    source_file: &NodeWithSource,
    parser: &mut MarzanoGritParser,
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

        let tree = parser.parse_file(pattern, Some(Path::new(file)))?;
        let root = tree.root_node();
        node_to_definitions(
            &root,
            &mut node_context,
            &mut pattern_definitions,
            &mut predicate_definitions,
            &mut function_definitions,
            &mut foreign_function_definitions,
        )?;

        if let Some(bare_pattern) = root.child_by_field_name("pattern") {
            let mut local_vars = BTreeMap::new();
            let (scope_index, mut local_context) = create_scope!(node_context, local_vars);
            let path = Path::new(file);
            let Some(name) = path.file_stem().and_then(OsStr::to_str) else {
                bail!("failed to get pattern name from definition in file {file}");
            };

            let body = PatternCompiler::from_node(&bare_pattern, &mut local_context)?;
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
        source_file,
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
    parser: &mut MarzanoGritParser,
    root: NodeWithSource,
) -> Result<DefsToFilenames> {
    let mut patterns = BTreeMap::new();
    let mut predicates = BTreeMap::new();
    let mut functions = BTreeMap::new();
    let mut foreign_functions = BTreeMap::new();
    for (file, pattern) in libs.iter() {
        let tree = parser.parse_file(pattern, Some(Path::new(file)))?;
        let node = tree.root_node();
        for definition in node.named_children_by_field_name("definitions") {
            if let Some(pattern_definition) = definition.child_by_field_name("pattern") {
                let name = pattern_definition
                    .child_by_field_name("name")
                    .ok_or_else(|| anyhow!("missing name of pattern definition"))?;
                let name = name.text()?;
                let name = name.trim();
                // todo check for duplicates?
                patterns.insert(name.to_owned(), file.to_owned());
            } else if let Some(predicate_definition) = definition.child_by_field_name("predicate") {
                let name = predicate_definition
                    .child_by_field_name("name")
                    .ok_or_else(|| anyhow!("missing name of pattern definition"))?;
                let name = name.text()?;
                let name = name.trim();
                // todo check for duplicates?
                predicates.insert(name.to_owned(), file.to_owned());
            } else if let Some(function_definition) = definition.child_by_field_name("function") {
                let name = function_definition
                    .child_by_field_name("name")
                    .ok_or_else(|| anyhow!("missing name of function definition"))?;
                let name = name.text()?;
                let name = name.trim();
                functions.insert(name.to_owned(), file.to_owned());
            } else if let Some(foreign_definition) = definition.child_by_field_name("foreign") {
                let name = foreign_definition
                    .child_by_field_name("name")
                    .ok_or_else(|| anyhow!("missing name of function definition"))?;
                let name = name.text()?;
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

    for definition in root.named_children_by_field_name("definitions") {
        if let Some(pattern_definition) = definition.child_by_field_name("pattern") {
            let name = pattern_definition
                .child_by_field_name("name")
                .ok_or_else(|| anyhow!("missing name of pattern definition"))?;
            let name = name.text()?;
            let name = name.trim();
            // todo check for duplicates?
            patterns.remove(name);
        } else if let Some(predicate_definition) = definition.child_by_field_name("predicate") {
            let name = predicate_definition
                .child_by_field_name("name")
                .ok_or_else(|| anyhow!("missing name of pattern definition"))?;
            let name = name.text()?;
            let name = name.trim();
            // todo check for duplicates?
            predicates.remove(name);
        } else if let Some(function_definition) = definition.child_by_field_name("function") {
            let name = function_definition
                .child_by_field_name("name")
                .ok_or_else(|| anyhow!("missing name of function definition"))?;
            let name = name.text()?;
            let name = name.trim();
            functions.remove(name);
        } else if let Some(foreign_definition) = definition.child_by_field_name("foreign") {
            let name = foreign_definition
                .child_by_field_name("name")
                .ok_or_else(|| anyhow!("missing name of function definition"))?;
            let name = name.text()?;
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

pub(crate) fn filter_libs(
    libs: &BTreeMap<String, String>,
    src: &str,
    parser: &mut MarzanoGritParser,
    will_autowrap: bool,
) -> Result<Vec<(String, String)>> {
    let node_like = "nodeLike";
    let predicate_call = "predicateCall";
    let tree = parser.parse_file(src, Some(Path::new(DEFAULT_FILE_NAME)))?;
    let DefsToFilenames {
        patterns: pattern_file,
        predicates: predicate_file,
        functions: function_file,
        foreign_functions: foreign_file,
    } = defs_to_filenames(libs, parser, tree.root_node())?;
    let mut filtered: BTreeMap<String, String> = BTreeMap::new();
    // gross but necessary due to running these patterns befor and after each file

    let mut stack: Vec<Tree> = if will_autowrap {
        let before_each_file = "before_each_file()";
        let before_tree =
            parser.parse_file(before_each_file, Some(Path::new(DEFAULT_FILE_NAME)))?;
        let after_each_file = "after_each_file()";
        let after_tree = parser.parse_file(after_each_file, Some(Path::new(DEFAULT_FILE_NAME)))?;

        vec![tree, before_tree, after_tree]
    } else {
        vec![tree]
    };

    while let Some(tree) = stack.pop() {
        let root = tree.root_node();
        let cursor = root.walk();
        for n in traverse(cursor, Order::Pre).filter(|n| {
            n.node.is_named() && (n.node.kind() == node_like || n.node.kind() == predicate_call)
        }) {
            let name = n
                .child_by_field_name("name")
                .ok_or_else(|| anyhow!("missing name of nodeLike"))?;
            let name = name.text()?;
            let name = name.trim();
            if n.node.kind() == node_like {
                if let Some(tree) =
                    find_definition_if_exists(&pattern_file, parser, libs, &mut filtered, name)?
                {
                    stack.push(tree);
                }
                if let Some(tree) =
                    find_definition_if_exists(&function_file, parser, libs, &mut filtered, name)?
                {
                    stack.push(tree);
                }
                if let Some(tree) =
                    find_definition_if_exists(&foreign_file, parser, libs, &mut filtered, name)?
                {
                    stack.push(tree);
                }
            } else if n.node.kind() == predicate_call {
                if let Some(tree) =
                    find_definition_if_exists(&predicate_file, parser, libs, &mut filtered, name)?
                {
                    stack.push(tree);
                }
            }
        }
    }
    Ok(filtered.into_iter().collect_vec())
}

fn find_definition_if_exists(
    files: &BTreeMap<String, String>,
    parser: &mut MarzanoGritParser,
    libs: &BTreeMap<String, String>,
    filtered: &mut BTreeMap<String, String>,
    name: &str,
) -> Result<Option<Tree>> {
    if let Some(file_name) = files.get(name) {
        if !filtered.contains_key(file_name) {
            if let Some(file_body) = libs.get(file_name) {
                filtered.insert(file_name.to_owned(), file_body.to_owned());
                let tree = parser.parse_file(file_body, Some(Path::new(file_name)))?;
                return Ok(Some(tree));
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
    instrument(
        name = "src_to_problem_libs",
        skip(libs, default_lang, name, file_ranges)
    )
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
    let mut parser = MarzanoGritParser::new()?;
    let src_tree = parser.parse_file(&src, Some(Path::new(DEFAULT_FILE_NAME)))?;
    let lang = TargetLanguage::from_tree(&src_tree).unwrap_or(default_lang);
    let builder = PatternBuilder::start(src, libs, lang, name, &mut parser, custom_built_ins)?;
    builder.compile(file_ranges, injected_limit)
}

#[derive(Debug, Default)]
pub struct VariableLocations {
    pub(crate) locations: Vec<Vec<VariableSourceLocations>>,
}

impl VariableLocations {
    pub(crate) fn new(locations: Vec<Vec<VariableSourceLocations>>) -> Self {
        Self { locations }
    }

    pub(crate) fn compiled_vars(&self) -> Vec<VariableMatch> {
        let mut variables = vec![];
        for (i, scope) in self.locations.iter().enumerate() {
            for (j, var) in scope.iter().enumerate() {
                if var.file == DEFAULT_FILE_NAME {
                    variables.push(VariableMatch {
                        name: var.name.clone(),
                        scoped_name: format!("{}_{}_{}", i, j, var.name),
                        ranges: var.locations.iter().cloned().collect(),
                    });
                }
            }
        }
        variables
    }
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
