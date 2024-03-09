use anyhow::Result;
use tempfile::TempDir;
use tokio::fs;

pub async fn initialize_grit(dir: &TempDir, config: &str) -> Result<()> {
    let grit_dir = dir.path().join(".grit");
    let yml_path = grit_dir.join("grit.yml");

    fs::create_dir_all(&grit_dir).await.unwrap_or_else(|_| {
        panic!(
            "Failed to create grit dir at {:?}",
            grit_dir.to_str().unwrap()
        )
    });
    fs::write(&yml_path, config).await.unwrap_or_else(|_| {
        panic!(
            "Failed to write grit.yml to {:?}",
            yml_path.to_str().unwrap()
        )
    });

    Ok(())
}
