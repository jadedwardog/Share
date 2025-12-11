use crate::encryption::{SymmetricKey, KEY_SIZE};
use crate::CryptoError;
use aes_kw::KekAes256;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use x25519_dalek::{PublicKey, StaticSecret};

/// Represents a secure transmission of a symmetric key to a recipient.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Envelope {
    /// The ephemeral public key of the sender (for this specific envelope).
    /// Allows the recipient to perform ECDH to recover the KEK.
    pub ephemeral_pub: [u8; 32],

    /// The recipient's public identity (Ed25519 converted to X25519) or specific enc key.
    pub recipient_id: [u8; 32],

    /// Unix timestamp (seconds) when this key access expires.
    pub expiry: u64,

    /// The wrapped Data Encryption Key (DEK).
    /// Encrypted using AES-KW with the derived KEK.
    pub wrapped_key: Vec<u8>,
}

impl Envelope {
    /// Create a new envelope containing a SymmetricKey for a specific recipient.
    ///
    /// # Arguments
    /// * `dek` - The Data Encryption Key to protect.
    /// * `recipient_pub_bytes` - The recipient's X25519 public key bytes.
    /// * `expiry` - Expiration timestamp.
    pub fn seal(
        dek: &SymmetricKey,
        recipient_pub_bytes: [u8; 32],
        expiry: u64,
    ) -> Result<Self, CryptoError> {
        // 1. Generate Ephemeral Keypair for this envelope
        let ephemeral_secret = StaticSecret::random_from_rng(OsRng);
        let ephemeral_pub = PublicKey::from(&ephemeral_secret);

        // 2. Perform ECDH: Ephemeral_Priv * Recipient_Pub
        let recipient_pub = PublicKey::from(recipient_pub_bytes);
        let shared_secret = ephemeral_secret.diffie_hellman(&recipient_pub);

        // 3. Derive KEK (Key Encryption Key) from Shared Secret
        // Use SHA-256 to distill the curve point into a uniform key
        let mut hasher = Sha256::new();
        hasher.update(shared_secret.as_bytes());
        let kek_bytes: [u8; 32] = hasher.finalize().into();

        // 4. Wrap the DEK using AES-KW
        let kek = KekAes256::from(kek_bytes);
        let wrapped_key = kek
            .wrap_vec(dek.as_bytes())
            .map_err(|_| CryptoError::EncryptionError("Key wrapping failed".into()))?;

        Ok(Envelope {
            ephemeral_pub: *ephemeral_pub.as_bytes(),
            recipient_id: recipient_pub_bytes,
            expiry,
            wrapped_key,
        })
    }

    /// Open an envelope to recover the SymmetricKey.
    ///
    /// # Arguments
    /// * `recipient_secret_bytes` - The recipient's X25519 private key.
    pub fn open(
        &self,
        recipient_secret_bytes: [u8; 32],
    ) -> Result<SymmetricKey, CryptoError> {
        // 1. Reconstruct Recipient Secret
        let recipient_secret = StaticSecret::from(recipient_secret_bytes);

        // 2. Perform ECDH: Recipient_Priv * Ephemeral_Pub
        let ephemeral_pub = PublicKey::from(self.ephemeral_pub);
        let shared_secret = recipient_secret.diffie_hellman(&ephemeral_pub);

        // 3. Derive KEK
        let mut hasher = Sha256::new();
        hasher.update(shared_secret.as_bytes());
        let kek_bytes: [u8; 32] = hasher.finalize().into();

        // 4. Unwrap the DEK
        let kek = KekAes256::from(kek_bytes);
        let dek_bytes_vec = kek
            .unwrap_vec(&self.wrapped_key)
            .map_err(|_| CryptoError::DecryptionError("Key unwrapping failed".into()))?;

        if dek_bytes_vec.len() != KEY_SIZE {
            return Err(CryptoError::DecryptionError("Invalid unwrapped key length".into()));
        }

        let mut arr = [0u8; KEY_SIZE];
        arr.copy_from_slice(&dek_bytes_vec);
        Ok(SymmetricKey::from_bytes(arr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_flow() {
        // Recipient Setup
        let rec_secret = StaticSecret::random_from_rng(OsRng);
        let rec_pub = PublicKey::from(&rec_secret);

        // Data Key to Protect
        let dek = SymmetricKey::generate();

        // Seal
        let envelope = Envelope::seal(
            &dek, 
            *rec_pub.as_bytes(), 
            9999999999
        ).unwrap();

        // Open
        let recovered_dek = envelope.open(rec_secret.to_bytes()).unwrap();

        assert_eq!(dek.as_bytes(), recovered_dek.as_bytes());
    }
}