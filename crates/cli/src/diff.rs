use anyhow::Result;
use marzano_util::position::{FileRange, Position, RangeWithoutByte, UtilRange};
use serde::Serialize;
use std::{fs::File, io::Read, path::PathBuf, str::FromStr};

use crate::commands::apply::SharedApplyArgs;

pub fn git_diff(path: &PathBuf) -> Result<String> {
    let output = std::process::Command::new("git")
        .arg("diff")
        .arg("HEAD")
        .arg("--relative")
        .arg("--unified=0")
        .arg(path)
        .output()?;
    Ok(String::from_utf8(output.stdout)?)
}

pub fn extract_modified_ranges(diff_path: &PathBuf) -> Result<Vec<FileDiff>> {
    let mut file = File::open(diff_path)?;
    let mut diff = String::new();

    file.read_to_string(&mut diff)?;
    parse_modified_ranges(&diff)
}

// Define a new struct to hold before and after ranges
#[derive(Debug, Clone, Serialize)]
pub struct FileDiff {
    pub file_path: String,
    pub before: Vec<FileRange>,
    pub after: Vec<FileRange>,
}

pub fn parse_modified_ranges(diff: &str) -> Result<Vec<FileDiff>> {
    let mut results = Vec::new();
    let lines = diff.lines();

    let mut is_deleted_file = false;
    let mut current_file_diff = FileDiff {
        file_path: String::new(),
        before: Vec::new(),
        after: Vec::new(),
    };
    let mut start_pos_before = Position { line: 0, column: 0 };
    let mut end_pos_before = Position { line: 0, column: 0 };
    let mut start_pos_after = Position { line: 0, column: 0 };
    let mut end_pos_after = Position { line: 0, column: 0 };

    for line in lines {
        if line.starts_with("+++") {
            if !current_file_diff.file_path.is_empty() {
                // If the previous file was marked as deleted, only "before" ranges are relevant.
                if is_deleted_file {
                    current_file_diff.after.clear();
                }
                results.push(current_file_diff);
            }
            is_deleted_file = line.contains("/dev/null");
            current_file_diff = FileDiff {
                file_path: line
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("")
                    .to_string()
                    .trim_start_matches("b/")
                    .to_string(),
                before: Vec::new(),
                after: Vec::new(),
            };
        } else if line.starts_with("@@") {
            let range_parts: Vec<&str> = line
                .split_whitespace()
                .nth(1)
                .unwrap_or("")
                .split('+')
                .collect();
            let after_range_parts: Vec<&str> = line
                .split_whitespace()
                .nth(2)
                .unwrap_or("")
                .split('+')
                .collect();

            if let Ok(line_num_before) = u32::from_str(range_parts[0].trim_start_matches('-')) {
                start_pos_before.line = line_num_before;
                end_pos_before.line = line_num_before
                    + range_parts
                        .get(1)
                        .map_or(1, |&x| x.parse::<u32>().unwrap_or(1));
            }

            if let Ok(line_num_after) = u32::from_str(after_range_parts[0]) {
                start_pos_after.line = line_num_after;
                end_pos_after.line = line_num_after
                    + after_range_parts
                        .get(1)
                        .map_or(1, |&x| x.parse::<u32>().unwrap_or(1));
            }

            current_file_diff.before.push(FileRange {
                file_path: current_file_diff.file_path.clone(),
                range: UtilRange::RangeWithoutByte(RangeWithoutByte {
                    start: start_pos_before,
                    end: end_pos_before,
                }),
            });

            current_file_diff.after.push(FileRange {
                file_path: current_file_diff.file_path.clone(),
                range: UtilRange::RangeWithoutByte(RangeWithoutByte {
                    start: start_pos_after,
                    end: end_pos_after,
                }),
            });
        }
    }

    if !current_file_diff.file_path.is_empty() {
        if is_deleted_file {
            current_file_diff.after.clear();
        }
        results.push(current_file_diff);
    }

    Ok(results)
}

pub(crate) fn extract_target_ranges(
    arg: &Option<Option<PathBuf>>,
) -> Result<Option<Vec<FileRange>>> {
    if let Some(Some(diff_path)) = &arg {
        let diff_ranges = extract_modified_ranges(diff_path)?;
        Ok(Some(
            diff_ranges.into_iter().flat_map(|x| x.after).collect(),
        ))
    } else if let Some(None) = &arg {
        let diff = git_diff(&std::env::current_dir()?)?;
        let diff_ranges = parse_modified_ranges(&diff)?;
        Ok(Some(
            diff_ranges.into_iter().flat_map(|x| x.after).collect(),
        ))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

    #[test]
    fn parse_one_file_diff() {
        let diff = r#"diff --git a/crates/cli_bin/fixtures/es6/empty_export_object.js b/crates/cli_bin/fixtures/es6/empty_export_object.js
index adacd90..71b96e0 100644
--- a/crates/cli_bin/fixtures/es6/empty_export_object.js
+++ b/crates/cli_bin/fixtures/es6/empty_export_object.js
@@ -5,7 +5,7 @@ module.exports = {
    };

    export async function createTeam() {
-  console.log('cool');
+  console.log('very cool');
    }

    export const addTeamToOrgSubscription = () => console.log('cool');
"#;
        let parsed = parse_modified_ranges(diff).unwrap();
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn parse_with_multiple_files() {
        let diff = r#"diff --git a/crates/cli_bin/fixtures/es6/empty_export_object.js b/crates/cli_bin/fixtures/es6/empty_export_object.js
index adacd90..71b96e0 100644
--- a/crates/cli_bin/fixtures/es6/empty_export_object.js
+++ b/crates/cli_bin/fixtures/es6/empty_export_object.js
@@ -5,7 +5,7 @@ module.exports = {
    };

    export async function createTeam() {
-  console.log('cool');
+  console.log('very cool');
    }

    export const addTeamToOrgSubscription = () => console.log('cool');
diff --git a/crates/cli_bin/fixtures/es6/export_object.js b/crates/cli_bin/fixtures/es6/export_object.js
index f6e1a2c..2c58ad2 100644
--- a/crates/cli_bin/fixtures/es6/export_object.js
+++ b/crates/cli_bin/fixtures/es6/export_object.js
@@ -2,7 +2,9 @@ async function createTeam() {
    console.log('cool');
    }

-const addTeamToOrgSubscription = () => console.log('cool');
+const addTeamToOrgSubscription = () => {
+  console.log('cool')
+};

    module.exports = {
    createTeam,
"#;
        let parsed = parse_modified_ranges(diff).unwrap();
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn parse_with_created_file() {
        let diff = r#"diff --git a/crates/cli_bin/fixtures/es6/empty_export_object.js b/crates/cli_bin/fixtures/es6/empty_export_object.js
index adacd90..71b96e0 100644
--- a/crates/cli_bin/fixtures/es6/empty_export_object.js
+++ b/crates/cli_bin/fixtures/es6/empty_export_object.js
@@ -5,7 +5,7 @@ module.exports = {
    };

    export async function createTeam() {
-  console.log('cool');
+  console.log('very cool');
    }

    export const addTeamToOrgSubscription = () => console.log('cool');
diff --git a/crates/cli_bin/fixtures/es6/export_object.js b/crates/cli_bin/fixtures/es6/export_object.js
index f6e1a2c..2c58ad2 100644
--- a/crates/cli_bin/fixtures/es6/export_object.js
+++ b/crates/cli_bin/fixtures/es6/export_object.js
@@ -2,7 +2,9 @@ async function createTeam() {
    console.log('cool');
    }

-const addTeamToOrgSubscription = () => console.log('cool');
+const addTeamToOrgSubscription = () => {
+  console.log('cool')
+};

    module.exports = {
    createTeam,
diff --git a/crates/cli_bin/fixtures/es6/index.js b/crates/cli_bin/fixtures/es6/index.js
new file mode 100644
index 0000000..7b232cd
--- /dev/null
+++ b/crates/cli_bin/fixtures/es6/index.js
@@ -0,0 +1,12 @@
+async function createTeam() {
+  console.log("cool");
+}
+
+const addTeamToOrgSubscription = () => {
+  console.log("cool");
+};
+
+module.exports = {
+  createTeam,
+  addTeamToOrgSubscription,
+};
"#;
        let parsed = parse_modified_ranges(diff).unwrap();
        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn parse_with_deleted_file() {
        let diff = r#"diff --git a/crates/cli_bin/fixtures/es6/empty_export_object.js b/crates/cli_bin/fixtures/es6/empty_export_object.js
index adacd90..71b96e0 100644
--- a/crates/cli_bin/fixtures/es6/empty_export_object.js
+++ b/crates/cli_bin/fixtures/es6/empty_export_object.js
@@ -5,7 +5,7 @@ module.exports = {
    };

    export async function createTeam() {
-  console.log('cool');
+  console.log('very cool');
    }

    export const addTeamToOrgSubscription = () => console.log('cool');
diff --git a/crates/cli_bin/fixtures/es6/export.js b/crates/cli_bin/fixtures/es6/export.js
deleted file mode 100644
index 52de8a9..0000000
--- a/crates/cli_bin/fixtures/es6/export.js
+++ /dev/null
@@ -1,19 +0,0 @@
-const king = '9';
-
-module.exports = {
-  king,
-  queen: '8',
-};
-
-async function createTeam() {
-  console.log('cool');
-}
-
-const addTeamToOrgSubscription = () => console.log('cool');
-
-module.exports = {
-  createTeam,
-  addTeamToOrgSubscription,
-};
-
-module.exports.queen = '9';
diff --git a/crates/cli_bin/fixtures/es6/export_object.js b/crates/cli_bin/fixtures/es6/export_object.js
index f6e1a2c..2c58ad2 100644
--- a/crates/cli_bin/fixtures/es6/export_object.js
+++ b/crates/cli_bin/fixtures/es6/export_object.js
@@ -2,7 +2,9 @@ async function createTeam() {
    console.log('cool');
    }

-const addTeamToOrgSubscription = () => console.log('cool');
+const addTeamToOrgSubscription = () => {
+  console.log('cool')
+};

    module.exports = {
    createTeam,
"#;
        let parsed = parse_modified_ranges(diff).unwrap();
        assert_yaml_snapshot!(parsed);
    }
}
