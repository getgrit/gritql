use marzano_gritmodule::testing::GritTestResultState;
use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types::notification::Notification;

/// Notification sent when a test run is started or completed

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum RunGritTestState {
    Started,
    Completed,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A request to start or finish a test run
pub struct RunGritTestParams {
    pub test_run_id: String,
    pub state: RunGritTestState,
}

#[derive(Debug)]
pub enum RunGritTest {}

impl Notification for RunGritTest {
    type Params = RunGritTestParams;
    const METHOD: &'static str = "$/grit.runGritTest";
}

/// Notification with the results of a specific test

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum GritTestResult {
    Completed(GritTestResultState),
    Pending,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShowGritTestParams {
    pub test_id: String,
    pub test_run_id: Option<String>,
    pub parent_test_id: Option<String>,
    pub test_display_name: String,
    pub test_result: GritTestResult,
    pub test_message: Option<String>,
    pub expected_output: Option<String>,
    pub actual_output: Option<String>,
    pub file_uri: String,
    // zero-indexed position array: [start_line, start_char, end_line, end_char]
    pub file_range: Option<[u32; 4]>,
}

#[derive(Debug)]
pub enum ShowGritTest {}

impl Notification for ShowGritTest {
    type Params = ShowGritTestParams;
    const METHOD: &'static str = "$/grit.showGritTest";
}
