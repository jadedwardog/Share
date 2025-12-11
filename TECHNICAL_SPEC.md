# Technical Specification

Defines account lifecycle, content encryption pipeline, client behavior, governance flows, and enforcement. The protocol biases toward verifiable state, minimal metadata, and bandwidth-aware operations.

- Account creation generates a recovery keypair. The recovery key is the sole authority for posting.
- Authentication requires possession of the recovery key. Clients remain authorized until a key change is requested.
- Key change only possible if authenticated with the current recovery key.
- Loss of recovery key results in permanent loss of account access. Accounts become inert but remain on-chain.
- Accounts can be blacklisted by council decision, community majority request, or quorum failure. Blacklisting procedures are defined in the governance charter. Blacklisted accounts cannot post, and their content becomes inaccessible.
- All content encrypted per-post using AEAD with unique symmetric keys.
- Ciphertext immutable and addressed by content hashes.
- Decryption requires valid, non-revoked envelopes tied to active accounts.
- Councils publish signed revocation lists. Clients purge keys and refuse decryption for blacklisted accounts. Revocation procedures are defined in the governance charter.
- Summaries are lightweight manifests containing titles, thumbnails, and digests. They remain visible even if full content is revoked.
- Technical limitations include content size caps, mandatory compression formats, deterministic chunking, and per-post encryption.
- Councils are elected by the community and distributed regionally, as defined in the governance charter.
- Council members serve a maximum of two four-year terms.
- Councils dissolve automatically if quorum is not met, by majority council vote, or by majority community request. Dissolution rules are defined in the governance charter. Dissolution triggers new elections.
- Councils enforce global guidelines defined in the manifesto.
- Councils may propose supplemental guidelines tailored to regional requirements. Supplemental guidelines are subject to community vote before adoption, as defined in the governance charter.
- Governance actions are transparent, logged on-chain with threshold signatures, and enforced by clients.
- Clients validate manifests, governance flags, and revocation lists before decryption.
- Clients refuse to render or propagate content from blacklisted accounts.
- Clients support selective sync, header-only modes, and on-demand retrieval to minimize bandwidth usage.
- Users can follow specific identities and receive summaries of new content. Full content is fetched only when explicitly requested.
- Clients enforce privacy rules, refusing to seed or render content containing unauthorized PII.
- Envelope encryption ensures each post has a unique symmetric key.
- Keys wrapped for recipients via public key envelopes with explicit expiry.
- Forward secrecy and envelope expiry ensure revoked content becomes unreadable.
- Blacklisting invalidates all envelopes from an account, making historical posts inaccessible.
- Honest clients cannot recover content from blacklisted accounts.
- All governance actions, council votes, dissolutions, blacklists, and revocations logged on-chain. Threshold signatures ensure decisions are legitimate and verifiable. Clients enforce these decisions automatically.
- Clients designed to minimize bandwidth usage for users under ISP data caps.
- Summaries provide lightweight visibility without requiring full content downloads.
- On-demand retrieval ensures users only consume bandwidth when explicitly viewing content.
- Regional restrictions prevent unnecessary downloads of inaccessible content.
- Forks or derivative works of the platform code are only recognized if approved by a council of at least ten members and ratified by community vote, as defined in the governance charter. Clients refuse to connect to non-ratified forks.