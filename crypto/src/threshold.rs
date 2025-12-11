use threshold_crypto::{
    PublicKeySet, SecretKeyShare, SignatureShare, Combine, Ciphertext,
};
use serde::{Deserialize, Serialize};
use crate::CryptoError;

/// A wrapper around the Council's aggregated public key set.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CouncilKeySet(PublicKeySet);

impl CouncilKeySet {
    /// Create from raw bytes (usually from on-chain storage).
    /// Note: Implementation depends on serialization format of threshold_crypto.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError> {
        bincode::deserialize(bytes)
            .map(CouncilKeySet)
            .map_err(|_| CryptoError::ThresholdError)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&self.0).unwrap_or_default()
    }

    /// Verify a fully aggregated signature from the council.
    pub fn verify(&self, message: &[u8], signature: &CombinedSignature) -> bool {
        self.0.public_key().verify(&signature.0, message)
    }

    /// The threshold required to reconstruct a signature (e.g., 7 out of 10).
    pub fn threshold(&self) -> usize {
        self.0.threshold()
    }
}

/// A single council member's secret key share.
pub struct MemberSecret(SecretKeyShare);

impl MemberSecret {
    /// Sign a proposal hash to create a partial vote.
    pub fn sign(&self, message: &[u8]) -> PartialVote {
        PartialVote(self.0.sign(message))
    }
}

/// A partial signature (vote) from one council member.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PartialVote(SignatureShare);

/// The final aggregated signature representing the Council's will.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CombinedSignature(threshold_crypto::Signature);

impl CombinedSignature {
    /// Aggregate partial votes into a final signature.
    pub fn combine(
        key_set: &CouncilKeySet,
        shares: Vec<(usize, &PartialVote)>,
    ) -> Result<Self, CryptoError> {
        // Map our wrapper types to the library types
        let internal_shares: Vec<(usize, &SignatureShare)> = shares
            .iter()
            .map(|(idx, vote)| (*idx, &vote.0))
            .collect();

        key_set.0
            .combine_signatures(internal_shares)
            .map(CombinedSignature)
            .map_err(|_| CryptoError::ThresholdError)
    }
}

// Helper for testing generation
#[cfg(test)]
mod tests {
    use super::*;
    use threshold_crypto::SecretKeySet;

    #[test]
    fn test_threshold_flow() {
        let mut rng = rand::thread_rng();
        // 1. DKG Simulation: Create a set for 5 members, threshold 3
        let sk_set = SecretKeySet::random(3, &mut rng);
        let pk_set = CouncilKeySet(sk_set.public_keys());

        let msg = b"Ratify Fork v2.0";

        // 2. Members sign individually
        let member_0 = MemberSecret(sk_set.secret_key_share(0));
        let member_1 = MemberSecret(sk_set.secret_key_share(1));
        let member_2 = MemberSecret(sk_set.secret_key_share(2));
        let member_3 = MemberSecret(sk_set.secret_key_share(3));

        let vote_0 = member_0.sign(msg);
        let vote_1 = member_1.sign(msg);
        let vote_3 = member_3.sign(msg);

        // 3. Aggregate (Indices must match the key share index)
        let shares = vec![
            (0, &vote_0),
            (1, &vote_1),
            (3, &vote_3),
        ];

        let combined = CombinedSignature::combine(&pk_set, shares).expect("Aggregation failed");

        // 4. Verify
        assert!(pk_set.verify(msg, &combined));
    }
}