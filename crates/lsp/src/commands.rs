use std::{collections::HashMap, fmt};

use marzano_core::pattern::api::MatchResult;
use marzano_gritmodule::config::ResolvedGritDefinition;
use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types::{notification::Notification, request::Request};

#[derive(Debug, PartialEq)]
pub enum LspCommand {
    ApplyPattern,
    ApplyResult,
    ApplyNamedPattern,
    ShowDebug,
    OpenPatternSelector,
    FixFile,
    SearchGritQL,
    PingGrit,
}

impl LspCommand {
    pub fn maybe_from_str(s: &str) -> Option<Self> {
        match s {
            "grit.applyPattern" => Some(Self::ApplyPattern),
            "grit.applyResult" => Some(Self::ApplyResult),
            "grit.applyNamedPattern" => Some(Self::ApplyNamedPattern),
            "grit.showDebugInfo" => Some(Self::ShowDebug),
            "grit.openPatternSelectorForFile" => Some(Self::OpenPatternSelector),
            "grit.fixSelectedFile" => Some(Self::FixFile),
            "grit.searchGritQL" => Some(Self::SearchGritQL),
            "grit.ping" => Some(Self::PingGrit),
            _ => None,
        }
    }
}

impl fmt::Display for LspCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ApplyPattern => write!(f, "grit.applyPattern"),
            Self::ApplyResult => write!(f, "grit.applyResult"),
            Self::ApplyNamedPattern => write!(f, "grit.applyNamedPattern"),
            Self::ShowDebug => write!(f, "grit.showDebugInfo"),
            Self::OpenPatternSelector => write!(f, "grit.openPatternSelectorForFile"),
            Self::FixFile => write!(f, "grit.fixSelectedFile"),
            Self::SearchGritQL => write!(f, "grit.searchGritQL"),
            Self::PingGrit => write!(f, "grit.ping"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShowPatternSelectorParams {
    pub file: String,
    pub patterns: Vec<ResolvedGritDefinition>,
}

#[derive(Debug)]
pub enum ShowPatternSelector {}

impl Request for ShowPatternSelector {
    type Params = ShowPatternSelectorParams;
    type Result = ();
    const METHOD: &'static str = "grit.showPatternSelector";
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum GritHighlightKind {
    Search,
    PatternTest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowGritHighlightsRequest {
    pub kind: GritHighlightKind,
    /// The results of the search, keyed by document URI
    pub results: HashMap<String, Vec<MatchResult>>,
}

pub enum ShowGritHighlights {}

impl Request for ShowGritHighlights {
    type Params = ShowGritHighlightsRequest;
    type Result = ();
    const METHOD: &'static str = "grit.highlightSearchResults";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GritPongNotificationParams {}

pub enum GritPongNotification {}

impl Notification for GritPongNotification {
    type Params = GritPongNotificationParams;

    const METHOD: &'static str = "$/grit.pong";
}
