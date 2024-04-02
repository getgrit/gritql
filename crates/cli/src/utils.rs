use clap::ValueEnum;
use git2::{Repository, StatusOptions};
use marzano_gritmodule::searcher::find_git_dir_from;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    net::{SocketAddr, TcpListener},
    path::PathBuf,
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize, ValueEnum)]
pub enum OperatingSystem {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "macos")]
    MacOS,
}

impl Display for OperatingSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatingSystem::Windows => write!(f, "windows"),
            OperatingSystem::Linux => write!(f, "linux"),
            OperatingSystem::MacOS => write!(f, "macos"),
        }
    }
}

pub fn get_client_os() -> &'static str {
    match std::env::consts::OS {
        "macos" => "macos",
        "linux" => "linux",
        "windows" => "windows",
        "darwin" => "macos",
        _ => "linux",
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize, ValueEnum)]
pub enum Architecture {
    #[serde(rename = "x64")]
    X64,
    #[serde(rename = "arm64")]
    Arm64,
}

impl Display for Architecture {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Architecture::X64 => write!(f, "x64"),
            Architecture::Arm64 => write!(f, "arm64"),
        }
    }
}

pub fn get_client_arch() -> &'static str {
    match std::env::consts::ARCH {
        "x86_64" => "x64",
        "aarch64" => "arm64",
        // Fall back to x64
        _ => "x64",
    }
}

#[allow(dead_code)]
pub fn get_random_port() -> Option<u16> {
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))).ok()?;
    let port = listener.local_addr().ok()?.port();
    drop(listener);
    Some(port)
}

pub fn is_pattern_name(pattern: &str) -> bool {
    let regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*(\(\))?$").unwrap();
    regex.is_match(pattern)
}

pub async fn has_uncommitted_changes(dir: PathBuf) -> bool {
    let git_dir = match find_git_dir_from(dir).await {
        Some(git_dir) => git_dir,
        None => return false,
    };

    let repo = match Repository::open(git_dir) {
        Ok(repo) => repo,
        Err(_) => return false,
    };

    let mut options = StatusOptions::new();
    options.include_untracked(true).recurse_untracked_dirs(true);

    let statuses = match repo.statuses(Some(&mut options)) {
        Ok(statuses) => statuses,
        Err(_) => return false,
    };

    let has_uncommitted_changes = statuses.iter().any(|s| {
        s.status().is_wt_modified() || s.status().is_wt_new() || s.status().is_wt_deleted()
    });

    has_uncommitted_changes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_port() {
        let port = get_random_port();
        assert!(port.is_some());
        let port = port.unwrap();
        assert!(port > 0);
        println!("Random port: {}", port);
    }
}

