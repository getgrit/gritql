use sha2::{Digest, Sha256};

pub fn hash(src: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(src);
    let result = hasher.finalize();
    result.into()
}
