/* ================================================================================

    getMulti.

================================================================================ */

use anyhow::Result;
use git2::{Delta, DiffLineType, DiffOptions, Repository};

/// Given a before and after for a file, edit the *after* to not include any spurious changes
pub fn standardize_rewrite(repo: &Repository, before: String, after: String) -> Result<String> {
    let mut diff_opts = DiffOptions::new();
    diff_opts.ignore_whitespace(true);
    diff_opts.ignore_whitespace_change(true);
    diff_opts.context_lines(0);

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

    let before_lines = before.lines().collect::<Vec<_>>();
    let mut standardized_after = String::new();
    let mut before_line_num = 1_usize;
    let mut pending_additions: Vec<(usize, String)> = Vec::new();

    repo.diff_blobs(
        Some(&left_blob),
        None,
        Some(&right_blob),
        None,
        Some(&mut diff_opts),
        None,
        None,
        None,
        Some(&mut |_delta, hunk, line| {
            let hunk = hunk.unwrap();
            println!("\n=== Processing Hunk ===");
            println!(
                "Current before_line_num: {}, Hunk old_start: {}, old_lines: {}, new_lines: {}",
                before_line_num,
                hunk.old_start(),
                hunk.old_lines(),
                hunk.new_lines()
            );
            println!(
                "Hunk header: {}",
                std::str::from_utf8(hunk.header()).unwrap()
            );

            let hunk_start: usize = hunk.old_start().try_into().unwrap();

            // Copy any unchanged lines up to this hunk
            while before_line_num < hunk_start {
                let this_line = before_lines[before_line_num - 1];
                println!(
                    "Injecting unchanged line {}: '{}'",
                    before_line_num, this_line
                );
                standardized_after.push_str(this_line);
                standardized_after.push('\n');
                before_line_num += 1;
            }

            match line.origin_value() {
                DiffLineType::Deletion => {
                    println!(
                        "Deletion at line {}: '{}'",
                        before_line_num,
                        before_lines[before_line_num - 1]
                    );
                    before_line_num += 1;
                }
                DiffLineType::Addition => {
                    let new_content = std::str::from_utf8(line.content()).unwrap().to_string();
                    let target_position = hunk.new_start() as usize;
                    println!(
                        "Queueing addition at position {}: '{}'",
                        target_position, new_content
                    );
                    // Insert into pending_additions in sorted order by position
                    let insert_pos =
                        pending_additions.partition_point(|(pos, _)| *pos <= target_position);
                    pending_additions.insert(insert_pos, (target_position, new_content));
                }
                _ => {
                    println!(
                        "Other line type: {:?}, num_lines: {}",
                        line.origin_value(),
                        line.num_lines()
                    );
                }
            }

            // Process any pending additions that should come before the next hunk
            let mut current_line = before_line_num;
            while let Some((pos, content)) = pending_additions.first() {
                if *pos <= current_line {
                    standardized_after.push_str(&content);
                    pending_additions.remove(0);
                } else {
                    break;
                }
            }

            true
        }),
    )
    .map_err(|e| anyhow::anyhow!("Failed to generate diff: {:?}", e))?;

    // Add any remaining content and pending additions
    while before_line_num <= before_lines.len() || !pending_additions.is_empty() {
        let next_pending = pending_additions.first().map(|(pos, _)| *pos);
        let should_add_original = match next_pending {
            Some(pos) => before_line_num < pos,
            None => true,
        };

        if should_add_original && before_line_num <= before_lines.len() {
            standardized_after.push_str(before_lines[before_line_num - 1]);
            standardized_after.push('\n');
            before_line_num += 1;
        } else if let Some((_, content)) = pending_additions.first() {
            standardized_after.push_str(content);
            pending_additions.remove(0);
        }
    }

    Ok(standardized_after)
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
        let after_standard = "function test() {console.log('test');\n}\n".to_string();
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
        let after1 = "line1\nmodified line2\n  line3\nnew line4\n".to_string();
        let after2 = "line1\nmodified line2\n  line3\nnew line4\n".to_string();
        let result = standardize_rewrite(&repo, before, after1)?;
        assert_eq!(result, after2);
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
}
    "#
        .to_string();

        let result = standardize_rewrite(&repo, before, after.clone())?;
        assert_eq!(result, after);
        Ok(())
    }

    #[test]
    fn test_remove_early_add_late() -> Result<()> {
        let (repo, _td) = setup_test_repo()?;

        let before = r#"
// this starts us off

fn first_function() {
    println!("This will be removed");
    println!("This will also be removed");
}

fn middle_function() {
    // This stays the same
    println!("Hello");
}

fn last_function() {
    println!("Original");
}

// This is other content"#;

        let after = r#"
// this starts us off

fn middle_function() {
    // This stays the same
    println!("Hello");
}

fn last_function() {
    println!("Original");
    println!("New line added here");
    println!("And another one");
}

// This is other content"#;

        let result = standardize_rewrite(&repo, before.to_string(), after.to_string())?;
        assert_snapshot!(result);
        Ok(())
    }
}
