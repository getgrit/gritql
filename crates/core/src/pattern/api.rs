use super::{FileOwner, InputRanges, Problem};
use crate::{fs, tree_sitter_serde::tree_sitter_node_to_json};
use anyhow::{bail, Result};
use im::Vector;
use marzano_language::target_language::TargetLanguage;
use marzano_util::analysis_logs::AnalysisLog as MarzanoAnalysisLog;
use marzano_util::position::{Position, Range, VariableMatch};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::path::PathBuf;
use std::{fmt, ops::Range as StdRange, str::FromStr, vec};
use tree_sitter::Tree;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "__typename")]
pub enum MatchResult {
    #[serde(rename = "PatternInfo")]
    PatternInfo(PatternInfo),
    #[serde(rename = "AllDone")]
    AllDone(AllDone),
    #[serde(rename = "Match")]
    Match(Match),
    #[serde(rename = "InputFile")]
    InputFile(InputFile),
    #[serde(rename = "Rewrite")]
    Rewrite(Rewrite),
    #[serde(rename = "CreateFile")]
    CreateFile(CreateFile),
    #[serde(rename = "RemoveFile")]
    RemoveFile(RemoveFile),
    #[serde(rename = "DoneFile")]
    DoneFile(DoneFile),
    #[serde(rename = "AnalysisLog")]
    AnalysisLog(AnalysisLog),
}

/// Make a path look the way provolone expects it to
/// Removes leading "./", or the root path if it's provided
fn normalize_path_in_project<'a>(path: &'a str, root_path: Option<&'a PathBuf>) -> &'a str {
    #[cfg(debug_assertions)]
    if let Some(root_path) = root_path {
        if !root_path.to_str().unwrap_or_default().ends_with('/') {
            panic!("root_path must end with a slash.");
        }
    }
    if let Some(root_path) = root_path {
        let root_path = root_path.to_str().unwrap();
        path.strip_prefix(root_path).unwrap_or(path)
    } else {
        path.strip_prefix("./").unwrap_or(path)
    }
}

impl MatchResult {
    /// Make the paths compatible with provolone expectations, by removing leading "./"
    pub fn normalize_paths(&mut self, root_path: Option<&PathBuf>) {
        match self {
            MatchResult::PatternInfo(pi) => {
                pi.source_file = normalize_path_in_project(&pi.source_file, root_path).to_owned()
            }
            MatchResult::AllDone(_) => {}
            MatchResult::Match(m) => {
                m.source_file = normalize_path_in_project(&m.source_file, root_path).to_owned()
            }
            MatchResult::InputFile(input_file) => {
                input_file.source_file =
                    normalize_path_in_project(&input_file.source_file, root_path).to_owned()
            }
            MatchResult::Rewrite(r) => {
                r.original.source_file =
                    normalize_path_in_project(&r.original.source_file, root_path).to_owned();
                r.rewritten.source_file =
                    normalize_path_in_project(&r.rewritten.source_file, root_path).to_owned()
            }
            MatchResult::CreateFile(cf) => {
                cf.rewritten.source_file =
                    normalize_path_in_project(&cf.rewritten.source_file, root_path).to_owned()
            }
            MatchResult::RemoveFile(rf) => {
                rf.original.source_file =
                    normalize_path_in_project(&rf.original.source_file, root_path).to_owned()
            }
            MatchResult::DoneFile(df) => {
                df.relative_file_path =
                    normalize_path_in_project(&df.relative_file_path, root_path).to_owned()
            }
            MatchResult::AnalysisLog(log) => {
                log.file = normalize_path_in_project(&log.file, root_path).to_owned()
            }
        }
    }

    pub(crate) fn file_to_match_result(
        file: &Vector<&FileOwner>,
        language: &TargetLanguage,
    ) -> Result<Option<MatchResult>> {
        if file.is_empty() {
            bail!("cannot have file with no versions")
        } else if file.len() == 1 {
            let file = file.last().unwrap();
            if file.new {
                return Ok(Some(MatchResult::CreateFile(CreateFile::file_to_create(
                    file.name.to_string_lossy().as_ref(),
                    &file.source,
                ))));
            } else if let Some(ranges) = &file.matches.borrow().input_matches {
                if ranges.suppressed {
                    return Ok(None);
                }
                return Ok(Some(MatchResult::Match(Match::file_to_match(
                    ranges,
                    &file.source,
                    file.name.to_string_lossy().as_ref(),
                    &file.tree,
                    language,
                ))));
            } else {
                return Ok(None);
            }
        } else {
            return Ok(Some(MatchResult::Rewrite(Rewrite::file_to_rewrite(
                file.front().unwrap(),
                file.back().unwrap(),
                language,
            )?)));
        }
    }

    pub fn file_name(&self) -> Option<&str> {
        match self {
            MatchResult::PatternInfo(pi) => Some(&pi.source_file),
            MatchResult::AllDone(_) => None,
            MatchResult::Match(m) => Some(&m.source_file),
            MatchResult::InputFile(input_file) => Some(&input_file.source_file),
            MatchResult::Rewrite(r) => Some(r.file_name()),
            MatchResult::CreateFile(cf) => Some(cf.file_name()),
            MatchResult::RemoveFile(rf) => Some(rf.file_name()),
            MatchResult::DoneFile(df) => Some(&df.relative_file_path),
            MatchResult::AnalysisLog(log) => Some(&log.file),
        }
    }

    fn extract_original_match(&self) -> Option<&Match> {
        match self {
            MatchResult::DoneFile(_)
            | MatchResult::AnalysisLog(_)
            | MatchResult::InputFile(_)
            | MatchResult::CreateFile(_)
            | MatchResult::AllDone(_)
            | MatchResult::PatternInfo(_) => None,
            MatchResult::Match(m)
            | MatchResult::RemoveFile(RemoveFile { original: m, .. })
            | MatchResult::Rewrite(Rewrite { original: m, .. }) => Some(m),
        }
    }

    /// Extract the original path, if any
    pub fn extract_original_path(&self) -> Option<&str> {
        let original_match = self.extract_original_match()?;
        Some(&original_match.source_file)
    }

    /// Given a MatchResult, create a MatchResult::Rewrite that suppresses the match.
    /// Returns None if it has any issues.
    pub fn get_rewrite_to_suppress(
        &self,
        language: &TargetLanguage,
        pattern_name: Option<&str>,
    ) -> Option<MatchResult> {
        let comment = make_suppress_comment(pattern_name, language);
        let ranges_starts = fs::extract_ranges(self)?
            .iter()
            .map(|r| r.start_byte as usize)
            .collect();

        let original_file_name = self.extract_original_path()?;
        let original_match = self.extract_original_match()?;

        let original_src = std::fs::read_to_string(original_file_name).ok()?;
        let rewritten_content =
            split_string_at_indices(&original_src, ranges_starts).join(&comment);
        let ef = EntireFile::file_to_entire_file(original_file_name, &rewritten_content, None);
        Some(MatchResult::Rewrite(Rewrite::new(
            original_match.clone(),
            ef,
        )))
    }
}

pub fn make_suppress_comment(pattern_name: Option<&str>, language: &TargetLanguage) -> String {
    match pattern_name {
        None => language.make_single_line_comment("grit-ignore"),
        Some(pattern_name) => {
            language.make_single_line_comment(&format!("grit-ignore {pattern_name}"))
        }
    }
}

fn split_string_at_indices(s: &str, indices: Vec<usize>) -> Vec<&str> {
    let mut result = Vec::new();
    let mut prev = 0;
    for &index in indices.iter() {
        if let Some(substring) = s.get(prev..index) {
            result.push(substring);
        }
        prev = index;
    }
    if let Some(substring) = s.get(prev..) {
        result.push(substring);
    }
    result
}

pub fn is_match(result: &MatchResult) -> bool {
    match result {
        MatchResult::AnalysisLog(_) => false,
        MatchResult::Match(_) => true,
        MatchResult::InputFile(_) => false,
        MatchResult::CreateFile(_) => true,
        MatchResult::RemoveFile(_) => true,
        MatchResult::Rewrite(_) => true,
        MatchResult::DoneFile(_) => false,
        MatchResult::AllDone(_) => false,
        MatchResult::PatternInfo(_) => false,
    }
}

pub trait FileMatchResult {
    fn file_name(&self) -> &str;
    fn ranges(&mut self) -> &Vec<Range>;
    // A verb representing the action taken on the file
    fn action() -> &'static str;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct PatternInfo {
    pub messages: Vec<Message>,
    pub variables: Vec<VariableMatch>,
    pub source_file: String,
    pub parsed_pattern: String,
    pub valid: bool,
}

impl PatternInfo {
    pub fn from_compiled(compiled: Problem, source_file: String) -> Self {
        let node = compiled.tree.root_node();
        let parsed_pattern =
            to_string_pretty(&tree_sitter_node_to_json(&node, &source_file, None)).unwrap();
        Self {
            messages: vec![],
            variables: compiled.compiled_vars(),
            source_file,
            parsed_pattern,
            valid: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct InputFile {
    pub source_file: String,
    pub syntax_tree: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub messages: Vec<Message>,
    pub variables: Vec<VariableMatch>,
    pub source_file: String,
    pub ranges: Vec<Range>,
    pub debug: String,
}

impl Match {
    fn file_to_match(
        match_ranges: &InputRanges,
        file: &str,
        name: &str,
        tree: &Tree,
        language: &TargetLanguage,
    ) -> Self {
        let input_file_debug_text = to_string_pretty(&tree_sitter_node_to_json(
            &tree.root_node(),
            file,
            Some(language),
        ))
        .unwrap();
        Self {
            debug: input_file_debug_text,
            source_file: name.to_owned(),
            ranges: match_ranges.ranges.to_owned(),
            variables: match_ranges.variables.to_owned(),
            messages: vec![],
        }
    }
}

impl FileMatchResult for Match {
    fn file_name(&self) -> &str {
        &self.source_file
    }
    fn ranges(&mut self) -> &Vec<Range> {
        &self.ranges
    }
    fn action() -> &'static str {
        "matched"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct EntireFile {
    pub messages: Vec<Message>,
    pub variables: Vec<VariableMatch>,
    pub source_file: String,
    pub content: String,
    pub byte_ranges: Option<Vec<ByteRange>>,
}

impl EntireFile {
    fn file_to_entire_file(name: &str, body: &str, byte_range: Option<&Vec<ByteRange>>) -> Self {
        Self {
            source_file: name.to_owned(),
            content: body.to_owned(),
            variables: vec![],
            messages: vec![],
            byte_ranges: byte_range.map(|r| r.to_owned()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Rewrite {
    pub original: Match,
    pub rewritten: EntireFile,
    pub ansi_summary: String,
    pub reason: Option<RewriteReason>,
}

impl From<Rewrite> for MatchResult {
    fn from(r: Rewrite) -> Self {
        MatchResult::Rewrite(r)
    }
}

impl Rewrite {
    fn file_to_rewrite(
        initial: &FileOwner,
        rewrite: &FileOwner,
        language: &TargetLanguage,
    ) -> Result<Self> {
        let original = if let Some(ranges) = &initial.matches.borrow().input_matches {
            Match::file_to_match(
                ranges,
                &initial.source,
                initial.name.to_string_lossy().as_ref(),
                &initial.tree,
                language,
            )
        } else {
            bail!("cannot have rewrite without matches")
        };
        let rewritten = EntireFile::file_to_entire_file(
            rewrite.name.to_string_lossy().as_ref(),
            &rewrite.source,
            rewrite.matches.borrow().byte_ranges.as_ref(),
        );
        Ok(Rewrite::new(original, rewritten))
    }
}

impl FileMatchResult for Rewrite {
    fn file_name(&self) -> &str {
        &self.rewritten.source_file
    }
    fn ranges(&mut self) -> &Vec<Range> {
        &self.original.ranges
    }
    fn action() -> &'static str {
        "rewritten"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct RewriteReason {
    pub metadata_json: Option<String>,
    pub source: RewriteSource,
    pub name: Option<String>,
    pub level: Option<EnforcementLevel>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub enum EnforcementLevel {
    None = 0,
    #[default]
    Info = 1,
    Warn = 2,
    Error = 3,
}

impl FromStr for EnforcementLevel {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "none" => Ok(EnforcementLevel::None),
            "info" => Ok(EnforcementLevel::Info),
            "warn" => Ok(EnforcementLevel::Warn),
            "error" => Ok(EnforcementLevel::Error),
            _ => bail!("'{}' is not a valid level", s),
        }
    }
}

impl fmt::Display for EnforcementLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnforcementLevel::None => write!(f, "none"),
            EnforcementLevel::Info => write!(f, "info"),
            EnforcementLevel::Warn => write!(f, "warning"),
            EnforcementLevel::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "UPPERCASE")]
pub enum RewriteSource {
    Agent,
    Gritql,
    Stdlib,
    Unknown,
}

impl Rewrite {
    pub fn new(original: Match, rewritten: EntireFile) -> Self {
        Self {
            original,
            rewritten,
            ansi_summary: String::new(),
            reason: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct CreateFile {
    pub rewritten: EntireFile,
    pub ansi_summary: String,
    range: Option<Vec<Range>>,
}

impl From<CreateFile> for MatchResult {
    fn from(r: CreateFile) -> Self {
        MatchResult::CreateFile(r)
    }
}

impl CreateFile {
    fn file_to_create(name: &str, body: &str) -> CreateFile {
        CreateFile {
            rewritten: EntireFile::file_to_entire_file(name, body, None),
            ansi_summary: String::new(),
            range: None,
        }
    }
}

impl FileMatchResult for CreateFile {
    fn file_name(&self) -> &str {
        &self.rewritten.source_file
    }
    fn ranges(&mut self) -> &Vec<Range> {
        match self.range {
            Some(ref r) => r,
            None => {
                let start = Position::first();
                let end = Position::last(&self.rewritten.content);
                self.range = Some(vec![Range::new(
                    start,
                    end,
                    0,
                    self.rewritten.content.len() as u32,
                )]);
                self.ranges()
            }
        }
    }
    fn action() -> &'static str {
        "created"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFile {
    pub original: Match,
    pub ansi_summary: String,
}

impl From<RemoveFile> for MatchResult {
    fn from(r: RemoveFile) -> Self {
        MatchResult::RemoveFile(r)
    }
}

impl FileMatchResult for RemoveFile {
    fn file_name(&self) -> &str {
        &self.original.source_file
    }
    fn ranges(&mut self) -> &Vec<Range> {
        &self.original.ranges
    }
    fn action() -> &'static str {
        "removed"
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct DoneFile {
    pub relative_file_path: String,
    #[serde(skip_serializing)]
    pub has_results: Option<bool>,
    #[serde(skip_serializing)]
    pub file_hash: Option<[u8; 32]>,
    #[serde(skip_serializing)]
    pub from_cache: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub message: String,
    pub range: Vec<Range>,
    pub variable_runtime_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct AllDone {
    pub processed: i32,
    pub found: i32,
    pub reason: AllDoneReason,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum AllDoneReason {
    NoInputPaths,
    AllMatchesFound,
    MaxResultsReached,
    Aborted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct ByteRange {
    pub start: usize,
    pub end: usize,
}

impl From<StdRange<usize>> for ByteRange {
    fn from(range: StdRange<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisLog {
    pub level: u16,
    pub message: String,
    pub position: Position,
    pub file: String,
    pub engine_id: String,
    pub range: Option<Range>,
    pub syntax_tree: Option<String>,
    pub source: Option<String>,
}

impl AnalysisLog {
    pub(crate) fn new_error(message: String, file: &str) -> Self {
        Self {
            level: 280,
            message,
            position: Position::first(),
            file: file.to_owned(),
            engine_id: "marzano".to_string(),
            range: None,
            syntax_tree: None,
            source: None,
        }
    }
}

impl From<MarzanoAnalysisLog> for AnalysisLog {
    fn from(log: MarzanoAnalysisLog) -> Self {
        Self {
            level: log.level.unwrap_or(280),
            message: log.message,
            position: log.position.unwrap_or_else(Position::first),
            file: log.file.unwrap_or_default(),
            engine_id: log.engine_id.unwrap_or_else(|| "marzano".to_string()),
            range: log.range,
            syntax_tree: log.syntax_tree,
            source: log.source,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AnalysisLogLevel {
    Error = 200,
    Warn = 300,
    Info = 400,
    Debug = 500,
}

pub fn derive_log_level(log: &AnalysisLog) -> AnalysisLogLevel {
    match log.level {
        0..=299 => AnalysisLogLevel::Error,
        300..=399 => AnalysisLogLevel::Warn,
        400..=499 => AnalysisLogLevel::Info,
        _ => AnalysisLogLevel::Debug,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path_in_project_no_root() {
        let path = "./src/main.rs";
        assert_eq!(normalize_path_in_project(path, None), "src/main.rs");
    }

    #[test]
    fn test_normalize_path_in_project_with_root() {
        let path = "/home/user/project/src/main.rs";
        let root_path = PathBuf::from("/home/user/project/");
        assert_eq!(
            normalize_path_in_project(path, Some(&root_path)),
            "src/main.rs"
        );
    }

    #[test]
    #[should_panic]
    fn test_normalize_path_in_project_with_root_no_slash() {
        let path = "/home/user/project/src/main.rs";
        let root_path = PathBuf::from("/home/user/project");
        normalize_path_in_project(path, Some(&root_path));
    }

    #[test]
    fn test_normalize_path_in_project_already_normalized() {
        let path = "src/main.rs";
        assert_eq!(normalize_path_in_project(path, None), "src/main.rs");
    }
}
