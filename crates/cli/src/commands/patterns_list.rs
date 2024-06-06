use anyhow::Result;
use marzano_core::api::EnforcementLevel;
use marzano_gritmodule::config::{DefinitionSource, ResolvedGritDefinition};

use crate::{
    flags::GlobalFormatFlags,
    lister::{list_applyables, Listable},
    resolver::resolve_from_flags_or_cwd,
};

use super::list::ListArgs;

impl Listable for ResolvedGritDefinition {
    fn name(&self) -> &str {
        self.local_name.as_str()
    }

    fn source(&self) -> Option<DefinitionSource> {
        Some(self.module.clone())
    }

    fn language(&self) -> Option<&str> {
        Some(self.language.language_name())
    }

    fn tags(&self) -> Vec<&str> {
        let mut normal_tags = self.tags().iter().map(|t| t.as_str()).collect::<Vec<_>>();
        let more_tags = match self.level() {
            EnforcementLevel::Warn => vec!["warn"],
            EnforcementLevel::Error => vec!["error"],
            _ => vec![],
        };
        normal_tags.extend(more_tags);
        normal_tags
    }
}

pub(crate) async fn run_patterns_list(arg: ListArgs, parent: GlobalFormatFlags) -> Result<()> {
    let (resolved, curr_repo) = resolve_from_flags_or_cwd(&parent, &arg.source).await?;
    list_applyables(false, false, resolved, arg.level, &parent, curr_repo).await
}
