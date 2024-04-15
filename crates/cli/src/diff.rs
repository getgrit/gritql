use anyhow::Result;
use marzano_util::position::{FileRange, Position, RangeWithoutByte, UtilRange};
use regex::Regex;
use serde::Serialize;
use std::{fs::File, io::Read, path::PathBuf, str::FromStr};

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
    pub old_path: Option<String>,
    pub new_path: String,
    pub before: Vec<UtilRange>,
    pub after: Vec<UtilRange>,
}

fn parse_hunk_part(range_part: &str) -> Result<UtilRange> {
    let range_parts: Vec<&str> = range_part.split(',').collect();
    if let Ok(line_num) = u32::from_str(range_parts[0].trim_start_matches(['+', '-'])) {
        return Ok(UtilRange::RangeWithoutByte(RangeWithoutByte {
            start: Position {
                line: line_num,
                column: 0,
            },
            end: Position {
                line: line_num
                    + range_parts
                        .get(1)
                        .map_or(1, |&x| x.parse::<u32>().unwrap_or(0)),
                column: 0,
            },
        }));
    }
    Err(anyhow::anyhow!("Failed to parse hunk part"))
}

pub fn parse_modified_ranges(diff: &str) -> Result<Vec<FileDiff>> {
    let mut results = Vec::new();
    let lines = diff.lines();

    let mut current_file_diff = None;

    for line in lines {
        if line.starts_with("---") {
            let old_file_name = line
                .split_whitespace()
                .nth(1)
                .unwrap_or("")
                .to_string()
                .trim_start_matches("a/")
                .to_string();

            current_file_diff = Some(FileDiff {
                old_path: Some(old_file_name.clone()),
                new_path: Some(old_file_name),
                before: Vec::new(),
                after: Vec::new(),
            });
        } else if line.starts_with("+++") {
            let new_file_name = line
                .split_whitespace()
                .nth(1)
                .unwrap_or("")
                .to_string()
                .trim_start_matches("b/")
                .to_string();

            if let Some(file_diff) = &mut current_file_diff {
                file_diff.new_path = new_file_name;
            } else {
                current_file_diff = Some(FileDiff {
                    old_path: None,
                    new_path: new_file_name,
                    before: Vec::new(),
                    after: Vec::new(),
                });
            }
        } else if line.starts_with("@@") {
            let parts = line.split_whitespace();
            let before_range = parse_hunk_part(parts.nth(1).unwrap_or_default())?;
            let after_range = parse_hunk_part(parts.nth(2).unwrap_or_default())?;

            println!("Before: {:?}, After: {:?}", before_range, after_range);

            if let Some(file_diff) = &mut current_file_diff {
                file_diff.before.push(before_range);
                file_diff.after.push(after_range);
            } else {
                println!("No current file diff");
            }
        }
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
        let before_range = &parsed[0].before[0];
        assert_eq!(
            before_range.file_path,
            "crates/cli_bin/fixtures/es6/empty_export_object.js"
        );
        assert_eq!(before_range.range.start_line(), 5);
        assert_eq!(before_range.range.end_line(), 5);
        let after_range = &parsed[0].after[0];
        assert_eq!(
            after_range.file_path,
            "crates/cli_bin/fixtures/es6/empty_export_object.js"
        );
        assert_eq!(after_range.range.start_line(), 5);
        assert_eq!(after_range.range.end_line(), 5);
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
