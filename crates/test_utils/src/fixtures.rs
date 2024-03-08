use anyhow::Result;
use tempfile::tempdir;

/// Fetch a fixtures director, relative to the current binary under test
pub fn get_fixtures_root(manifest_dir: &str) -> Result<std::path::PathBuf> {
    let mut fixtures_root = std::path::PathBuf::from(manifest_dir);
    fixtures_root.push("fixtures");
    Ok(fixtures_root)
}

/// This function is used in tests to get a copy of a particular fixture in a tempdir.
///
/// Note: tempdir is automatically deleted after the test is run.
/// If you want to keep the tempdir for debugging, you can use `tempdir.into_path()`
///
/// Example:
///
/// ```
/// use marzano_test_utils::fixtures::get_fixture;
/// let (temp_dir, path) = get_fixture(env!("CARGO_MANIFEST_DIR"), "sample_dir", false).unwrap();
/// println!("dir: {:?}", temp_dir.into_path());
/// ```
pub fn get_fixture(
    manifest_dir: &str,
    subdirectory: &str,
    with_init: bool,
) -> Result<(tempfile::TempDir, std::path::PathBuf)> {
    // Create a temporary directory
    let temp_dir = tempdir()?;

    // Get the path of the temporary directory
    let temp_fixtures_root = temp_dir.path().to_path_buf();

    // Construct the source path for the subdirectory inside fixtures
    let mut fixtures_root = std::path::PathBuf::from(manifest_dir);
    fixtures_root.push("fixtures");
    fixtures_root.push(subdirectory);

    // Copy the contents of the subdirectory to the temporary directory
    let mut options = fs_extra::dir::CopyOptions::new();
    options.copy_inside = true;
    fs_extra::dir::copy(&fixtures_root, &temp_fixtures_root, &options)?;

    // Run init command if requested
    if with_init {
        todo!("run_init_cmd is not yet supported");
    }

    Ok((temp_dir, temp_fixtures_root.join(subdirectory)))
}
