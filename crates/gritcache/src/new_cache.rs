use anyhow::{Context, Result};
use marzano_util::cache::GritCache;
use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::thread::JoinHandle;

/// A HashKey is a 64 byte array, which is the result of combining a file hash and a pattern hash.
type HashKey = [u8; 64];

/// ThreadCache leverages multi-threading to make caching safe and concurrent.
/// - Reading can be done from any thread (safely and directly), without locks
/// - Writing is done from a dedicated thread, and is sent to the cache via a channel
/// - Subsequent reads are *not* blocked by writes, but may return stale data
pub struct ThreadedCache {
    /// The cache itself, the keys are created by combining a file has and a pattern hash.
    /// The value is a boolean, true if there are *no* matches, false if there might be matches.
    no_matches: HashMap<HashKey, bool>,

    /// The channel to send new misses to the cache
    sender: Sender<HashKey>,
}

const MISMATCHES_CACHE_NAME: &str = "mismatches_cache";
const MISMATCHES_CACHE_VERSION: u8 = 2;

impl ThreadedCache {
    /// Create a new ThreadedCache
    pub(crate) async fn new(dir: PathBuf, refresh: bool) -> Result<(Self, JoinHandle<()>)> {
        let mismatches_path = dir.join(MISMATCHES_CACHE_NAME);

        let no_matches = if refresh {
            Self::reset(&mismatches_path)?;
            HashMap::new()
        } else {
            Self::initialize(&mismatches_path)?
        };

        let (sender, receiver) = mpsc::channel::<HashKey>();
        let mut writer = Self::new_writer(&mismatches_path)?;
        let manager = thread::spawn(move || {
            while let Ok(key) = receiver.recv() {
                writer.write_all(&key).unwrap();
            }
        });

        Ok((Self { no_matches, sender }, manager))
    }

    fn reset(path: &PathBuf) -> Result<()> {
        let mut writer = BufWriter::new(
            std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)
                .context("Failed to truncate cache file".to_string())?,
        );
        writer.write_all(&[MISMATCHES_CACHE_VERSION])?;
        Ok(())
    }

    fn initialize(path: &PathBuf) -> Result<HashMap<HashKey, bool>> {
        let file_vector = match std::fs::read(path) {
            Ok(bytes) => bytes,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    return Ok(HashMap::new());
                } else {
                    return Err(e).context(format!(
                        "Failed to read cache file {}",
                        path.to_string_lossy()
                    ));
                }
            }
        };

        if file_vector.is_empty() {
            return Ok(HashMap::new());
        }
        let (version_byte, key_vector) = file_vector.split_at(1);
        if (version_byte[0]) != MISMATCHES_CACHE_VERSION {
            Self::reset(path)?;
            return Ok(HashMap::new());
        }

        let map: Result<HashMap<_, _>> = key_vector
            .chunks_exact(64)
            .map(|chunk| {
                let array: Result<HashKey, _> = chunk.try_into();
                array.map_err(anyhow::Error::from)
            })
            .map(|r| r.map(|a| (a, true)))
            .collect();
        let map = map?;

        Ok(map)
    }

    fn new_writer(path: &PathBuf) -> Result<BufWriter<std::fs::File>> {
        let writer = BufWriter::new(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .context("Failed to open cache file".to_string())
                .context("Please run `grit init` or set GRIT_CACHE_DIR to cache check results")?,
        );

        Ok(writer)
    }

    fn key(file_hash: [u8; 32], pattern_hash: [u8; 32]) -> HashKey {
        // Concat hashes into a single key
        let mut key = [0u8; 64];
        let (first_half, second_half) = key.split_at_mut(32);
        first_half.copy_from_slice(&file_hash);
        second_half.copy_from_slice(&pattern_hash);
        key
    }
}

impl GritCache for ThreadedCache {
    fn has_no_matches(&self, file_hash: [u8; 32], pattern_hash: [u8; 32]) -> bool {
        let key = Self::key(file_hash, pattern_hash);
        self.no_matches.contains_key(&key)
    }

    fn put_no_matches(&self, file_hash: [u8; 32], pattern_hash: [u8; 32]) -> Result<()> {
        if self.has_no_matches(file_hash, pattern_hash) {
            return Ok(());
        }

        let key = Self::key(file_hash, pattern_hash);

        // Send the key to the cache
        self.sender.send(key)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use marzano_util::hasher::hash;

    use super::*;
    use std::path::PathBuf;
    fn assert_is_mismatch(cache: &ThreadedCache, file_hash: [u8; 32], pattern_hash: [u8; 32]) {
        assert!(
            cache.has_no_matches(file_hash, pattern_hash),
            "Should be a known mismatch in the cache",
        );
    }

    fn assert_is_not_mismatch(cache: &ThreadedCache, file_hash: [u8; 32], pattern_hash: [u8; 32]) {
        assert!(
            !cache.has_no_matches(file_hash, pattern_hash),
            "Should not be a known mismatch in the cachefile1_hash",
        );
    }

    #[tokio::test]
    async fn test_mismatches_cache() -> Result<()> {
        let file1_hash = hash("&file1");
        let file2_hash = hash("&file2");
        let file3_hash = hash("&file3");
        let pattern1_hash = hash("&pattern1");
        let pattern2_hash = hash("&pattern2");
        let pattern3_hash = hash("&pattern3");

        let path = PathBuf::from(".");
        let mismatches_cache_path = path.join(MISMATCHES_CACHE_NAME);
        let bad_path = PathBuf::from("./doesnotexist").join(MISMATCHES_CACHE_NAME);

        println!(
            "mismatches_cache_path: {}",
            mismatches_cache_path.to_string_lossy(),
        );

        // Delete file if exists
        if mismatches_cache_path.exists() {
            std::fs::remove_file(&mismatches_cache_path)?;
        }

        // assert cache creation fails gracefully on invalid paths
        assert!(ThreadedCache::new(bad_path.clone(), false).await.is_err());

        // Create an empty cache
        let (cache, manager) = ThreadedCache::new(path.clone(), true).await?;

        // Check that the cache is empty
        assert_is_not_mismatch(&cache, file1_hash, pattern1_hash);
        assert_is_not_mismatch(&cache, file2_hash, pattern2_hash);
        assert_is_not_mismatch(&cache, file3_hash, pattern3_hash);

        // Insert file1-pattern1 into the cache
        cache.put_no_matches(file1_hash, pattern1_hash)?;

        drop(cache);

        // Flush the manager
        manager.join().unwrap();

        // Check that the file exists
        assert!(
            mismatches_cache_path.exists(),
            "The mismatches cache file should exist"
        );

        // Read cache back
        let (cache, manager) = ThreadedCache::new(path.clone(), false).await?;

        // Check that only file1-pattern1 is still in the cache
        assert_is_mismatch(&cache, file1_hash, pattern1_hash);
        assert_is_not_mismatch(&cache, file2_hash, pattern2_hash);
        assert_is_not_mismatch(&cache, file3_hash, pattern3_hash);

        drop(cache);
        manager.join().unwrap();

        // Reset the cache
        let (cache, manager) = ThreadedCache::new(path.clone(), true).await?;

        // Check that the cache is empty
        assert_is_not_mismatch(&cache, file1_hash, pattern1_hash);
        assert_is_not_mismatch(&cache, file2_hash, pattern2_hash);
        assert_is_not_mismatch(&cache, file3_hash, pattern3_hash);

        // Check that the file exists
        assert!(
            mismatches_cache_path.exists(),
            "The mismatches cache file should exist"
        );

        // Insert file1-pattern1 into the cache
        cache.put_no_matches(file1_hash, pattern1_hash)?;

        // Flush the manager
        drop(cache);
        manager.join().unwrap();

        // Open it again without resetting and insert file2-pattern2
        let (cache, manager) = ThreadedCache::new(path.clone(), false).await?;
        cache.put_no_matches(file2_hash, pattern2_hash)?;

        // Flush the manager
        drop(cache);
        manager.join().unwrap();

        // Write and read back the cache
        let (cache, manager) = ThreadedCache::new(path.clone(), false).await?;

        // Contents are the same as before
        assert_is_mismatch(&cache, file1_hash, pattern1_hash);
        assert_is_mismatch(&cache, file2_hash, pattern2_hash);
        assert_is_not_mismatch(&cache, file3_hash, pattern3_hash);

        drop(cache);
        manager.join().unwrap();

        // Delete file
        std::fs::remove_file(mismatches_cache_path.clone())?;
        Ok(())
    }
}
