use crate::common::get_test_cmd;
use anyhow::Result;
use common::get_fixture;
use insta::assert_snapshot;

mod common;

#[test]
fn format_patterns_with_rewrite() -> Result<()> {
    let (_temp_dir, grit_dir) = get_fixture("unformatted_patterns", true)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("format")
        .arg("--write")
        .current_dir(grit_dir.clone());
    let output = cmd.output()?;

    println!("stderr: {}", String::from_utf8(output.stderr.clone())?);
    println!("stdout: {}", String::from_utf8(output.stdout.clone())?);

    assert!(!output.status.success());
    assert!(output.stderr.is_empty());

    let yaml_file_content = std::fs::read_to_string(grit_dir.join(".grit/grit.yaml"))?;
    assert_snapshot!(yaml_file_content);
    let test_move_import_file_content =
        std::fs::read_to_string(grit_dir.join(".grit/others/test_move_import.md"))?;
    assert_snapshot!(test_move_import_file_content);
    let aspect_ratio_md_file_content =
        std::fs::read_to_string(grit_dir.join(".grit/patterns/aspect_ratio.md"))?;
    assert_snapshot!(aspect_ratio_md_file_content);
    let dependency_grit_file_content =
        std::fs::read_to_string(grit_dir.join(".grit/patterns/dependency.grit"))?;
    assert_snapshot!(dependency_grit_file_content);
    let test_move_import_file_content =
        std::fs::read_to_string(grit_dir.join(".grit/others/test_move_import.md"))?;
    assert_snapshot!(test_move_import_file_content);
    let aspect_ratio_md_file_content =
        std::fs::read_to_string(grit_dir.join(".grit/patterns/aspect_ratio.md"))?;
    assert_snapshot!(aspect_ratio_md_file_content);
    Ok(())
}
