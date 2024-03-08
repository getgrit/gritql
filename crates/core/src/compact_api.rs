use serde::Serialize;

use crate::pattern::api::{
    AllDone, AnalysisLog, CreateFile, DoneFile, EntireFile, InputFile, Match, MatchResult,
    PatternInfo, RemoveFile, Rewrite, RewriteReason,
};

/// Compact API representations for all API types.
/// These are meant to only contain the absolute minimum information needed.
/// Serialization is surprisingly expensive, so they are very helpful for scan performance.
/// Make sure to keep src/matching/compact.ts in sync with this file.

pub fn compact(item: MatchResult) -> CompactResult {
    match item {
        MatchResult::PatternInfo(i) => CompactResult::PatternInfo(i),
        MatchResult::AllDone(i) => CompactResult::AllDone(i),
        MatchResult::Match(i) => CompactResult::Match(i.into()),
        MatchResult::InputFile(i) => CompactResult::InputFile(i),
        MatchResult::Rewrite(i) => CompactResult::Rewrite(i.into()),
        MatchResult::CreateFile(i) => CompactResult::CreateFile(i.into()),
        MatchResult::RemoveFile(i) => CompactResult::RemoveFile(i.into()),
        MatchResult::DoneFile(i) => CompactResult::DoneFile(i),
        MatchResult::AnalysisLog(i) => CompactResult::AnalysisLog(i),
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "__typename")]
pub enum CompactResult {
    #[serde(rename = "PatternInfo")]
    PatternInfo(PatternInfo),
    #[serde(rename = "AllDone")]
    AllDone(AllDone),
    #[serde(rename = "Match")]
    Match(CompactMatch),
    #[serde(rename = "InputFile")]
    InputFile(InputFile),
    #[serde(rename = "Rewrite")]
    Rewrite(CompactRewrite),
    #[serde(rename = "CreateFile")]
    CreateFile(CompactCreateFile),
    #[serde(rename = "RemoveFile")]
    RemoveFile(CompactRemoveFile),
    #[serde(rename = "DoneFile")]
    DoneFile(DoneFile),
    #[serde(rename = "AnalysisLog")]
    AnalysisLog(AnalysisLog),
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct CompactMatch {
    pub source_file: String,
}

impl From<Match> for CompactMatch {
    fn from(m: Match) -> Self {
        CompactMatch {
            source_file: m.source_file,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct CompactFile {
    pub source_file: String,
}

impl From<EntireFile> for CompactFile {
    fn from(m: EntireFile) -> Self {
        CompactFile {
            source_file: m.source_file,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct CompactRewrite {
    pub original: CompactMatch,
    pub rewritten: CompactFile,
    pub reason: Option<RewriteReason>,
}

impl From<Rewrite> for CompactRewrite {
    fn from(m: Rewrite) -> Self {
        CompactRewrite {
            original: m.original.into(),
            rewritten: m.rewritten.into(),
            reason: m.reason,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct CompactCreateFile {
    pub rewritten: CompactFile,
}

impl From<CreateFile> for CompactCreateFile {
    fn from(m: CreateFile) -> Self {
        CompactCreateFile {
            rewritten: m.rewritten.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct CompactRemoveFile {
    pub original: CompactMatch,
}

impl From<RemoveFile> for CompactRemoveFile {
    fn from(m: RemoveFile) -> Self {
        CompactRemoveFile {
            original: m.original.into(),
        }
    }
}
