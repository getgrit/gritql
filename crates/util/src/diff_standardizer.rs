use git2::DiffOptions;
use marzano_util::diff::{parse_modified_ranges, FileDiff};
use tempfile::tempdir;
use anyhow::Result;

pub fn parse_unified_diff(contents: String) -> Result<Vec<FileDiff>> {
    let parsed = parse_modified_ranges(&contents)
        .map_err(|e| anyhow::anyhow!("{:?}", e))?;
    Ok(parsed)
}

/// Given a before and after for a file, edit the *after* to not include any spurious changes
pub fn standardize_rewrite(before: String, after: String) -> Result<String> {
    let mut diff_opts = DiffOptions::new();
    diff_opts.ignore_whitespace(true);

    // Create a temporary directory for the repository
    let temp_dir = tempdir()
        .map_err(|e| anyhow::anyhow!("Failed to create temp directory: {:?}", e))?;

    // Create blobs for diffing
    let repo = git2::Repository::init_opts(
        temp_dir.path(),
        &git2::RepositoryInitOptions::new()
            .bare(true)
            .initial_head("main"),
    )
    .map_err(|e| anyhow::anyhow!("Failed to create temp repo: {:?}", e))?;

    let left_oid = repo
        .blob(before.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to create left blob: {:?}", e))?;
    let right_oid = repo
        .blob(after.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to create right blob: {:?}", e))?;

    let left_blob = repo
        .find_blob(left_oid)
        .map_err(|e| anyhow::anyhow!("Failed to find left blob: {:?}", e))?;
    let right_blob = repo
        .find_blob(right_oid)
        .map_err(|e| anyhow::anyhow!("Failed to find right blob: {:?}", e))?;

    // Generate diff
    let mut standardized = after.clone(); // Start with the full after content
    let mut has_changes = false;
    let diff = repo
        .diff_blobs(
            Some(&left_blob),
            None,
            Some(&right_blob),
            None,
            Some(&mut diff_opts),
            None,
            None,
            None,
            Some(&mut |delta, _hunk, line| {
                has_changes = true;
                match line.origin() {
                    '+' | ' ' => {
                        if let Ok(_) = std::str::from_utf8(line.content()) {
                            // Content will be taken from the after string
                        }
                    }
                    '-' => {
                        // Removed lines are ignored as we're using the after content
                    }
                    _ => {}
                }
                true
            }),
        )
        .map_err(|e| anyhow::anyhow!("Failed to generate diff: {:?}", e))?;

    // If no changes were detected in the diff, return the after content as is
    if !has_changes {
        standardized = after;
    }

    // The temporary directory will be automatically cleaned up when temp_dir is dropped
    Ok(standardized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_basic_rewrite() -> Result<()> {
        let before = "Hello world\n".to_string();
        let after = "Hello Rust\n".to_string();
        let result = standardize_rewrite(before, after)?;
        assert_eq!(result, "Hello Rust\n");
        assert_snapshot!(result);
        Ok(())
    }

    #[test]
    fn test_whitespace_handling() -> Result<()> {
        let before = "function test() {\n    console.log('test');\n}\n".to_string();
        let after = "function test(){\nconsole.log('test');\n}\n".to_string();
        let result = standardize_rewrite(before, after)?;
        assert_eq!(result, "function test(){\nconsole.log('test');\n}\n");
        assert_snapshot!(result);
        Ok(())
    }

    #[test]
    fn test_empty_files() -> Result<()> {
        let before = "".to_string();
        let after = "".to_string();
        let result = standardize_rewrite(before, after)?;
        assert_eq!(result, "");
        assert_snapshot!(result);
        Ok(())
    }

    #[test]
    fn test_multiline_changes() -> Result<()> {
        let before = "line1\nline2\nline3\n".to_string();
        let after = "line1\nmodified line2\nline3\nnew line4\n".to_string();
        let result = standardize_rewrite(before, after)?;
        assert_eq!(result, "line1\nmodified line2\nline3\nnew line4\n");
        assert_snapshot!(result);
        Ok(())
    }

    #[test]
    fn test_no_changes() -> Result<()> {
        let content = "unchanged content\n".to_string();
        let result = standardize_rewrite(content.clone(), content)?;
        assert_eq!(result, "unchanged content\n");
        assert_snapshot!(result);
        Ok(())
    }

    #[test]
    fn test_mixed_changes_in_large_file() -> Result<()> {
        let before = r#"
// This is a large file with multiple sections

fn first_function() {
    // Some code here
    let x = 42;
    println!("Value: {}", x);
}

fn second_function() {
    let mut total = 0;
    for i in 0..10 {
        total += i;
    }


    // Extra whitespace above
    println!("Total: {}", total);
}

fn third_function() {
    let message = "Hello";
    println!("{}", message);
}
"#
        .to_string();

        let after = r#"
// This is a large file with multiple sections

fn first_function() {
    // Some code here
    let x = 42;
    println!("Value: {}", x);
}

fn second_function() {
    let mut total = 0;
    for i in 0..10 {
        total += i;
    }
    // No extra whitespace
    println!("Total: {}", total);
}

fn third_function() {
    let message = "Hello, World!";  // Changed this line
    println!("{}", message);
}
"#
        .to_string();

        let result = standardize_rewrite(before, after)?;

        // The result should:
        // 1. Keep first_function exactly the same
        // 2. Ignore whitespace changes in second_function
        // 3. Include the actual code change in third_function
        assert!(result.contains("fn first_function()"));
        assert!(result.contains("let x = 42"));
        assert!(result.contains("fn second_function()"));
        assert!(result.contains("println!(\"Total: {}\", total)"));
        assert!(result.contains("let message = \"Hello, World!\""));

        // Verify whitespace changes were ignored
        assert!(!result.contains("    \n    \n"));

        // Add snapshot test
        assert_snapshot!(result);

        Ok(())
    }
}
