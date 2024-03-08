use anyhow::Result;

pub trait GritCache: Send + Sync {
    /// Check if the cache has marked that the file-pattern pair has no matches
    fn has_no_matches(&self, file_hash: [u8; 32], pattern_hash: [u8; 32]) -> bool;

    /// Mark that the file-pattern pair has no matches
    fn put_no_matches(&self, file_hash: [u8; 32], pattern_hash: [u8; 32]) -> Result<()>;
}

/// A cache that does nothing, useful for places where we don't have a cache available
pub struct NullCache {}

impl Default for NullCache {
    fn default() -> Self {
        Self::new()
    }
}

impl NullCache {
    pub fn new() -> Self {
        Self {}
    }
}

impl GritCache for NullCache {
    fn has_no_matches(&self, _file_hash: [u8; 32], _pattern_hash: [u8; 32]) -> bool {
        false
    }

    fn put_no_matches(&self, _file_hash: [u8; 32], _pattern_hash: [u8; 32]) -> Result<()> {
        Ok(())
    }
}
