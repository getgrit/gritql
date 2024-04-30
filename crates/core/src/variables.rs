use crate::pattern_compiler::compiler::NodeCompilationContext;
use anyhow::Result;
use grit_pattern_matcher::{
    constants::GLOBAL_VARS_SCOPE_INDEX,
    pattern::{Variable, VariableSourceLocations},
};
use grit_util::Range;
use std::{collections::BTreeSet, path::Path};

pub(crate) fn variable_from_name(
    name: &str,
    context: &mut NodeCompilationContext,
) -> Result<Variable> {
    register_variable_optional_range(name, None, context)
}

pub(crate) fn get_variables(
    params: &[(String, Range)],
    context: &mut NodeCompilationContext,
) -> Result<Vec<(String, Variable)>> {
    params
        .iter()
        .map(|(name, range)| {
            let index = register_variable(name, *range, context)?;
            Ok((name.to_owned(), index))
        })
        .collect()
}

pub(crate) fn register_variable(
    name: &str,
    range: Range,
    context: &mut NodeCompilationContext,
) -> Result<Variable> {
    register_variable_optional_range(
        name,
        Some(FileLocation {
            range,
            file_name: context.compilation.file,
        }),
        context,
    )
}

struct FileLocation<'a> {
    file_name: Option<&'a Path>,
    range: Range,
}

fn register_variable_optional_range(
    name: &str,
    location: Option<FileLocation>,
    context: &mut NodeCompilationContext,
) -> Result<Variable> {
    let NodeCompilationContext {
        vars,
        vars_array,
        global_vars,
        scope_index,
        ..
    } = context;

    if let Some(i) = vars.get(name) {
        if let Some(FileLocation { range, .. }) = location {
            vars_array[*scope_index][*i].locations.insert(range);
        }
        return Ok(Variable::new(*scope_index, *i));
    }

    if let Some(i) = global_vars.get(name) {
        if let Some(FileLocation { range, file_name }) = location {
            if file_name.is_none() {
                vars_array[GLOBAL_VARS_SCOPE_INDEX][*i]
                    .locations
                    .insert(range);
            }
        }
        return Ok(Variable::new(GLOBAL_VARS_SCOPE_INDEX, *i));
    }
    let (name_map, scope_index) = if name.starts_with("$GLOBAL_") || name.starts_with("^GLOBAL_") {
        (global_vars, GLOBAL_VARS_SCOPE_INDEX)
    } else {
        (vars, *scope_index)
    };
    let scope = &mut vars_array[scope_index];
    let index = scope.len();
    name_map.insert(name.to_owned(), index);

    let (locations, file) = if let Some(FileLocation { range, file_name }) = location {
        let mut set = BTreeSet::new();
        set.insert(range);
        (set, file_name.to_owned())
    } else {
        // this currently only comes up with the $match variable which we autowrap, and is not
        // usually used by the user, but feels like this could potentially be a source of bugs
        (BTreeSet::new(), None)
    };
    scope.push(VariableSourceLocations {
        name: name.to_owned(),
        file: file.map(Path::to_owned),
        locations,
    });
    Ok(Variable::new(scope_index, index))
}
