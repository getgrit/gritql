use anyhow::anyhow;
use anyhow::Result;
use path_absolutize::Absolutize;
use std::path::{Path, PathBuf};

#[cfg(feature = "absolute_filename")]
pub(crate) fn absolutize(path: &Path) -> Result<PathBuf> {
    path.absolutize()
        .map(|path| path.to_path_buf())
        .map_err(|_| anyhow!("could not build absolute path from file name"))
}

#[cfg(not(feature = "absolute_filename"))]
pub(crate) fn absolutize(path: &Path) -> Result<PathBuf> {
    Ok(path.to_owned())
}

pub(crate) fn resolve(target_path: &Path, from_file: &Path) -> Result<PathBuf> {
    let source_path = from_file.parent().ok_or_else(|| {
        anyhow!(
            "could not get parent directory of file name {}",
            from_file.display()
        )
    })?;
    let absolutized = target_path.absolutize_from(source_path)?;
    // path.push(target_path);
    Ok(absolutized.to_path_buf())
}
