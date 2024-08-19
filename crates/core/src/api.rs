use crate::{fs, problem::Problem, tree_sitter_serde::tree_sitter_node_to_json};
use anyhow::{bail, Result};
use grit_pattern_matcher::file_owners::FileOwner;
pub use grit_util::ByteRange;
use grit_util::{AnalysisLog as GritAnalysisLog, Ast, Position, Range, VariableMatch};
use im::Vector;
use marzano_language::grit_ts_node::grit_node_types;
use marzano_language::language::{MarzanoLanguage, Tree};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::path::PathBuf;
use std::{fmt, str::FromStr, vec};
use uuid::Uuid;

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

impl MatchResult {
    pub fn is_match(&self) -> bool {
        is_match(self)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, MatchResult::AnalysisLog(log) if log.level < 400)
    }

    pub fn kind(&self) -> &'static str {
        match self {
            MatchResult::PatternInfo(_) => "PatternInfo",
            MatchResult::AllDone(_) => "AllDone",
            MatchResult::Match(_) => "Match",
            MatchResult::InputFile(_) => "InputFile",
            MatchResult::Rewrite(_) => "Rewrite",
            MatchResult::CreateFile(_) => "CreateFile",
            MatchResult::RemoveFile(_) => "RemoveFile",
            MatchResult::DoneFile(_) => "DoneFile",
            MatchResult::AnalysisLog(_) => "AnalysisLog",
        }
    }
}

/// Make a path look the way provolone expects it to
/// Removes leading "./", or the root path if it's provided
fn normalize_path_in_project<'a>(path: &'a str, root_path: Option<&'a PathBuf>) -> &'a str {
    if let Some(root_path) = root_path {
        let basic = path
            .strip_prefix(root_path.to_string_lossy().as_ref())
            .unwrap_or(path);
        // Stip the leading / if it's there
        basic.strip_prefix('/').unwrap_or(basic)
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
        file: &Vector<&FileOwner<Tree>>,
    ) -> Result<Option<MatchResult>> {
        if file.is_empty() {
            bail!("cannot have file with no versions")
        } else if file.len() == 1 {
            let file = file.last().unwrap();
            if file.new {
                return Ok(Some(MatchResult::CreateFile(CreateFile::file_to_create(
                    file.name.to_string_lossy().as_ref(),
                    &file.tree.source,
                ))));
            } else if let Some(ranges) = &file.matches.borrow().input_matches {
                if ranges.suppressed {
                    return Ok(None);
                }
                let fm = EntireFile::from_file(file)?;
                return Ok(Some(MatchResult::Match(fm.into())));
            } else {
                return Ok(None);
            }
        } else {
            return Ok(Some(MatchResult::Rewrite(Rewrite::file_to_rewrite(
                file.front().unwrap(),
                file.back().unwrap(),
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

    fn extract_original_match(&self) -> Option<EntireFile> {
        match self {
            MatchResult::DoneFile(_)
            | MatchResult::AnalysisLog(_)
            | MatchResult::InputFile(_)
            | MatchResult::CreateFile(_)
            | MatchResult::AllDone(_)
            | MatchResult::PatternInfo(_) => None,
            MatchResult::Match(m) => Some(m.clone().into()),
            MatchResult::RemoveFile(RemoveFile { original: m, .. }) => Some(m.clone()),
            MatchResult::Rewrite(Rewrite { original: m, .. }) => Some(m.clone()),
        }
    }

    fn extract_reason(&self) -> Option<&MatchReason> {
        match self {
            MatchResult::Match(Match { reason: r, .. })
            | MatchResult::RemoveFile(RemoveFile { reason: r, .. })
            | MatchResult::Rewrite(Rewrite { reason: r, .. })
            | MatchResult::CreateFile(CreateFile { reason: r, .. }) => r.as_ref(),
            MatchResult::PatternInfo(_)
            | MatchResult::AllDone(_)
            | MatchResult::InputFile(_)
            | MatchResult::DoneFile(_)
            | MatchResult::AnalysisLog(_) => None,
        }
    }

    /// Extract the original path, if any
    pub fn extract_original_path(&self) -> Option<&str> {
        match self {
            MatchResult::DoneFile(_)
            | MatchResult::AnalysisLog(_)
            | MatchResult::InputFile(_)
            | MatchResult::CreateFile(_)
            | MatchResult::AllDone(_)
            | MatchResult::PatternInfo(_) => None,
            MatchResult::Match(m) => Some(&m.source_file),
            MatchResult::RemoveFile(r) => Some(&r.original.source_file),
            MatchResult::Rewrite(r) => Some(&r.original.source_file),
        }
    }

    /// Get the original content
    pub fn extract_original_content(&self) -> Option<&str> {
        match self {
            MatchResult::DoneFile(_)
            | MatchResult::AnalysisLog(_)
            | MatchResult::InputFile(_)
            | MatchResult::CreateFile(_)
            | MatchResult::AllDone(_)
            | MatchResult::PatternInfo(_) => None,
            MatchResult::Match(m) => m.content().ok(),
            MatchResult::RemoveFile(r) => r.original.content.as_deref(),
            MatchResult::Rewrite(r) => r.original.content.as_deref(),
        }
    }

    pub fn get_ranges(&self) -> Option<&Vec<Range>> {
        fs::extract_ranges(self)
    }

    /// Given a MatchResult, create a MatchResult::Rewrite that suppresses the match.
    /// Returns None if it has any issues.
    pub fn get_rewrite_to_suppress<'a>(
        &self,
        language: &impl MarzanoLanguage<'a>,
        pattern_name: Option<&str>,
    ) -> Option<MatchResult> {
        let comment = make_suppress_comment(pattern_name, language);
        let ranges_starts = fs::extract_ranges(self)?
            .iter()
            .map(|r| r.start_byte as usize)
            .collect();

        let original_file_name = self.extract_original_path()?;
        let original_match = self.extract_original_match()?;

        let original_src = self.extract_original_content()?;
        let rewritten_content = split_string_at_indices(original_src, ranges_starts).join(&comment);
        let ef = EntireFile::file_to_entire_file(original_file_name, &rewritten_content, None);
        Some(MatchResult::Rewrite(Rewrite::new(
            original_match,
            ef,
            self.extract_reason().cloned(),
        )))
    }
}

pub fn make_suppress_comment<'a>(
    pattern_name: Option<&str>,
    language: &impl MarzanoLanguage<'a>,
) -> String {
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
    fn content(&self) -> Result<&str>;
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
    pub uses_ai: bool,
}

impl PatternInfo {
    pub fn from_compiled(compiled: Problem, source_file: String) -> Self {
        let node = compiled.tree.root_node();
        let grit_node_types = grit_node_types();
        let parsed_pattern = to_string_pretty(&tree_sitter_node_to_json(
            &node.node,
            &source_file,
            &grit_node_types,
        ))
        .unwrap();

        let uses_ai = crate::analysis::uses_ai(&compiled.pattern, &compiled.definitions());

        Self {
            messages: vec![],
            variables: compiled.compiled_vars(),
            source_file,
            parsed_pattern,
            valid: true,
            uses_ai,
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
    // Do *NOT* use serde(flatten) in wasm-serializable items
    // Due to https://github.com/RReverser/serde-wasm-bindgen/issues/49, they will end up as maps.
    #[serde(default)]
    pub messages: Vec<Message>,
    #[serde(default)]
    pub variables: Vec<VariableMatch>,
    pub source_file: String,
    /// The full content of the file
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub ranges: Vec<Range>,
    #[serde(default)]
    pub reason: Option<MatchReason>,
    #[serde(default)]
    pub id: Uuid,
    /// Parsed content of the file, if a source map was used
    #[serde(skip)]
    inner_content: Option<String>,
}

impl From<EntireFile> for Match {
    fn from(file_match: EntireFile) -> Self {
        Self {
            messages: file_match.messages,
            variables: file_match.variables,
            source_file: file_match.source_file,
            ranges: file_match.ranges,
            reason: None,
            content: file_match.content,
            id: Uuid::new_v4(),
            inner_content: file_match.inner_content,
        }
    }
}

impl From<Match> for EntireFile {
    fn from(file_match: Match) -> Self {
        Self {
            messages: file_match.messages,
            variables: file_match.variables,
            source_file: file_match.source_file,
            ranges: file_match.ranges,
            // TODO: fix this or drop byte_ranges entirely
            byte_ranges: None,
            content: file_match.content,
            inner_content: file_match.inner_content,
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
    fn content(&self) -> Result<&str> {
        if let Some(inner_content) = self.inner_content.as_deref() {
            return Ok(inner_content);
        }

        let Some(content) = self.content.as_deref() else {
            bail!("No content in match")
        };

        if content.is_empty() {
            bail!("No content in match")
        } else {
            Ok(content)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct EntireFile {
    #[serde(default)]
    pub messages: Vec<Message>,
    #[serde(default)]
    pub variables: Vec<VariableMatch>,
    pub source_file: String,
    pub content: Option<String>,
    pub byte_ranges: Option<Vec<ByteRange>>,
    #[serde(default)]
    pub ranges: Vec<Range>,
    /// Inner (parsed) content of the file, if a source map was used
    #[serde(skip)]
    inner_content: Option<String>,
}

impl EntireFile {
    /// Create an entire file for cases where we don't really have an original file to reference
    /// When working with rewrites, `from_file` should be used instead
    fn file_to_entire_file(name: &str, body: &str, byte_range: Option<&Vec<ByteRange>>) -> Self {
        Self {
            source_file: name.to_owned(),
            content: Some(body.to_owned()),
            variables: vec![],
            messages: vec![],
            byte_ranges: byte_range.map(|r| r.to_owned()),
            ranges: vec![],
            inner_content: None,
        }
    }

    /// Create an entire file from a file owner, including handling source maps and byte ranges
    fn from_file(file: &FileOwner<Tree>) -> Result<Self> {
        let mut basic = if let Some(source_map) = &file.tree.source_map {
            let outer_source = source_map.fill_with_inner(&file.tree.source)?;

            let mut basic = Self::file_to_entire_file(
                file.name.to_string_lossy().as_ref(),
                &outer_source,
                // Exclude the matches, since they aren't reliable yet
                None,
            );
            basic.inner_content = Some(file.tree.source.to_owned());
            basic
        } else {
            Self::file_to_entire_file(
                file.name.to_string_lossy().as_ref(),
                file.tree.outer_source(),
                file.matches.borrow().byte_ranges.as_ref(),
            )
        };
        if let Some(input_ranges) = file.matches.borrow().input_matches.as_ref() {
            basic.ranges = input_ranges.ranges.clone();
            basic.variables = input_ranges.variables.clone();
        };
        Ok(basic)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Rewrite {
    pub original: EntireFile,
    pub rewritten: EntireFile,
    #[serde(default)]
    pub reason: Option<MatchReason>,
    #[serde(default)]
    pub id: Uuid,
}

impl From<Rewrite> for MatchResult {
    fn from(r: Rewrite) -> Self {
        MatchResult::Rewrite(r)
    }
}

impl Rewrite {
    fn file_to_rewrite(
        initial: &FileOwner<Tree>,
        rewritten_file: &FileOwner<Tree>,
    ) -> Result<Self> {
        let original = EntireFile::from_file(initial)?;
        let rewritten = EntireFile::from_file(rewritten_file)?;
        Ok(Rewrite::new(original, rewritten, None))
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
    fn content(&self) -> Result<&str> {
        self.rewritten.content.as_deref().ok_or_else(|| {
            anyhow::anyhow!(
                "No content in rewritten file {}",
                self.rewritten.source_file
            )
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct MatchReason {
    pub metadata_json: Option<String>,
    pub source: RewriteSource,
    /// The name of the pattern that matched, or another programmatic identifier
    pub name: Option<String>,
    /// A human-readable title for the match
    pub title: Option<String>,
    /// A human-readable explanation of the match
    pub explanation: Option<String>,
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
    pub fn new(original: EntireFile, rewritten: EntireFile, reason: Option<MatchReason>) -> Self {
        Self {
            original,
            rewritten,
            reason,
            id: Uuid::new_v4(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct CreateFile {
    pub rewritten: EntireFile,
    range: Option<Vec<Range>>,
    pub reason: Option<MatchReason>,
    #[serde(default)]
    pub id: Uuid,
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
            range: None,
            reason: None,
            id: Uuid::new_v4(),
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
                let end = Position::last(self.rewritten.content.as_deref().unwrap_or_default());
                self.range = Some(vec![Range::new(
                    start,
                    end,
                    0,
                    self.rewritten.content.as_deref().unwrap_or_default().len() as u32,
                )]);
                self.ranges()
            }
        }
    }
    fn action() -> &'static str {
        "created"
    }
    fn content(&self) -> Result<&str> {
        self.rewritten.content.as_deref().ok_or_else(|| {
            anyhow::anyhow!("No content in created file {}", self.rewritten.source_file)
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFile {
    pub original: EntireFile,
    #[serde(default)]
    pub reason: Option<MatchReason>,
    #[serde(default)]
    pub id: Uuid,
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
    fn content(&self) -> Result<&str> {
        self.original.content.as_deref().ok_or_else(|| {
            anyhow::anyhow!("No content in removed file {}", self.original.source_file)
        })
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
    #[serde(skip_serializing, skip_deserializing)]
    pub from_cache: bool,
}

impl DoneFile {
    pub fn new(relative_file_path: String) -> Self {
        Self {
            relative_file_path,
            has_results: None,
            file_hash: None,
            from_cache: false,
        }
    }
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
    /// How many files have been processed
    pub processed: i32,
    /// How many matches were found
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
    pub fn new_error(message: String, file: &str) -> Self {
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

    pub fn floating_error(message: String) -> Self {
        Self {
            level: 280,
            message,
            position: Position::first(),
            file: "".to_string(),
            engine_id: "marzano".to_string(),
            range: None,
            syntax_tree: None,
            source: None,
        }
    }
}

impl From<GritAnalysisLog> for AnalysisLog {
    fn from(log: GritAnalysisLog) -> Self {
        Self {
            level: log.level.unwrap_or(280),
            message: log.message,
            position: log.position.unwrap_or_else(Position::first),
            file: log
                .file
                .map(|file| file.to_string_lossy().to_string())
                .unwrap_or_default(),
            engine_id: log.engine_id.unwrap_or_else(|| "marzano".to_string()),
            range: log.range,
            syntax_tree: log.syntax_tree,
            source: log.source,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug, Copy, Clone)]
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
    fn test_normalize_path_in_project_with_root_no_slash() {
        let path = "/home/user/project/src/main.rs";
        let root_path = PathBuf::from("/home/user/project");
        assert_eq!(
            normalize_path_in_project(path, Some(&root_path)),
            "src/main.rs",
        );
    }

    #[test]
    fn test_normalize_path_in_project_already_normalized() {
        let path = "src/main.rs";
        assert_eq!(normalize_path_in_project(path, None), "src/main.rs");
    }
}
