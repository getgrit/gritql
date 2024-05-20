use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, cmp::Ordering, path::PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct RichPath {
    pub path: PathBuf,
    pub hash: Option<[u8; 32]>,
}

impl RichPath {
    pub fn new(path: PathBuf, hash: Option<[u8; 32]>) -> Self {
        Self { path, hash }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct RichFile {
    pub path: String,
    pub content: String,
}

impl RichFile {
    pub fn new(name: String, content: String) -> Self {
        Self {
            path: name,
            content,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl PartialOrd for RichFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.path.cmp(&other.path))
    }
}

impl Ord for RichFile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FileName for RichFile {
    fn name(&self) -> String {
        self.path.to_owned()
    }
}

impl FileName for &RichFile {
    fn name(&self) -> String {
        self.path.to_owned()
    }
}

// there must be a better way right?
impl FileName for &(RichFile, [u8; 32]) {
    fn name(&self) -> String {
        self.0.path.to_owned()
    }
}

impl FileName for PathBuf {
    fn name(&self) -> String {
        self.to_string_lossy().to_string()
    }
}

impl FileName for &RichPath {
    fn name(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
}

pub trait FileName {
    fn name(&self) -> String;
}

pub trait TryIntoInputFile {
    fn try_into_cow(&self) -> Result<Cow<RichFile>>;
}

// there must be a better way right?
impl TryIntoInputFile for &(RichFile, [u8; 32]) {
    fn try_into_cow(&self) -> Result<Cow<RichFile>> {
        Ok(Cow::Borrowed(&self.0))
    }
}

impl TryIntoInputFile for &RichFile {
    fn try_into_cow(&self) -> Result<Cow<RichFile>> {
        Ok(Cow::Borrowed(self))
    }
}

impl TryIntoInputFile for RichFile {
    fn try_into_cow(&self) -> Result<Cow<RichFile>> {
        Ok(Cow::Borrowed(self))
    }
}

impl TryIntoInputFile for PathBuf {
    fn try_into_cow(&self) -> Result<Cow<RichFile>> {
        let name = self.to_string_lossy().to_string();
        let content = fs_err::read_to_string(self).map_err(|e| anyhow!(e))?;
        Ok(Cow::Owned(RichFile::new(name, content)))
    }
}

impl TryIntoInputFile for &RichPath {
    fn try_into_cow(&self) -> Result<Cow<RichFile>> {
        let name = self.path.to_string_lossy().to_string();
        let content = fs_err::read_to_string(&self.path).map_err(|e| anyhow!(e))?;
        Ok(Cow::Owned(RichFile::new(name, content)))
    }
}

/// Core Marzano file trait
pub trait LoadableFile: TryIntoInputFile + FileName {}
impl<T> LoadableFile for T where T: TryIntoInputFile + FileName {}

/// All the required traits for processing a file in the Marzano engine
pub trait MarzanoFileTrait: TryIntoInputFile + FileName + Send + Sync + Clone {}

impl<T> MarzanoFileTrait for T where T: TryIntoInputFile + FileName + Send + Sync + Clone {}
