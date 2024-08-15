
use git2::{Repository, StatusOptions};
use marzano_gritmodule::searcher::find_git_dir_from;

use std::{
    net::{SocketAddr, TcpListener},
    path::PathBuf,
};

#[allow(dead_code)]
pub fn get_random_port() -> Option<u16> {
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))).ok()?;
    let port = listener.local_addr().ok()?.port();
    drop(listener);
    Some(port)
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
