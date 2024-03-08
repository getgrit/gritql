use anyhow::anyhow;
use anyhow::Result;
use std::borrow::Cow;
use std::path::Path;

use path_absolutize::Absolutize;

#[cfg(feature = "absolute_filename")]
pub(crate) fn absolutize(path: &str) -> Result<String> {
    Ok(Path::new(path)
        .absolutize()?
        .to_str()
        .ok_or_else(|| anyhow!("could not build absolute path from file name"))?
        .to_owned())
}

#[cfg(not(feature = "absolute_filename"))]
pub(crate) fn absolutize(path: &str) -> Result<String> {
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
