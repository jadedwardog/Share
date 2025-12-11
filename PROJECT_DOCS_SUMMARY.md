# Project documentation and development summary

I maintain this repository and use this file to record the current status of project documentation and development.

Last updated: 2025-12-11

## Intent of the Markdown files (as I use them)

- `README.md`: My high-level project overview and a map to the key documents, plus top-level governance and licensing notes.
- `TECHNICAL_SPEC.md`: The detailed technical specification I rely on for account lifecycle, encryption pipeline, client behavior, governance flows, and enforcement constraints.
- `USER_SPECIFICATION.md`: Rules I expect users to follow (recovery key responsibilities, posting model, license choices, following, appeals, privacy and bandwidth expectations).
- `PAYLOAD_SPECIFICATION.md`: The payload and manifest format I implement: chunking, compression, per-post encryption, envelope format, summaries, and bandwidth/encoding rules.
- `PROJECT_GROUNDWORK.md`: My implementation plan and workspace layout; it documents responsibilities per crate, recommended Rust crates, example snippets, and immediate next steps.
- `CLIENT_IMPLEMENTATION_GUIDE.md`: Client-level guidance I provide for revocation handling, selective sync, privacy enforcement, and bandwidth optimizations.
- `CRYPTO_PROTOCOL_SPEC.md`: Placeholder for the full cryptographic protocol (AEAD selection, KDFs, envelope formats, revocation encoding, threshold signatures). I need to complete this before finalizing implementations.
- `GOVERNANCE_CHARTER.md`: The operational governance rules I enforce (council size/quorum, elections, dissolution, fork ratification, blacklisting, and auditability).
- `GOVERNANCE_CONSTITUTION.md`: Placeholder for the constitutional text that formalizes council procedures and appeals; this must be completed to anchor the charter.
- `MANIFESTO.md`: Placeholder for the project's guiding principles (decentralization, cryptographic enforcement, constitutional governance); I intend to author this next.
- `CONTENT_LICENSE.md`: How I expect user content to be licensed and how manifests should include license metadata.
- `CONTRIBUTING.md`: The contribution process I require, including governance-impacting PRs and ratification paths.
- `LICENSE.md`: Platform code licensing (AGPL-3.0) and notes about how governance and fork recognition interact with licensing.

## Development I've performed so far

- I set up the repository as a Cargo workspace with separate crates for the main concerns: `client/`, `core/`, `crypto/`, `governance/`, and `runtime/` (each contains a `Cargo.toml` and `src/`).
- The following source files and modules are present and serve as the current codebase surface area:
  - `client/src/main.rs`, `client/src/lib.rs`
  - `core/src/lib.rs`, `core/src/account.rs`, `core/src/payload.rs`
  - `crypto/src/lib.rs`, `crypto/src/encryption.rs`, `crypto/src/hashing.rs`
  - `governance/src/lib.rs`, `governance/src/council.rs`, `governance/src/vote.rs`
  - `runtime/src/lib.rs`, `runtime/src/pallets/governance/` (runtime pallet scaffolding)
- I have built the workspace locally in the past; `target/debug/` and a populated build cache are present.
- `PROJECT_GROUNDWORK.md` contains the concrete scaffold and my recommended development order: implement crypto primitives first (AEAD, hashing, KDFs, threshold signatures), then governance logic, then Substrate runtime integration, and finally client enforcement and selective sync.

## Current documentation gaps and priorities

- The highest-priority missing documentation items are:
  1. `CRYPTO_PROTOCOL_SPEC.md` — finalize algorithm choices, parameters, envelope and revocation formats, and threshold signature design.
  2. `GOVERNANCE_CONSTITUTION.md` — draft the constitutional-level text that the charter references.
  3. `MANIFESTO.md` — articulate the project's guiding principles so governance and technical choices map back to them.

- I consider these three documents required before I finalize any public release or a stable runtime integration.

## Concrete next steps I will take

1. Align `Cargo.toml` dependencies in the workspace with the recommendations in `PROJECT_GROUNDWORK.md` (e.g., `blake3`, `chacha20poly1305`, `threshold_crypto`) and add minimal integration tests for the crypto primitives.