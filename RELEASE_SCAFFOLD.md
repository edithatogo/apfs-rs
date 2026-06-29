# Release, Provenance, and Packaging Scaffold

Version: 0.20.0

## Current state

This is release-readiness scaffolding, not a signed production release.

## Planned release artifacts

- Windows portable ZIP.
- Windows installer after read-only MVP.
- SBOM.
- SHA-256 checksums.
- GitHub artifact attestations.
- SLSA provenance where practical.
- Winget manifest after a stable read-only release.

## Required before public binaries

```bash
cargo test --workspace
cargo xtask release-preflight --write-manifest
cargo dist plan   # once cargo-dist is adopted
```

## Non-goals before MVP

- Signed write-capable binaries.
- Repair or format releases.
- Encrypted volume support claims.

## WinFsp packaging note

Windows read-only mount releases must document the WinFsp runtime dependency and must not bundle or require it until the installer/licence approach is reviewed.

## v0.21.0 local handoff checks

Release and handoff checks now include config sanity, local environment doctor output, known-risk ledger, and repository manifest generation. These are pre-release guardrails, not a production release process.
