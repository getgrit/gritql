use crate::{
    flags::GlobalFormatFlags,
    jsonl::JSONLineMessenger,
    resolver::{get_grit_files_from_cwd, GritModuleResolver},
};
use anyhow::{bail, Result};
use clap::Args;
use grit_util::Position;
use marzano_core::{
    api::{AnalysisLog, MatchResult, PatternInfo},
    parse::parse_input_file,
};
use marzano_language::target_language::{PatternLanguage, TargetLanguage};
use marzano_messenger::{
    emit::{Messager, VisibilityLevels},
    output_mode::OutputMode,
};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;
use tokio::fs;

#[derive(Args, Debug, Serialize)]
pub struct ParseArgs {
    #[clap(value_parser)]
    paths: Vec<PathBuf>,
}

#[derive(Deserialize)]
pub struct ParseInput {
    pub pattern_body: String,
    paths: Vec<PathBuf>,
}

impl From<ParseInput> for ParseArgs {
    fn from(input: ParseInput) -> Self {
        Self { paths: input.paths }
    }
}

pub(crate) async fn run_parse(
    arg: ParseArgs,
    parent: GlobalFormatFlags,
    pattern_body: Option<String>,
) -> Result<()> {
    if !parent.jsonl {
        bail!("Only JSONL output is supported for parse command");
    }

    let mut emitter = JSONLineMessenger::new(io::stdout(), OutputMode::default());

    let parse_input = ParseInput {
        pattern_body: pattern_body.to_owned().unwrap_or_default(),
        paths: arg.paths,
    };

    // we should be reading the default from a config
    let lang: TargetLanguage = PatternLanguage::get_language(&parse_input.pattern_body)
        .unwrap_or_default()
        .try_into()?;
    let visibility = VisibilityLevels::Hidden;

    if let Some(body) = pattern_body {
        let result = parse_one_pattern(body, None).await?;
        emitter.emit(&result, &visibility)?;
    }

    for path in parse_input.paths {
        let input = fs::read_to_string(&path).await?;

        let match_result = if path.extension().eq(&Some("grit".as_ref())) {
            parse_one_pattern(input, Some(&path)).await?
        } else {
            let input_file = match parse_input_file(&lang, &input, &path) {
                Ok(input_file) => input_file,
                Err(_) => {
                    continue;
                }
            };
            MatchResult::InputFile(input_file)
        };
        emitter.emit(&match_result, &visibility)?;
    }

    Ok(())
}

async fn parse_one_pattern(body: String, path: Option<&PathBuf>) -> Result<MatchResult> {
    let current_dir = std::env::current_dir()?;
    let resolver = GritModuleResolver::new(current_dir.to_str().unwrap());
    let lang = PatternLanguage::get_language(&body);
    let pattern = resolver.make_pattern(&body, None)?;
    let pattern_libs = get_grit_files_from_cwd().await?;
    let pattern_libs = pattern_libs.get_language_directory_or_default(lang)?;
    let problem = match pattern.compile(&pattern_libs, None, None, None) {
        Ok(problem) => problem,
        Err(e) => {
            let log = match e.downcast::<grit_util::AnalysisLog>() {
                Ok(al) => MatchResult::AnalysisLog(AnalysisLog::from(al)),
                Err(er) => MatchResult::AnalysisLog(AnalysisLog {
                    level: 200,
                    message: er.to_string(),
                    position: Position::first(),
                    file: if let Some(path) = path {
                        path.to_string_lossy().to_string()
                    } else {
                        "PlaygroundPattern".to_string()
                    },
                    engine_id: "marzano".to_string(),
                    syntax_tree: None,
                    range: None,
                    source: None,
                }),
            };
            return Ok(log);
        }
    };
    let pinfo = PatternInfo::from_compiled(problem.problem, body.clone());
    let result = MatchResult::PatternInfo(pinfo);
    Ok(result)
}
