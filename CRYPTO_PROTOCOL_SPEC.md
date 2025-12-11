# **Cryptographic Protocol Specification**

## **1\. Overview**

This document defines the cryptographic primitives, data structures, and protocols required to enforce the guarantees outlined in the TECHNICAL\_SPEC.md and GOVERNANCE\_CHARTER.md. All implementations must adhere strictly to these algorithms to ensure interoperability and security.

## **2\. Primitive Selection**

| Component | Algorithm | Implementation Crate | Rationale |
| :---- | :---- | :---- | :---- |
| Content Hashing | BLAKE3 | blake3 | Superior speed/security ratio; supports Merkle tree parallelism for large files. |
| Symmetric Encryption | XChaCha20-Poly1305 | chacha20poly1305 | Extended nonce support (192-bit) eliminates risk of nonce reuse collisions during random generation. |
| Key Derivation (KDF) | Argon2id | argon2 | Memory-hard password hashing to resist GPU/ASIC cracking of recovery keys. |
| Digital Signatures | Ed25519 | ed25519-dalek | Fast, deterministic signing for individual user actions (posting, updating). |
| Threshold Signatures | BLS12-381 | blst or threshold\_crypto | Allows aggregation of council votes into a single verifiable signature; short signature size. |
| Key Wrapping | KW-AES-256 | aes-kw | RFC 3394 compliant key wrapping for secure envelope transmission. |

## **3\. Account & Key Management**

### **3.1. Recovery Key (Master Authority)**

* Format is a 32-byte raw scalar.  
* Generated via CSPRNG or derived from a high-entropy user passphrase using Argon2id (parameters: t=2, m=64MB, p=4).  
* It is never used to sign content directly. It is used solely to sign "Delegation Certificates" which authorize session keys.

### **3.2. Identity Keys (Ed25519)**

* Users generate an Ed25519 keypair derived from the Master Secret.  
* The Public Key (ID\_PUB) serves as the permanent User ID.

## **4\. Payload Encryption Pipeline**

### **4.1. Per-Post Encryption**

1. A fresh 32-byte symmetric key (DEK \- Data Encryption Key) is generated for every post.  
2. A random 24-byte nonce is generated.  
3. Content is encrypted: Ciphertext \= XChaCha20Poly1305(Key=DEK, Nonce=Nonce, AAD=ManifestHeader, Plaintext=Content).  
4. AAD (Additional Authenticated Data) must include the Manifest Header to bind the ciphertext to its metadata.

### **4.2. Envelope Format (Recipient Wrapping)**

To share content, the DEK is wrapped for specific recipients (or a public group key).

struct Envelope {  
    recipient\_id: \[u8; 32\], // Ed25519 Public Key of recipient  
    expiry: u64,            // Unix Timestamp (seconds)  
    wrapped\_key: Vec\<u8\>,   // DEK encrypted with recipient's public key (ECIES or similar)  
}

## **5\. Governance & Threshold Cryptography**

### **5.1. Council Keys**

* Each Council Member generates a BLS12-381 keypair.  
* A Distributed Key Generation (DKG) ceremony establishes the shared public key for the Council epoch.

### **5.2. Voting & Ratification**

* A hash H(Proposal) is created for the proposal.  
* Council members sign H(Proposal) with their private key share.  
* Once t (threshold) signatures are collected, they are aggregated into a single Signature\_Council.  
* Clients verify Signature\_Council against the Council's Group Public Key.

### **5.3. Revocation Lists (CRLs)**

* Structure is a Bloom filter or compressed bitset containing hashes of revoked EnvelopeIDs or AccountIDs.  
* Must be signed by the current Council's Threshold Signature.  
* Clients attempting to decrypt an Envelope must first check if the EnvelopeID or Author's AccountID is present in the latest signed CRL.

## **6\. Manifest Structure (Canonical Serialization)**

Manifests are serialized using CBOR (Compact Binary Object Representation) for determinism and compactness.

Manifest \= {  
  "v": 1,                       // Version  
  "auth\_id": \<Ed25519\_Pub\>,     // Author  
  "sig": \<Ed25519\_Sig\>,         // Signature of this manifest  
  "content\_hash": \<BLAKE3\>,     // Hash of ciphertext  
  "size": \<u64\>,                // Size in bytes  
  "algo": "XChaCha20Poly1305",  // Encryption Algo  
  "c\_license": \<String\>,        // License SPDX (e.g., "CC-BY-4.0")  
  "gov\_flags": \<u8\>             // Bitmask for content warnings/flags  
}  
