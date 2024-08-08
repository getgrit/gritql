use anyhow::Result;
use clap::Args;
use indicatif::MultiProgress;

use marzano_gritmodule::searcher::find_workflow_file_from;
use marzano_messenger::emit::ApplyDetails;
use serde::Serialize;
use std::env::current_dir;
use std::path::PathBuf;

use crate::flags::GlobalFormatFlags;

#[cfg(feature = "workflows_v2")]
use super::apply_migration::{run_apply_migration, ApplyMigrationArgs};
use super::{
    apply_pattern::{run_apply_pattern, ApplyPatternArgs},
    filters::SharedFilterArgs,
};

#[derive(Args, Debug, Serialize)]
pub struct ApplyArgs {
    #[clap(
        index = 1,
        help = "Pattern or workflow to apply",
        long_help = "The pattern to apply, in a few forms:
  - A pattern name (ex. `raw_no_console_log`)
  - A pattern by itself (ex. ``'`console.log` => `console.error`'``)
  - A pattern call, with arguments (ex. `'openai_main(client=`openai`)'`)
  - A path to a pattern file (ex. `./patterns/raw_no_console_log.grit`)
  - A workflow name (ex. `lint`)"
    )]
    pattern_or_workflow: String,
    #[clap(index = 2, value_parser, default_value = ".")]
    paths: Vec<PathBuf>,

    #[cfg(feature = "workflows_v2")]
    #[command(flatten)]
    apply_migration_args: ApplyMigrationArgs,

    #[command(flatten)]
    apply_pattern_args: ApplyPatternArgs,

    #[command(flatten)]
    shared_apply_args: SharedFilterArgs,
}

pub(crate) async fn run_apply(
    args: ApplyArgs,
    multi: MultiProgress,
    details: &mut ApplyDetails,
    flags: &GlobalFormatFlags,
) -> Result<()> {
    #[cfg(feature = "workflows_v2")]
    {
        let current_dir = current_dir()?;
        let current_repo_root = marzano_gritmodule::fetcher::LocalRepo::from_dir(&current_dir)
            .await
            .map(|repo| repo.root())
            .transpose()?;

        let ranges = crate::commands::filters::extract_filter_diff(
            &args.shared_apply_args,
            current_repo_root.as_ref(),
        )?;

        #[cfg(feature = "remote_workflows")]
        if args.apply_migration_args.remote {
            return crate::workflows::run_remote_workflow(
                args.pattern_or_workflow,
                args.apply_migration_args,
                ranges,
            )
            .await;
        }

        let custom_workflow = find_workflow_file_from(current_dir, &args.pattern_or_workflow).await;
        if let Some(custom_workflow) = custom_workflow {
            return run_apply_migration(
                custom_workflow,
                args.paths,
                ranges,
                args.apply_migration_args,
                flags,
                args.apply_pattern_args.visibility,
            )
            .await;
        }
    }

    run_apply_pattern(
        args.pattern_or_workflow,
        args.shared_apply_args,
        args.paths,
        args.apply_pattern_args,
        multi,
        details,
        None,
        None,
        flags,
        None,
    )
    .await
}

// TODO files that call stdlib functions don't work here
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use anyhow::Result;
    use indicatif::MultiProgress;
    use marzano_messenger::emit::ApplyDetails;
    use std::env;

    use crate::{
        commands::{apply_migration::ApplyMigrationArgs, apply_pattern::ApplyPatternArgs},
        flags::GlobalFormatFlags,
    };

    use marzano_test_utils::fixtures::get_fixtures_root;

    use super::{run_apply, ApplyArgs};

    pub const GRIT_GLOBAL_DIR_ENV: &str = "GRIT_GLOBAL_DIR";

    async fn test_apply(pattern: String, paths: Vec<PathBuf>) -> Result<()> {
        let multi = MultiProgress::new();
        let mut details = ApplyDetails {
            matched: 0,
            rewritten: 0,
            named_pattern: None,
        };
        let apply_migration_args = ApplyMigrationArgs::default();
        let apply_pattern_args = ApplyPatternArgs::default();
        let args = ApplyArgs {
            pattern_or_workflow: pattern,
            paths,
            apply_migration_args,
            apply_pattern_args,
            shared_apply_args: Default::default(),
        };
        run_apply(args, multi, &mut details, &GlobalFormatFlags::default()).await?;
        Ok(())
    }

    #[ignore = "interacts weirdly with cache::tests::test_mismatches_cache"]
    #[tokio::main]
    #[test]
    async fn debuggable_apply() -> Result<()> {
        let tempdir = tempfile::tempdir()?;
        let grit_global_dir = tempfile::tempdir()?;
        env::set_var(GRIT_GLOBAL_DIR_ENV, grit_global_dir.path());
        let fixtures_root = get_fixtures_root(env!("CARGO_MANIFEST_DIR"))?;
        let pattern_grit = fixtures_root.join("stdlib").join("raw_no_console_log.grit");
        let pattern_dest = tempdir.path().join("raw_no_console_log.grit");
        fs_err::copy(pattern_grit, pattern_dest.clone())?;
        let input = fixtures_root.join("stdlib").join("simple.js");
        let input_dest = tempdir.path().join("simple.js");
        fs_err::copy(input, &input_dest)?;
        env::set_current_dir(tempdir.path())?;
        test_apply(
            "raw_no_console_log.grit".to_string(),
            vec![input_dest.clone()],
        )
        .await?;
        let content = fs_err::read_to_string(&input_dest)?;
        assert_eq!(content, "\n".to_owned());
        Ok(())
    }

    #[ignore = "interacts weirdly with cache::tests::test_mismatches_cache"]
    #[tokio::main]
    #[test]
    async fn applies_openai_python() -> Result<()> {
        let tempdir = tempfile::tempdir()?;
        let grit_global_dir = tempfile::tempdir()?;
        env::set_var(GRIT_GLOBAL_DIR_ENV, grit_global_dir.path());
        let fixtures_root = get_fixtures_root(env!("CARGO_MANIFEST_DIR"))?;
        let pattern_grit = fixtures_root.join("openai").join("pattern.grit");
        let pattern_dest = tempdir.path().join("pattern.grit");
        fs_err::copy(pattern_grit, pattern_dest.clone())?;
        let input = fixtures_root.join("openai").join("input.py");
        let input_dest = tempdir.path().join("input.py");
        fs_err::copy(input, &input_dest)?;
        env::set_current_dir(tempdir.path())?;
        test_apply("pattern.grit".to_string(), vec![input_dest.clone()]).await?;
        Ok(())
    }
}
