# **Project documentation and development summary**

I maintain this repository and use this file to record the current status of project documentation and development.

Last updated: 2025-12-11

## **Intent of the Markdown files (as I use them)**

* README.md: My high-level project overview and a map to the key documents, plus top-level governance and licensing notes.  
* TECHNICAL\_SPEC.md: The detailed technical specification I rely on for account lifecycle, encryption pipeline, client behavior, governance flows, and enforcement constraints.  
* USER\_SPECIFICATION.md: Rules I expect users to follow (recovery key responsibilities, posting model, license choices, following, appeals, privacy and bandwidth expectations).  
* PAYLOAD\_SPECIFICATION.md: The payload and manifest format I implement: chunking, compression, per-post encryption, envelope format, summaries, and bandwidth/encoding rules.  
* PROJECT\_GROUNDWORK.md: My implementation plan and workspace layout; it documents responsibilities per crate, recommended Rust crates, example snippets, and immediate next steps.  
* CLIENT\_IMPLEMENTATION\_GUIDE.md: Client-level guidance I provide for revocation handling, selective sync, privacy enforcement, and bandwidth optimizations.  
* CRYPTO\_PROTOCOL\_SPEC.md: The definitive cryptographic protocol (AEAD selection, KDFs, envelope formats, revocation encoding, threshold signatures) that I enforce.  
* GOVERNANCE\_CHARTER.md: The operational governance rules I enforce (council size/quorum, elections, dissolution, fork ratification, blacklisting, and auditability).  
* GOVERNANCE\_CONSTITUTION.md: The binding constitutional text formalizing council procedures, appeals, and the bootstrap phase.  
* MANIFESTO.md: The project's guiding principles (decentralization, cryptographic enforcement, constitutional governance).  
* CONTENT\_LICENSE.md: How I expect user content to be licensed and how manifests should include license metadata.  
* CONTRIBUTING.md: The contribution process I require, including governance-impacting PRs and ratification paths.  
* LICENSE.md: Platform code licensing (AGPL-3.0) and notes about how governance and fork recognition interact with licensing.

## **Development I've performed so far**

* I set up the repository as a Cargo workspace with separate crates for the main concerns: client/, core/, crypto/, governance/, and runtime/.  
* I have authored the critical missing governance and technical documents: CRYPTO\_PROTOCOL\_SPEC.md, GOVERNANCE\_CONSTITUTION.md, and MANIFESTO.md.  
* I have fully implemented the **crypto** crate, which now contains:  
  * hashing.rs: BLAKE3 content addressing.  
  * encryption.rs: XChaCha20-Poly1305 symmetric encryption with AAD.  
  * kdf.rs: Argon2id for recovery key generation.  
  * signatures.rs: Ed25519 identity signatures.  
  * threshold.rs: BLS12-381 primitives for Council voting.  
  * envelopes.rs: X25519 \+ AES-KW for secure key exchange.

## **Current documentation gaps and priorities**

* The documentation suite is now complete for Phase 1\.  
* My priority has shifted from specification to implementation of the core logic that utilizes the now-ready crypto primitives.

## **Concrete next steps I will take**

1. Implement the core crate structures (Account, Manifest, Chunk) utilizing the crypto library.  
2. Define the serialization formats (CBOR) for these structures to ensure deterministic canonicalization.  
3. Build unit tests in core to verify that Manifest generation and Account creation flows work as expected before moving to the governance logic.