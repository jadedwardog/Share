# **Payload Specification**

* Content manifests include: content hash, chunk root, type/codec, size class, summary fields, governance flags, and threshold signatures.  
* Payloads are encrypted per-post using AEAD with unique symmetric keys.  
* Chunks are deterministically sized (e.g., 256 KB) and addressed by hash (BLAKE3).  
* Compression is mandatory (e.g., WebP for images, AV1 for video).  
* Summaries include lightweight metadata: title, thumbnail hash, digest.  
* Envelopes wrap per-chunk keys to recipients’ public keys with explicit expiry and revocation linkage.  
* Payloads remain immutable on-chain; revocation lists invalidate envelopes to make content inaccessible.  
* Clients enforce governance legitimacy by refusing to decrypt payloads from blacklisted accounts, as defined in GOVERNANCE\_CHARTER.md.  
* Payload specification ensures bandwidth efficiency through selective sync, summaries, and on-demand retrieval.  
* Payloads must comply with technical limits defined in TECHNICAL\_SPEC.md and governance rules in GOVERNANCE\_CHARTER.md.