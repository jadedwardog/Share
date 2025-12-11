use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2, Params,
};
use crate::CryptoError;

/// Derives a 32-byte recovery key from a user passphrase using Argon2id.
///
/// # Parameters (as per CRYPTO_PROTOCOL_SPEC.md)
/// * Memory (m): 64 MB (65536 KB)
/// * Iterations (t): 2
/// * Parallelism (p): 4
pub fn derive_recovery_key(passphrase: &str, salt: &str) -> Result<[u8; 32], CryptoError> {
    // 1. Configure Argon2id params
    let params = Params::new(65536, 2, 4, None)
        .map_err(|e| CryptoError::KdfError)?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V13, params);

    // 2. Parse the salt (must be ASCII for SaltString)
    // In production, salts should be CSPRNG generated and stored.
    let salt_string = SaltString::from_b64(salt)
        .map_err(|_| CryptoError::KdfError)?;

    // 3. Hash the password
    let password_hash = argon2
        .hash_password(passphrase.as_bytes(), &salt_string)
        .map_err(|e| CryptoError::KdfError)?;

    // 4. Extract the raw output key (the "hash" part)
    // Note: We extract the raw 32 bytes to use as the Recovery Key seed.
    let mut output_key = [0u8; 32];
    
    // Argon2 outputs a structured string. We need to grab the hash part if we want raw bytes,
    // or we can use the `hash_password_custom` flow if we need exact raw bytes derivation.
    // For simplicity and standard compliance, we rely on the PHC string for verification,
    // but for a "Recovery Key", we often treat the hash output as the seed.
    // BELOW is a simplified extraction for the sake of the keypair seed.
    
    if let Some(hash) = password_hash.hash {
        let bytes = hash.as_bytes();
        if bytes.len() >= 32 {
            output_key.copy_from_slice(&bytes[0..32]);
            Ok(output_key)
        } else {
            Err(CryptoError::KdfError)
        }
    } else {
        Err(CryptoError::KdfError)
    }
}

/// Generates a random salt for new account creation.
pub fn generate_salt() -> String {
    SaltString::generate(&mut OsRng).as_str().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdf_consistency() {
        let password = "correct horse battery staple";
        let salt = generate_salt();

        // Derivation should be deterministic for same salt
        let key1 = derive_recovery_key(password, &salt).unwrap();
        let key2 = derive_recovery_key(password, &salt).unwrap();

        assert_eq!(key1, key2);
    }

    #[test]
    fn test_kdf_uniqueness() {
        let password = "same password";
        let salt1 = generate_salt();
        let salt2 = generate_salt();

        // Different salts must produce different keys
        let key1 = derive_recovery_key(password, &salt1).unwrap();
        let key2 = derive_recovery_key(password, &salt2).unwrap();

        assert_ne!(key1, key2);
    }
}