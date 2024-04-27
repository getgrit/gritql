use anyhow::{bail, Result};
use grit_util::{Position, RangeWithoutByte};
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RangePair {
    pub before: RangeWithoutByte,
    pub after: RangeWithoutByte,
}

// Define a new struct to hold before and after ranges
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct FileDiff {
    pub old_path: Option<String>,
    pub new_path: Option<String>,
    pub ranges: Vec<RangePair>,
}

/// Extract the line numbers from a hunk part
/// Note this does *NOT* necessarily correspond to the actual line numbers in the file, since context can be included in the hunks
/// But we are choosing to treat this as good enough for now
fn parse_hunk_part(range_part: &str) -> Result<RangeWithoutByte> {
    let range_parts: Vec<&str> = range_part.split(',').collect();
    if let Ok(line_num) = u32::from_str(range_parts[0].trim_start_matches(['+', '-'])) {
        return Ok(RangeWithoutByte {
            start: Position {
                line: line_num,
                column: 1,
            },
            end: Position {
                line: line_num
                    + range_parts
                        .get(1)
                        .map_or(0, |&x| x.parse::<u32>().unwrap_or(0)),
                column: 1,
            },
        });
    }
    Err(anyhow::anyhow!("Failed to parse hunk part"))
}

fn parse_hunk(hunk_str: &str) -> Result<RangePair> {
    let mut parts = hunk_str.split_whitespace();
    let before_range = parse_hunk_part(parts.nth(1).unwrap_or(""))?;
    // Note nth mutates the iterator, so after range is the next element
    let after_range = parse_hunk_part(parts.next().unwrap_or(""))?;

    Ok(RangePair {
        before: before_range,
        after: after_range,
    })
}

/// parse_modified_ranges is the core function for parsing the unified diff format
/// This is the format that `git diff` outputs, and is the most common diff format
///
/// This will output a list of FileDiff objects, which contain the old and new paths, and the ranges of changes
/// Each range is a combined pair of before/after ranges
///
/// A deletion will have a range with an empty after range, and an addition will have a range with an empty before range
/// Note that ranges (currently) cover the whole line, not the specific columns that were changed
///
/// The output *ignores* all context lines, so the same underlying change will be represented the same way regardless
/// of how many context lines are included
pub fn parse_modified_ranges(diff: &str) -> Result<Vec<FileDiff>> {
    let mut results = Vec::new();
    let lines = diff.lines();

    // We keep a cursor for the current left and right lines (this gets reset when encountering hunks)
    let mut left_line_cursor = 0;
    let mut right_line_cursor = 0;

    // When there is no context at all, we need to shift deletions by one
    let mut has_context = false;

    // Then we might be adding to a left range, or to a right range.
    let mut current_range_pair: Option<RangePair> = None;

    for line in lines {
        if line.starts_with("--- ") {
            let old_file_name = line
                .split_whitespace()
                .nth(1)
                .unwrap_or("")
                .to_string()
                .trim_start_matches("a/")
                .to_string();

            results.push(FileDiff {
                old_path: if old_file_name == "/dev/null" {
                    None
                } else {
                    Some(old_file_name)
                },
                new_path: None,
                ranges: Vec::new(),
            });
        } else if line.starts_with("+++ ") {
            let new_file_name = line
                .split_whitespace()
                .nth(1)
                .unwrap_or("")
                .to_string()
                .trim_start_matches("b/")
                .to_string();

            if let Some(file_diff) = results.last_mut() {
                file_diff.new_path = if new_file_name == "/dev/null" {
                    None
                } else {
                    Some(new_file_name)
                };
            } else {
                bail!("Encountered new file path without a current file diff");
            };
        } else if line.starts_with("@@ ") {
            insert_range_if_found(&mut current_range_pair, &mut results, has_context)?;
            has_context = false;

            let parsed_hunk = parse_hunk(line)?;

            left_line_cursor = parsed_hunk.before.start_line();
            right_line_cursor = parsed_hunk.after.start_line();
        } else if line.starts_with(' ') || line.is_empty() {
            insert_range_if_found(&mut current_range_pair, &mut results, has_context)?;
            has_context = true;

            left_line_cursor += 1;
            right_line_cursor += 1;
        } else if line.starts_with('-') || line.starts_with('+') {
            if line.starts_with('-') {
                // Removed sections always come before added sections
                if let Some(ref mut pair) = current_range_pair {
                    // We already have a pair, so we are just expanding the remove section
                    pair.before.end.line += 1;
                } else {
                    // Start a new hunk, which will initially be a removal
                    current_range_pair = Some(RangePair {
                        before: RangeWithoutByte {
                            start: Position {
                                line: left_line_cursor,
                                column: 1,
                            },
                            end: Position {
                                line: left_line_cursor + 1,
                                column: 1,
                            },
                        },
                        after: RangeWithoutByte {
                            start: Position {
                                line: right_line_cursor,
                                column: 1,
                            },
                            end: Position {
                                line: right_line_cursor,
                                column: 1,
                            },
                        },
                    });
                }

                // This *always* advances the left cursor by one line
                left_line_cursor += 1;
            } else {
                if let Some(ref mut pair) = current_range_pair {
                    // We already have a pair, so we are just expanding the add section
                    pair.after.end.line = right_line_cursor + 1;
                } else {
                    // We are adding, without any removal
                    current_range_pair = Some(RangePair {
                        before: RangeWithoutByte {
                            start: Position {
                                line: left_line_cursor,
                                column: 1,
                            },
                            end: Position {
                                line: left_line_cursor,
                                column: 1,
                            },
                        },
                        after: RangeWithoutByte {
                            start: Position {
                                line: right_line_cursor,
                                column: 1,
                            },
                            end: Position {
                                line: right_line_cursor + 1,
                                column: 1,
                            },
                        },
                    });
                }

                right_line_cursor += 1;
            }
        } else {
            // Ignore other lines
        }
    }

    // If we have a current hunk, add it to the current file diff
    insert_range_if_found(&mut current_range_pair, &mut results, has_context)?;

    Ok(results)
}

fn insert_range_if_found(
    current_range_pair: &mut Option<RangePair>,
    results: &mut [FileDiff],
    has_context: bool,
) -> Result<()> {
    if let Some(mut range) = current_range_pair.take() {
        // Deleted lines with no context are shifted by one
        if range.after.is_empty() && !has_context {
            range.after.start.line += 1;
            range.after.end.line += 1;
        }
        if let Some(file_diff) = results.last_mut() {
            file_diff.ranges.push(range);
        } else {
            bail!("Encountered hunk without a current file diff");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use grit_util::Range;
    use insta::assert_yaml_snapshot;

    #[test]
    fn parse_one_line_hunk() {
        let hunk = "@@ -12 +12 @@ use tracing_opentelemetry::OpenTelemetrySpanExt as _;";
        let parsed = parse_hunk(hunk).unwrap();
        assert_eq!(parsed.before.start_line(), 12);
        assert_eq!(parsed.before.end_line(), 12);
        assert_eq!(parsed.after.start_line(), 12);
        assert_eq!(parsed.after.end_line(), 12);
    }

    #[test]
    fn parses_verified_baseline() {
        let diff = r#"diff --git a/crates/cli/src/analyze.rs b/crates/cli/src/analyze.rs
index 893656e..6218f5e 100644
--- a/crates/cli/src/analyze.rs
+++ b/crates/cli/src/analyze.rs
@@ -9,7 +9,7 @@ use tracing::{event, instrument, Level};
    #[cfg(feature = "grit_tracing")]
    use tracing_opentelemetry::OpenTelemetrySpanExt as _;

-use grit_cache::paths::cache_for_cwd;
+use THIS WAS CHANGED;
    use ignore::Walk;
    use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};"#;

        let parsed = parse_modified_ranges(diff).unwrap();
        println!("{:?}", parsed);
        assert_eq!(parsed[0].ranges.len(), 1);
        let first_range = &parsed[0].ranges[0];
        assert_eq!(first_range.before.end_line(), 13);
        assert_eq!(first_range.after.start_line(), 12);
        assert_eq!(first_range.after.end_line(), 13);

        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn parse_one_file_diff() {
        let diff = r#"diff --git a/crates/cli_bin/fixtures/es6/empty_export_object.js b/crates/cli_bin/fixtures/es6/empty_export_object.js
index adacd90..71b96e0 100644
--- a/crates/cli_bin/fixtures/es6/empty_export_object.js
+++ b/crates/cli_bin/fixtures/es6/empty_export_object.js
@@ -5,7 +5,7 @@ module.exports = {
    };

   export async function createTeam() {
-    console.log('cool');
+    console.log('very cool');
   }

   export const addTeamToOrgSubscription = () => console.log('cool');
"#;
        let parsed = parse_modified_ranges(diff).unwrap();
        assert_eq!(parsed[0].ranges[0].before.start_line(), 8);
        assert_eq!(parsed[0].ranges[0].before.end_line(), 9);
        assert_eq!(parsed[0].ranges[0].after.start_line(), 8);
        assert_eq!(parsed[0].ranges[0].after.end_line(), 9);

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

        assert_eq!(parsed[0].ranges[0].before.start_line(), 8);
        assert_eq!(parsed[0].ranges[0].before.end_line(), 9);
        assert_eq!(parsed[0].ranges[0].after.start_line(), 8);
        assert_eq!(parsed[0].ranges[0].after.end_line(), 9);

        assert_eq!(parsed[1].ranges[0].before.start_line(), 5);
        assert_eq!(parsed[1].ranges[0].before.end_line(), 6);
        assert_eq!(parsed[1].ranges[0].after.start_line(), 5);
        assert_eq!(parsed[1].ranges[0].after.end_line(), 8);
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

        assert_eq!(
            parsed[1].old_path,
            Some("crates/cli_bin/fixtures/es6/export_object.js".to_string())
        );
        assert_eq!(
            parsed[1].new_path,
            Some("crates/cli_bin/fixtures/es6/export_object.js".to_string())
        );

        // Finally look at the new one
        let new_file = &parsed[2];
        assert_eq!(new_file.old_path, None);
        assert_eq!(
            new_file.new_path,
            Some("crates/cli_bin/fixtures/es6/index.js".to_string())
        );

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
        assert_eq!(
            parsed[1].old_path,
            Some("crates/cli_bin/fixtures/es6/export.js".to_string())
        );
        assert!(parsed[1].new_path.is_none());

        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn handles_weird_diffs() {
        // This diff includes diffs inside the diff itself
        let diff = include_str!("../fixtures/long_diff.diff");
        let parsed_diffs = parse_modified_ranges(diff).expect("Failed to parse diffs");
        assert!(!parsed_diffs.is_empty(), "No diffs parsed");
        assert!(parsed_diffs.iter().all(|diff| diff.new_path.is_some()));
        assert_eq!(parsed_diffs.len(), 21);
    }

    fn validate_known_diff(diff: Vec<FileDiff>) {
        // Sanity check
        assert_eq!(diff.len(), 1);

        // @@ -12 +12 @@ use tracing_opentelemetry::OpenTelemetrySpanExt as _;
        assert_eq!(
            diff[0].ranges[0],
            RangePair {
                before: RangeWithoutByte {
                    start: Position {
                        line: 12,
                        column: 1,
                    },
                    end: Position {
                        line: 13,
                        column: 1,
                    },
                },
                after: RangeWithoutByte {
                    start: Position {
                        line: 12,
                        column: 1,
                    },
                    end: Position {
                        line: 13,
                        column: 1,
                    },
                },
            },
            "Failed to parse range 1"
        );

        // @@ -14 +14 @@ use ignore::Walk;
        assert_eq!(
            diff[0].ranges[1],
            RangePair {
                before: RangeWithoutByte {
                    start: Position {
                        line: 14,
                        column: 1,
                    },
                    end: Position {
                        line: 15,
                        column: 1,
                    },
                },
                after: RangeWithoutByte {
                    start: Position {
                        line: 14,
                        column: 1,
                    },
                    end: Position {
                        line: 15,
                        column: 1,
                    },
                },
            },
            "Failed to parse range 2"
        );

        // @@ -17 +17 @@ use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};
        assert_eq!(
            diff[0].ranges[2],
            RangePair {
                before: RangeWithoutByte {
                    start: Position {
                        line: 17,
                        column: 1,
                    },
                    end: Position {
                        line: 18,
                        column: 1,
                    },
                },
                after: RangeWithoutByte {
                    start: Position {
                        line: 17,
                        column: 1,
                    },
                    end: Position {
                        line: 18,
                        column: 1,
                    },
                },
            },
            "Failed to parse range 3"
        );

        // @@ -19,4 +18,0 @@ use marzano_core::pattern_compiler::{src_to_problem_libs, CompilationResult};
        assert_eq!(
            diff[0].ranges[3],
            RangePair {
                before: RangeWithoutByte {
                    start: Position {
                        line: 19,
                        column: 1,
                    },
                    end: Position {
                        line: 23,
                        column: 1,
                    },
                },
                after: RangeWithoutByte {
                    start: Position {
                        line: 19,
                        column: 1,
                    },
                    end: Position {
                        line: 19,
                        column: 1,
                    },
                },
            },
            "Failed to parse range 4"
        );
    }

    #[test]
    fn ignores_context() {
        let no_context = include_str!("../fixtures/no_context.diff");
        let no_context_parsed =
            parse_modified_ranges(no_context).expect("Failed to parse no context diff");

        println!("Validating context-less diff...");
        validate_known_diff(no_context_parsed.clone());

        // These two diffs are *identical* except for the context line length
        println!("Parsing normal diff...");
        let normal_diff = include_str!("../fixtures/normal_diff.diff");
        let normal_diff_parsed =
            parse_modified_ranges(normal_diff).expect("Failed to parse normal diff");

        println!("Validating normal diff...");
        validate_known_diff(normal_diff_parsed.clone());

        // Ensure they are the same
        assert_eq!(normal_diff_parsed, no_context_parsed);
    }

    #[test]
    fn finds_the_right_range() {
        let diff = include_str!("../fixtures/file_diff.diff");
        let parsed = parse_modified_ranges(diff).expect("Failed to parse no context diff");

        let old_range = &parsed[0].ranges[0].before;
        assert_eq!(old_range.start.line, 5);
        assert_eq!(old_range.end.line, 6);

        let new_range = &parsed[0].ranges[0].after;
        assert_eq!(new_range.start.line, 5);
        assert_eq!(new_range.end.line, 6);

        // Make them into byte ranges and index into content
        let old_content = include_str!("../fixtures/file.baseline.js");
        let new_content = include_str!("../fixtures/file.changed.js");

        let old_range = Range::from_byteless(old_range.clone(), old_content);
        assert_eq!(
            old_content[old_range.range_index()].to_string(),
            "  console.log(\"World\");\n"
        );
        let new_range = Range::from_byteless(new_range.clone(), new_content);
        assert_eq!(
            new_content[new_range.range_index()].to_string(),
            "  console.log(\"change this\");\n"
        );
    }

    #[test]
    fn processes_multiline_edit() {
        let diff = include_str!("../fixtures/multiline_edit.diff");
        let parsed = parse_modified_ranges(diff).expect("Failed to parse no context diff");
        let old_range = &parsed[0].ranges[0].before;
        let new_range = &parsed[0].ranges[0].after;

        // Make them into byte ranges and index into content
        let old_content = include_str!("../fixtures/file.baseline.js");
        let new_content = include_str!("../fixtures/file.multiline.js");

        let old_range = Range::from_byteless(old_range.clone(), old_content);
        assert_eq!(
            old_content[old_range.range_index()].to_string(),
            "function foo() {\n  console.log(\"World\");\n"
        );
        let new_range = Range::from_byteless(new_range.clone(), new_content);
        assert_eq!(
            new_content[new_range.range_index()].to_string(),
            "function nice() {\n  console.log(\"MULTI\\nLINE\\nSTRING\");\n  throw new Error(\"Oh no!\");\n"
        );

        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn processes_newline_rm() {
        let diff = include_str!("../fixtures/newline_rm.diff");
        let parsed = parse_modified_ranges(diff).expect("Failed to parse no context diff");
        let old_range = &parsed[0].ranges[0].before;

        // Make them into byte ranges and index into content
        let old_content = include_str!("../fixtures/file.baseline.js");

        let old_range = Range::from_byteless(old_range.clone(), old_content);
        assert_eq!(old_content[old_range.range_index()].to_string(), "\n");

        assert_yaml_snapshot!(parsed);
    }

    #[test]
    fn processes_newline_add() {
        let diff = include_str!("../fixtures/newline_add.diff");
        let parsed = parse_modified_ranges(diff).expect("Failed to parse no context diff");
        let old_range = &parsed[0].ranges[0].before;

        let old_content = include_str!("../fixtures/file.baseline.js");
        let old_range = Range::from_byteless(old_range.clone(), old_content);
        assert_eq!(old_content[old_range.range_index()].to_string(), "");

        let new_content = include_str!("../fixtures/file.newline.js");
        let new_range = Range::from_byteless(parsed[0].ranges[0].after.clone(), new_content);
        assert_eq!(new_content[new_range.range_index()].to_string(), "\n");

        assert_yaml_snapshot!(parsed);
    }
}
