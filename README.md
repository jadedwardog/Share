# Share
Open source decentralised sharing platform

# Developer manifesto for a decentralized, governance-enforced, cryptographically secure content platform

This manifesto defines the principles and enforceable rules of a decentralized platform where governance is constitutional, cryptography is mandatory, privacy is preserved by default, and bandwidth costs are minimized through selective sync. It prioritizes protocol-level guarantees over policy text.

- No central authority or administrative override. All state is validated by clients; enforcement emerges from protocol rules and cryptographic signatures.
- Accounts contain no personally identifiable information unless users explicitly publish it as content. Identity is a public key (or DID) and signatures.
- The recovery key generated at account creation is the sole posting authority. Loss of this key renders the account inert. Key changes are only possible while authenticated; there is no secondary recovery.
- Councils can blacklist accounts and revoke content envelopes per constitutional rules. Honest clients refuse to render or decrypt blacklisted or revoked content while preserving on-chain immutability.
- Every post and chunk are encrypted with per-item AEAD keys, addressed by content hashes, and only decryptable via non-revoked envelopes bound to active accounts.
- Access envelopes expire and must be reissued under current policy, constraining residual access to old content after governance changes.
- Users follow chosen identities. Clients sync headers/manifests and optional summaries; full payloads are fetched on demand to respect ISP caps and metered connections.
- Councils are elected regionally, enforce global guidelines, and propose supplemental regional guidelines. Supplemental guidelines are ratified by majority community vote before adoption.
- Councils dissolve by quorum failure, majority council vote, or majority community request. Dissolution triggers new elections; term limits are strictly enforced.
- All governance actions (votes, dissolutions, blacklists, revocations) are on-chain with threshold signatures and reason codes. Auditability is non-negotiable.
- Mandatory compression, deterministic chunking, size caps, and manifest validation are enforced by clients and consensus rules to prevent abuse and ensure predictable resource usage.
- Data remains on-chain. Governance + cryptography ensure blacklisted accounts’ historical content is practically inaccessible to honest clients.
- Platform code is protected under strong copyleft; user content is licensed independently, allowing commercial use at the user’s discretion.
