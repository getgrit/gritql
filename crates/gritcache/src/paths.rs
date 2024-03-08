use std::{path::PathBuf, thread::JoinHandle};

use anyhow::Result;
use marzano_gritmodule::searcher::{find_global_grit_modules_dir, find_grit_modules_dir};
use marzano_util::cache::NullCache;

use crate::{dynamic::DynamicCache, new_cache::ThreadedCache};

pub async fn cache_dir(current_path: PathBuf) -> Result<PathBuf> {
    // If GRIT_CACHE_DIR env var is set, use that
    if let Ok(dir) = std::env::var("GRIT_CACHE_DIR") {
        return Ok(PathBuf::from(dir));
    }

    // Try to find local grit modules dir
    let local_grit_modules_dir = find_grit_modules_dir(current_path).await;
    if local_grit_modules_dir.is_ok() {
        return local_grit_modules_dir;
    }

    // Try to find global grit modules dir
    find_global_grit_modules_dir().await
}

/// Create a cache for the current working directory
/// A "null" cache will not actually do any caching
pub async fn cache_for_cwd(
    refresh: bool,
    null_cache: bool,
) -> Result<(DynamicCache, Option<JoinHandle<()>>)> {
    let cache_dir = cache_dir(std::env::current_dir()?).await?;
    if null_cache {
        Ok((DynamicCache::Null(NullCache::new()), None))
    } else {
        let (cache, manager) = ThreadedCache::new(cache_dir, refresh).await?;
        Ok((DynamicCache::Threaded(cache), Some(manager)))
    }
}
