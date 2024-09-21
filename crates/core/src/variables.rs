use crate::pattern_compiler::compiler::NodeCompilationContext;
use anyhow::Result;
use grit_pattern_matcher::{
    constants::{DEFAULT_FILE_NAME, GLOBAL_VARS_SCOPE_INDEX},
    pattern::{Variable, VariableSourceLocations},
};
use grit_util::ByteRange;
use std::collections::BTreeSet;

pub(crate) fn variable_from_name(
    name: &str,
    context: &mut NodeCompilationContext,
) -> Result<Variable> {
    register_variable_optional_range(name, None, context)
}

pub(crate) fn get_variables(
    params: &[(String, ByteRange)],
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
    range: ByteRange,
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
    file_name: &'a str,
    range: ByteRange,
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
            if file_name == DEFAULT_FILE_NAME {
                vars_array[GLOBAL_VARS_SCOPE_INDEX as usize][*i]
                    .locations
                    .insert(range);
            }
        }
        return Ok(Variable::new(GLOBAL_VARS_SCOPE_INDEX as usize, *i));
    }
    if name.starts_with("$GLOBAL_") || name.starts_with("^GLOBAL_") {
        let name_map = global_vars;
        let scope_index = GLOBAL_VARS_SCOPE_INDEX;
        let scope = &mut vars_array[scope_index as usize];
        let index = scope.len();
        name_map.insert(name.to_owned(), index);

        let (locations, file) = if let Some(FileLocation { range, file_name }) = location {
            let mut set = BTreeSet::new();
            set.insert(range);
            (set, file_name.to_owned())
        } else {
            // this currently only comes up with the $match variable which we autowrap, and is not
            // usually used by the user, but feels like this could potentially be a source of bugs
            (BTreeSet::new(), DEFAULT_FILE_NAME.to_owned())
        };
        scope.push(VariableSourceLocations {
            name: name.to_owned(),
            file,
            locations,
        });
        Ok(Variable::new(scope_index as usize, index))
    } else {
        // TODO: replicate VariableSourceLocations for local vars
        let scope = &mut vars_array[*scope_index as usize];
        let index = scope.len();
        scope.push(VariableSourceLocations {
            name: name.to_owned(),
            file: DEFAULT_FILE_NAME.to_owned(),
            locations: BTreeSet::new(),
        });
        let var = Variable::new(*scope_index as usize, index);

        Ok(var)
    }
}
