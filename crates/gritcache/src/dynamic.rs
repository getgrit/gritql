use marzano_util::cache::{GritCache, NullCache};

use crate::new_cache::ThreadedCache;

pub enum DynamicCache {
    Threaded(ThreadedCache),
    Null(NullCache),
}

impl DynamicCache {
    pub fn is_useful(&self) -> bool {
        match self {
            DynamicCache::Threaded(_) => true,
            DynamicCache::Null(_) => false,
        }
    }
}

impl GritCache for DynamicCache {
    fn has_no_matches(&self, file_hash: [u8; 32], pattern_hash: [u8; 32]) -> bool {
        match self {
            DynamicCache::Threaded(cache) => cache.has_no_matches(file_hash, pattern_hash),
            DynamicCache::Null(cache) => cache.has_no_matches(file_hash, pattern_hash),
        }
    }

    fn put_no_matches(&self, file_hash: [u8; 32], pattern_hash: [u8; 32]) -> anyhow::Result<()> {
        match self {
            DynamicCache::Threaded(cache) => cache.put_no_matches(file_hash, pattern_hash),
            DynamicCache::Null(cache) => cache.put_no_matches(file_hash, pattern_hash),
        }
    }
}
