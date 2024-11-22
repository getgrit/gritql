/* ================================================================================

    getMulti.

================================================================================ */

use std::collections::VecDeque;

use anyhow::Result;
use similar::{ChangeTag, TextDiff};

/// Given a before and after for a file, edit the *after* to not include any spurious changes
pub fn standardize_rewrite(before: &str, after: String) -> Result<String> {
    let mut differ = TextDiff::configure();
    differ.algorithm(similar::Algorithm::Myers);
    let diff = differ.diff_lines(before, &after);
    let mut standardized_after = String::new();

    for op in diff.ops() {
        match op.tag() {
            similar::DiffTag::Equal | similar::DiffTag::Insert => {
                for line in diff.iter_changes(op) {
                    standardized_after.push_str(line.value());
                }
            }
            similar::DiffTag::Delete => {
                // Simply skip deleted lines
            }
            similar::DiffTag::Replace => {
                let mut before_cache = VecDeque::new();
                for line in diff.iter_changes(op) {
                    match line.tag() {
                        ChangeTag::Delete => {
                            before_cache.push_back(line.value());
                        }
                        ChangeTag::Insert => {
                            let value = line.value();
                            if let Some(before) = before_cache.pop_front() {
                                if before.trim() == value.trim() {
                                    // skip whitespace-only changes
                                    standardized_after.push_str(before);
                                } else {
                                    // Otherwise, include the line
                                    standardized_after.push_str(value);
                                }
                            } else {
                                standardized_after.push_str(value);
                            }
                        }
                        ChangeTag::Equal => {
                            standardized_after.push_str(line.value());
                            before_cache.clear();
                        }
                    }
                }
            }
        }
    }

    Ok(standardized_after)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_basic_rewrite() -> Result<()> {
        let before = "Hello world\n".to_string();
        let after = "Hello Rust\n".to_string();
        let result = standardize_rewrite(&before, after)?;
        assert_eq!(result, "Hello Rust\n");
        assert_snapshot!(result);
        Ok(())
    }

    #[test]
    fn test_whitespace_handling() -> Result<()> {
        let before = "function test() {\n    console.bob('test');\n}\n".to_string();
        let after = "function test() {\nconsole.log('test');\n}\n".to_string();
        let after_standard = "function test() {\nconsole.log('test');\n}\n".to_string();
        let result = standardize_rewrite(&before, after)?;
        assert_eq!(result, after_standard);
        Ok(())
    }

    #[test]
    fn test_empty_files() -> Result<()> {
        let before = "".to_string();
        let after = "".to_string();
        let result = standardize_rewrite(&before, after)?;
        assert_eq!(result, "");
        assert_snapshot!(result);
        Ok(())
    }

    #[test]
    fn test_multiline_changes() -> Result<()> {
        let before = "line1\nline2\n  line3\n".to_string();
        let after1 = "line1\nmodified line2\n  line3\nnew line4\n".to_string();
        let after2 = "line1\nmodified line2\n  line3\nnew line4\n".to_string();
        let result = standardize_rewrite(&before, after1)?;
        assert_eq!(result, after2);
        Ok(())
    }

    #[test]
    fn test_no_changes() -> Result<()> {
        let content = "unchanged content\n".to_string();
        let result = standardize_rewrite(&content.clone(), content)?;
        assert_eq!(result, "unchanged content\n");
        assert_snapshot!(result);
        Ok(())
    }

    #[test]
    fn test_mixed_changes_in_large_file() -> Result<()> {
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

        let result = standardize_rewrite(&before, after)?;
        assert_snapshot!(result);
        Ok(())
    }

    #[test]
    fn test_code_removal() -> Result<()> {
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
}
"#
        .to_string();

        let after = r#"fn main() {
    let mut total = 0;
    println!("The total is: {}", total);
}
"#
        .to_string();

        let result = standardize_rewrite(&before, after.clone())?;
        assert_eq!(result, after);
        Ok(())
    }

    #[test]
    fn test_remove_early_add_late() -> Result<()> {
        let before = r#"fn main() {
    let early = "remove me";
    let keep = "stay";
    let middle = "remove me too";
    let end = "keep me";
}"#
        .to_string();

        let after = r#"fn main() {
    let keep = "stay";
    let end = "keep me";
    let new = "add me";
}"#
        .to_string();

        let result = standardize_rewrite(&before, after.clone())?;
        assert_eq!(result, after);
        Ok(())
    }

    #[test]
    fn test_std_files() -> Result<()> {
        let before = include_str!("../fixtures/std.before.txt").to_string();
        let after = include_str!("../fixtures/std.after.txt").to_string();

        let result = standardize_rewrite(&before, after)?;
        assert_eq!(result, before.replace("OldStuff", "NewStuff"));
        Ok(())
    }
}
