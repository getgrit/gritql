use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{includes::Includes, patterns::Pattern, variable::VariableSourceLocations};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct IncludesCompiler;

impl NodeCompiler for IncludesCompiler {
    type TargetPattern = Includes;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let includes = node
            .child_by_field_name("includes")
            .ok_or_else(|| anyhow!("missing includes of patternIncludes"))?;
        let includes = Pattern::from_node(
            &includes,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        Ok(Includes::new(includes))
    }
}
