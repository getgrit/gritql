use anyhow::Result;
use marzano_util::position::{FileRange, Position, RangeWithoutByte, UtilRange};
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

pub fn extract_modified_ranges(diff_path: &PathBuf) -> Result<Vec<FileRange>> {
    let mut file = File::open(diff_path)?;
    let mut diff = String::new();

    file.read_to_string(&mut diff)?;
    parse_modified_ranges(&diff)
}

pub fn parse_modified_ranges(diff: &str) -> Result<Vec<FileRange>> {
    let mut results = Vec::new();
    let lines = diff.lines();

    let mut current_file = String::new();
    let mut start_pos = Position { line: 0, column: 0 };
    let mut end_pos = Position { line: 0, column: 0 };

    for line in lines {
        if line.starts_with("+++") {
            current_file = line.split_whitespace().nth(1).unwrap_or("").to_string();
            if current_file.starts_with("b/") {
                current_file = current_file[2..].to_string();
            }
        } else if line.starts_with("@@") {
            if current_file == "/dev/null" {
                continue;
            }
            let range_part = line.split_whitespace().nth(2).unwrap_or("");
            let range_parts: Vec<&str> = range_part.split(',').collect();
            if let Ok(line_num) = u32::from_str(range_parts[0].trim_start_matches('+')) {
                start_pos.line = line_num;
                end_pos.line = line_num
                    + range_parts
                        .get(1)
                        .map_or(1, |&x| x.parse::<u32>().unwrap_or(0));
            }

            results.push(FileRange {
                file_path: current_file.clone(),
                range: UtilRange::RangeWithoutByte(RangeWithoutByte {
                    start: start_pos,
                    end: end_pos,
                }),
            });
        }
    }

    Ok(results)
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
