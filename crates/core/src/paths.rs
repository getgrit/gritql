use anyhow::anyhow;
use anyhow::Result;
use path_absolutize::Absolutize;
use std::borrow::Cow;
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

pub(crate) fn resolve<'a>(target_path: Cow<'a, str>, from_file: Cow<'a, str>) -> Result<String> {
    let source_path = Path::new(from_file.as_ref()).parent().ok_or_else(|| {
        anyhow!(
            "could not get parent directory of file name {}",
            from_file.as_ref()
        )
    })?;
    let our_path = Path::new(target_path.as_ref());
    let absolutized = our_path.absolutize_from(source_path)?;
    // path.push(target_path);
    Ok(absolutized
        .to_str()
        .ok_or_else(|| {
            anyhow!(
                "could not build absolute path from file name {}",
                target_path
            )
        })?
        .to_owned())
}
