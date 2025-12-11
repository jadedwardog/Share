// Crypto Crate Entry Point
// This crate exposes the specific algorithms defined in CRYPTO_PROTOCOL_SPEC.md

pub mod hashing;
pub mod encryption;
pub mod kdf;
pub mod signatures;
pub mod threshold;
pub mod envelopes;

// Re-exports for easier access
pub use hashing::ContentHash;
pub use encryption::Ciphertext;
pub use signatures::IdentityKey;

/// Common error types for cryptographic operations
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Encryption failure: {0}")]
    EncryptionError(String),
    #[error("Decryption failure: {0}")]
    DecryptionError(String),
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Key derivation failed")]
    KdfError,
    #[error("Threshold signature aggregation failed")]
    ThresholdError,
}