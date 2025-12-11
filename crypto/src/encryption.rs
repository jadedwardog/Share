use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    XChaCha20Poly1305, XNonce,
};
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use crate::CryptoError;

/// The size of the symmetric key in bytes (32 bytes = 256 bits).
pub const KEY_SIZE: usize = 32;
/// The size of the XChaCha20 nonce in bytes (24 bytes = 192 bits).
pub const NONCE_SIZE: usize = 24;

/// Represents an encrypted payload, including the unique nonce used.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ciphertext {
    /// The random 24-byte nonce used for this specific encryption.
    pub nonce: Vec<u8>,
    /// The actual encrypted data (including the Poly1305 tag).
    pub data: Vec<u8>,
}

/// A wrapper for the symmetric Data Encryption Key (DEK).
#[derive(Clone)]
pub struct SymmetricKey([u8; KEY_SIZE]);

impl SymmetricKey {
    /// Generate a new random symmetric key.
    pub fn generate() -> Self {
        let mut key_bytes = [0u8; KEY_SIZE];
        OsRng.fill_bytes(&mut key_bytes);
        Self(key_bytes)
    }

    /// Create a key from existing bytes (e.g., after unwrapping an Envelope).
    pub fn from_bytes(bytes: [u8; KEY_SIZE]) -> Self {
        Self(bytes)
    }

    /// access raw bytes
    pub fn as_bytes(&self) -> &[u8; KEY_SIZE] {
        &self.0
    }
}

/// Encrypts data using XChaCha20-Poly1305.
///
/// # Arguments
/// * `plaintext` - The data to encrypt.
/// * `key` - The symmetric key (DEK).
/// * `aad` - Additional Authenticated Data (e.g., Manifest Header) to bind to the ciphertext.
pub fn encrypt(
    plaintext: &[u8],
    key: &SymmetricKey,
    aad: &[u8],
) -> Result<Ciphertext, CryptoError> {
    // 1. Initialize the cipher with the key
    let cipher = XChaCha20Poly1305::new(key.as_bytes().into());

    // 2. Generate a fresh random nonce (24 bytes)
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = XNonce::from_slice(&nonce_bytes);

    // 3. Construct the AEAD payload (Msg + AAD)
    let payload = Payload {
        msg: plaintext,
        aad,
    };

    // 4. Encrypt
    let encrypted_data = cipher
        .encrypt(nonce, payload)
        .map_err(|_| CryptoError::EncryptionError("AEAD encryption failure".into()))?;

    Ok(Ciphertext {
        nonce: nonce_bytes.to_vec(),
        data: encrypted_data,
    })
}

/// Decrypts data using XChaCha20-Poly1305.
///
/// # Arguments
/// * `ciphertext` - The encrypted struct containing data and nonce.
/// * `key` - The symmetric key (DEK).
/// * `aad` - The EXACT same AAD used during encryption. Mismatches will cause failure.
pub fn decrypt(
    ciphertext: &Ciphertext,
    key: &SymmetricKey,
    aad: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    // 1. Initialize the cipher
    let cipher = XChaCha20Poly1305::new(key.as_bytes().into());

    // 2. Extract nonce
    if ciphertext.nonce.len() != NONCE_SIZE {
        return Err(CryptoError::DecryptionError("Invalid nonce length".into()));
    }
    let nonce = XNonce::from_slice(&ciphertext.nonce);

    // 3. Construct payload for authentication check
    let payload = Payload {
        msg: &ciphertext.data,
        aad,
    };

    // 4. Decrypt and Verify
    let plaintext = cipher
        .decrypt(nonce, payload)
        .map_err(|_| CryptoError::DecryptionError("AEAD verification failed (bad key, nonce, or AAD)".into()))?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_roundtrip() {
        let key = SymmetricKey::generate();
        let data = b"Confidential Payload";
        let aad = b"Manifest Header v1";

        // Encrypt
        let ciphertext = encrypt(data, &key, aad).unwrap();
        assert_ne!(ciphertext.data, data); // Should be encrypted

        // Decrypt with correct AAD
        let decrypted = decrypt(&ciphertext, &key, aad).unwrap();
        assert_eq!(decrypted, data);
    }

    #[test]
    fn test_aad_mismatch() {
        let key = SymmetricKey::generate();
        let data = b"Secret";
        let aad = b"Context A";
        let aad_wrong = b"Context B";

        let ciphertext = encrypt(data, &key, aad).unwrap();

        // Attempt decrypt with wrong AAD
        let result = decrypt(&ciphertext, &key, aad_wrong);
        assert!(result.is_err());
    }
}