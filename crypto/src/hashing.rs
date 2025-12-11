use blake3::Hasher;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A 32-byte BLAKE3 hash used for content addressing and integrity checks.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ContentHash([u8; 32]);

impl ContentHash {
    /// Compute the BLAKE3 hash of a byte slice.
    pub fn hash(data: &[u8]) -> Self {
        let hash = blake3::hash(data);
        Self(*hash.as_bytes())
    }

    /// Returns the raw bytes of the hash.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Create a ContentHash from raw bytes.
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

// Display as lower-case hex string
impl fmt::Display for ContentHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

// Debug as hex string for easier logging
impl fmt::Debug for ContentHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ContentHash({})", hex::encode(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashing_consistency() {
        let data = b"Hello, decentralized world!";
        let hash1 = ContentHash::hash(data);
        let hash2 = ContentHash::hash(data);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hex_display() {
        let data = b"abc";
        let hash = ContentHash::hash(data);
        // BLAKE3("abc") known hash prefix check
        assert!(hash.to_string().starts_with("6437b3"));
    }
}