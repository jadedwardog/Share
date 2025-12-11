use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey, Signature};
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use std::fmt;
use crate::CryptoError;

/// Represents a Public Identity (User ID).
/// This is the public half of the Ed25519 keypair.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct IdentityKey(VerifyingKey);

impl IdentityKey {
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, CryptoError> {
        VerifyingKey::from_bytes(bytes)
            .map(IdentityKey)
            .map_err(|_| CryptoError::InvalidSignature)
    }

    pub fn as_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    /// Verify a signature against a message.
    pub fn verify(&self, message: &[u8], signature_bytes: &[u8; 64]) -> Result<(), CryptoError> {
        let signature = Signature::from_bytes(signature_bytes);
        self.0.verify(message, &signature)
            .map_err(|_| CryptoError::InvalidSignature)
    }
}

// Custom Serde implementation to serialize as hex strings (friendly for JSON manifests)
impl Serialize for IdentityKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex_string = hex::encode(self.as_bytes());
        serializer.serialize_str(&hex_string)
    }
}

impl<'de> Deserialize<'de> for IdentityKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = hex::decode(s).map_err(serde::de::Error::custom)?;
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom("Invalid key length"));
        }
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        IdentityKey::from_bytes(&array).map_err(serde::de::Error::custom)
    }
}

/// A wrapper for the private Signing Key.
/// NEVER serialize this or expose it in logs.
pub struct UserSecret(SigningKey);

impl UserSecret {
    /// Generate a new random keypair.
    pub fn generate() -> Self {
        let mut csprng = rand::rngs::OsRng;
        UserSecret(SigningKey::generate(&mut csprng))
    }

    /// Sign a message.
    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        self.0.sign(message).to_bytes()
    }

    /// Get the corresponding public identity.
    pub fn public_key(&self) -> IdentityKey {
        IdentityKey(self.0.verifying_key())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signing_flow() {
        let secret = UserSecret::generate();
        let public = secret.public_key();
        let message = b"I authorize this post.";

        // Sign
        let sig = secret.sign(message);

        // Verify
        assert!(public.verify(message, &sig).is_ok());
    }

    #[test]
    fn test_serialization() {
        let secret = UserSecret::generate();
        let public = secret.public_key();

        // Serialize to JSON
        let json = serde_json::to_string(&public).unwrap();
        // Should look like a hex string
        assert!(json.contains("\"")); 

        // Deserialize back
        let deserialized: IdentityKey = serde_json::from_str(&json).unwrap();
        assert_eq!(public, deserialized);
    }
}