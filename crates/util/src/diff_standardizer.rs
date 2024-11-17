/* ================================================================================

    getMulti.

================================================================================ */

use anyhow::Result;
use git2::{Delta, DiffOptions, Repository};

/// Given a before and after for a file, edit the *after* to not include any spurious changes
pub fn standardize_rewrite(repo: &Repository, before: String, after: String) -> Result<String> {
    let mut diff_opts = DiffOptions::new();
    diff_opts.ignore_whitespace(true);
    diff_opts.ignore_whitespace_change(true);

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

    // Run the diff to check if there are changes
    let mut right_oid = Option::<git2::Oid>::None;
    repo.diff_blobs(
        Some(&left_blob),
        None,
        Some(&right_blob),
        None,
        Some(&mut diff_opts),
        Some(&mut |delta, _progress| {
            right_oid = Some(delta.new_file().id());
            true
        }),
        None,
        Some(&mut |delta, _progress| {
            println!("delta: {:?}, progress: {:?}", delta, _progress);
            true
        }),
        Some(&mut |delta, _hunk, line| {
            println!("line: {:?}", line);
            true
        }),
    )
    .map_err(|e| anyhow::anyhow!("Failed to generate diff: {:?}", e))?;

    let Some(right_oid) = right_oid else {
        return Ok(before);
    };

    let right_blob = repo
        .find_blob(right_oid)
        .map_err(|e| anyhow::anyhow!("Failed to find right blob: {:?}", e))?;

    let content = std::str::from_utf8(right_blob.content())
        .map_err(|e| anyhow::anyhow!("Failed to convert blob content to UTF-8: {:?}", e))?;

    Ok(content.to_string())
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
        let after_standard = "function test() {\nconsole.log('test');\n}\n".to_string();
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
        let after = "line1\nmodified line2\n\tline3\nnew line4\n".to_string();
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

    #[test]
    fn test_code_removal() -> Result<()> {
        let (repo, _temp) = setup_test_repo()?;
        let before = r#"fn main() {
    // First we do some setup
    let mut total = 0;

    // Then we do a big calculation
    for i in 0..100 {
        if i % 2 == 0 {
            total += i;
        } else {
            total -= i;
        }
    }

    // Finally print the result
    println!("The total is: {}", total);
}"#
        .to_string();

        let after = r#"fn main() {
    let mut total = 0;
    println!("The total is: {}", total);
}"#
        .to_string();

        let result = standardize_rewrite(&repo, before, after.clone())?;
        assert_eq!(result, after);
        Ok(())
    }
}
