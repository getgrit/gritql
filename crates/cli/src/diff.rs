use std::{fs::File, io::Read, path::PathBuf, str::FromStr};
pub fn extract_modified_ranges(diff_path: &PathBuf) -> Result<Vec<FileRange>> {
    let mut file = File::open(diff_path)?;
    let mut diff = String::new();

    file.read_to_string(&mut diff)?;
    parse_modified_ranges(&diff)
}

fn parse_modified_ranges(diff: &str) -> Result<Vec<FileRange>> {
                    start: start_pos,
                    end: end_pos,

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
}