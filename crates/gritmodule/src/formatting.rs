use anyhow::{Context, Result};
use log::{debug, info};
use marzano_language::target_language::PatternLanguage;
use marzano_util::rich_path::RichFile;
use std::env;
use tempfile::TempDir;
use tokio::{
    fs::{self, create_dir_all, OpenOptions},
    io::AsyncWriteExt,
    process::Command,
};

async fn preferred_formatter(dir: &TempDir) {
    log::debug!("Running code formatter");
    match env::var("FORCE_PRETTIER") {
        Ok(s) => {
            if s == "1" {
                prettier(dir, true).await;
            } else {
                biome(dir).await;
            }
        }
        Err(_) => biome(dir).await,
    }
}

async fn biome(dir: &TempDir) {
    match env::var("FORCE_NPX_BIOME") {
        Ok(v) => {
            if v == "1" {
                run_npx_biome(dir).await;
            } else {
                run_system_biome(dir).await;
            }
        }
        Err(_) => run_system_biome(dir).await,
    }
}

async fn run_npx_biome(dir: &TempDir) {
    run_biome(dir, vec!["npx", "@biomejs/biome", "format", "--write"]).await;
}

async fn run_system_biome(dir: &TempDir) {
    run_biome(dir, vec!["biome", "format", "--write"]).await
}

async fn run_biome(dir: &TempDir, commands: Vec<&str>) {
    let mut cmd = Command::new(&commands[0]);
    for a in &commands[1..] {
        cmd.arg(a);
    }
    cmd.arg(dir.path().join("**/*"));
    let output = cmd.output().await;
    log::debug!("biome output: {:?}", output)
}

async fn prettier(dir: &TempDir, babel: bool) {
    // npx has an interactive prompt asking if you want to install the package if it isn't installed.
    // we pass `--yes` to avoid the interactive prompt
    let mut cmd = Command::new("npx");
    cmd.arg("--yes").arg("prettier");

    if babel {
        cmd.arg("--parser").arg("babel-ts");
    }

    let output = cmd
        .arg("--write")
        .arg(dir.path().join("**/*"))
        .output()
        .await;

    log::debug!("prettier output: {:?}", output);
}

pub async fn format_rich_files(
    language: &PatternLanguage,
    files: Vec<RichFile>,
) -> Result<Vec<RichFile>> {
    let tempdir = TempDir::new()?;
    let mut target_file_paths = Vec::with_capacity(files.len());

    for (index, file) in files.iter().enumerate() {
        let file_path = tempdir.path().join(file.path.clone());

        let suffix = file_path.extension().unwrap_or_default();
        let new_file_path =
            file_path.with_extension(format!("{}.{}", index, suffix.to_str().unwrap_or_default()));
        let prefix = file_path.parent().unwrap();
        create_dir_all(prefix).await?;

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&new_file_path)
            .await
            .with_context(|| {
                format!(
                    "Failed to create file {} in tempdir {}",
                    file.path,
                    tempdir.path().display()
                )
            })?;

        f.write_all(file.content.as_bytes())
            .await
            .with_context(|| {
                format!(
                    "Failed to write file {} to tempdir {}",
                    file.path,
                    tempdir.path().display()
                )
            })?;

        target_file_paths.push(new_file_path);
    }

    // Format the whole dir
    format_temp_dir(&tempdir, vec![language]).await?;

    // Now gather them all
    let mut formatted_files: Vec<RichFile> = Vec::with_capacity(target_file_paths.len());

    for (index, file_path) in target_file_paths.iter().enumerate() {
        let content = fs::read_to_string(&file_path).await?;
        formatted_files.push(RichFile {
            path: files[index].path.clone(),
            content,
        });
    }

    Ok(formatted_files)
}

async fn format_temp_dir(dir: &TempDir, languages: Vec<&PatternLanguage>) -> Result<()> {
    if languages.contains(&&PatternLanguage::Java) {
        let mut cmd = Command::new("google-java-format");
        cmd.current_dir(dir);
        cmd.arg("--replace");
        for entry in fs_err::read_dir(dir.as_ref())? {
            let entry = entry?;
            if let Some(name) = entry.path().file_name() {
                if let Some(name_str) = name.to_str() {
                    cmd.arg(name_str);
                }
            }
        }
        let output = cmd.output().await;
        if let Err(e) = output {
            if e.kind() == std::io::ErrorKind::NotFound {
                log::warn!("google-java-format is not installed. If installed, it would have been used for Java formatting.");
            } else {
                return Err(e.into());
            }
        }
    }

    if languages.contains(&&PatternLanguage::Go) {
        let mut cmd = Command::new("gofmt");
        cmd.current_dir(dir);
        cmd.arg("-w");
        for entry in fs_err::read_dir(dir.as_ref())? {
            let entry = entry?;
            if let Some(name) = entry.path().file_name() {
                if let Some(name_str) = name.to_str() {
                    cmd.arg(name_str);
                }
            }
        }
        let output = cmd.output().await;
        if let Err(e) = output {
            if e.kind() == std::io::ErrorKind::NotFound {
                log::warn!("gofmt is not installed. If installed, it would have been used for Go formatting.");
            } else {
                return Err(e.into());
            }
        }
    }

    if languages.contains(&&PatternLanguage::Rust) {
        let mut cmd = Command::new("rustfmt");
        cmd.current_dir(dir);
        for entry in fs_err::read_dir(dir.as_ref())? {
            let entry = entry?;
            if let Some(name) = entry.path().file_name() {
                if let Some(name_str) = name.to_str() {
                    cmd.arg(name_str);
                }
            }
        }
        let output = cmd.output().await;
        if let Err(e) = output {
            if e.kind() == std::io::ErrorKind::NotFound {
                log::warn!("rustfmt is not installed. If installed, it would have been used for Rust formatting.");
            } else {
                return Err(e.into());
            }
        }
    }

    if languages.contains(&&PatternLanguage::Python) {
        let mut cmd = Command::new("ruff");
        cmd.current_dir(dir);
        cmd.arg("format");
        for entry in fs_err::read_dir(dir.as_ref())? {
            let entry = entry?;
            if let Some(name) = entry.path().file_name() {
                if let Some(name_str) = name.to_str() {
                    cmd.arg(name_str);
                }
            }
        }
        let output = cmd.output().await;
        if let Err(e) = output {
            if e.kind() == std::io::ErrorKind::NotFound {
                log::warn!("ruff is not installed. If installed, it would have been used for Python formatting.");
            } else {
                return Err(e.into());
            }
        }
    }

    if languages.contains(&&PatternLanguage::Hcl) {
        let mut cmd = Command::new("terraform");
        cmd.current_dir(dir);
        cmd.arg("fmt");

        for entry in fs_err::read_dir(dir.as_ref())? {
            let entry = entry?;
            if let Some(name) = entry.path().file_name() {
                if let Some(name_str) = name.to_str() {
                    cmd.arg(name_str);
                }
            }
        }

        let output = cmd.output().await;
        debug!("terraform output: {:?}", output);
        if let Err(e) = output {
            if e.kind() == std::io::ErrorKind::NotFound {
                log::warn!("Terraform is not installed. If installed, it would have been used for Hcl formatting.");
            } else {
                return Err(e.into());
            }
        }
    }

    if languages.contains(&&PatternLanguage::Tsx)
        || languages.contains(&&PatternLanguage::JavaScript)
        || languages.contains(&&PatternLanguage::TypeScript)
    {
        preferred_formatter(dir).await;
    }

    if languages.contains(&&PatternLanguage::Json)
        || languages.contains(&&PatternLanguage::Html)
        || languages.contains(&&PatternLanguage::Css)
    {
        preferred_formatter(dir).await;
    } else if languages.contains(&&PatternLanguage::MarkdownBlock)
        || languages.contains(&&PatternLanguage::MarkdownInline)
        || languages.contains(&&PatternLanguage::Yaml)
    {
        prettier(dir, false).await;
    }

    Ok(())
}
