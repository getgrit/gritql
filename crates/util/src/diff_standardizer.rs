use anyhow::Result;
use git2::{DiffOptions, Repository};

/// Given a before and after for a file, edit the *after* to not include any spurious changes
pub fn standardize_rewrite(repo: &Repository, before: String, after: String) -> Result<String> {
    let mut diff_opts = DiffOptions::new();
    diff_opts.ignore_whitespace(true);

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
    let mut standardized = String::new();
    let mut added_lines = Vec::new();
    let mut current_line = 0;

    // There are changes, build up the new content from the diff
    let mut diff_opts = DiffOptions::new();
    repo.diff_blobs(
        Some(&left_blob),
        None,
        Some(&right_blob),
        None,
        Some(&mut diff_opts),
        None,
        None,
        None,
        Some(&mut |_delta, _hunk, line| {
            if let Ok(content) = std::str::from_utf8(line.content()) {
                match line.origin() {
                    ' ' => {
                        standardized.push_str(content);
                        current_line += 1;
                    }
                    '+' => {
                        standardized.push_str(content);
                        added_lines.push(current_line);
                        current_line += 1;
                    }
                    '-' => {
                        // Skip removed lines but don't increment current_line
                    }
                    _ => {}
                }
            }
            true
        }),
    )
    .map_err(|e| anyhow::anyhow!("Failed to generate diff: {:?}", e))?;

    // Add any remaining content from the right blob that wasn't encountered
    if let Ok(remaining_content) = std::str::from_utf8(right_blob.content()) {
        let remaining_lines: Vec<&str> = remaining_content.lines().collect();
        for (i, line) in remaining_lines.iter().enumerate() {
            if !added_lines.contains(&i) {
                standardized.push_str(line);
                standardized.push('\n');
            }
        }
    }

    Ok(standardized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    fn setup_test_repo() -> Result<(Repository, tempfile::TempDir)> {
        let temp_dir = tempfile::tempdir()?;
        let repo = Repository::init_opts(
            temp_dir.path(),
            git2::RepositoryInitOptions::new()
                .bare(true)
                .initial_head("main"),
        )?;
        Ok((repo, temp_dir))
    }

    #[test]
    fn test_basic_rewrite() -> Result<()> {
        let (repo, _temp) = setup_test_repo()?;
        let before = "Hello world\n".to_string();
        let after = "Hello Rust\n".to_string();
        let result = standardize_rewrite(&repo, before, after)?;
        assert_eq!(result, "Hello Rust\n");
        assert_snapshot!(result);
        Ok(())
    }

    #[test]
    fn test_whitespace_handling() -> Result<()> {
        let (repo, _temp) = setup_test_repo()?;
        let before = "function test() {\n    console.bob('test');\n}\n".to_string();
        let after = "function test() {\nconsole.log('test');\n}\n".to_string();
        let after_standard = "function test() {\n    console.log('test');\n}\n".to_string();
        let result = standardize_rewrite(&repo, before, after)?;
        assert_eq!(result, after_standard);
        Ok(())
    }

    #[test]
    fn test_empty_files() -> Result<()> {
        let (repo, _temp) = setup_test_repo()?;
        let before = "".to_string();
        let after = "".to_string();
        let result = standardize_rewrite(&repo, before, after)?;
        assert_eq!(result, "");
        assert_snapshot!(result);
        Ok(())
    }

    #[test]
    fn test_multiline_changes() -> Result<()> {
        let (repo, _temp) = setup_test_repo()?;
        let before = "line1\nline2\n  line3\n".to_string();
        let after = "line1\nmodified line2\n\tline3\nnew line4\n".to_string();
        let result = standardize_rewrite(&repo, before, after)?;
        let after = "line1\nmodified line2\n  line3\nnew line4\n".to_string();
        assert_eq!(result, after);
        Ok(())
    }

    #[test]
    fn test_no_changes() -> Result<()> {
        let (repo, _temp) = setup_test_repo()?;
        let content = "unchanged content\n".to_string();
        let result = standardize_rewrite(&repo, content.clone(), content)?;
        assert_eq!(result, "unchanged content\n");
        assert_snapshot!(result);
        Ok(())
    }

    #[test]
    fn test_mixed_changes_in_large_file() -> Result<()> {
        let (repo, _temp) = setup_test_repo()?;
        let before = r#"
/* ================================================================================

       getMulti.

================================================================================ */
fn second_function() {
 let mut total = 0;
    for i in 0..10 {
        total += i;
    }
  println!("Total: {}", total);
}

fn third_function() {
    let message = "Hello";
    println!("{}", message);
}
"#
        .to_string();

        let after = r#"
/* ================================================================================

    getMulti.

================================================================================ */
fn second_function() {
    let mut total = 0;
    for i in 0..10 {
        total += i;
    }
    println!("Total: {}", total);
}

fn third_function() {
    let thing = "Hello";
    debug!("{}", thing);
}
"#
        .to_string();

        let result = standardize_rewrite(&repo, before, after)?;

        assert_snapshot!(result);

        Ok(())
    }
}
