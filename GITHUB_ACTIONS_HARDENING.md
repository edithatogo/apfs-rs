# GitHub Actions Hardening

Version: 0.29.0

This repository is configured for a hardened GitHub Actions posture before APFS-RS is run against real user media.

## Required controls

- Default workflow permissions must be read-only unless a job explicitly needs more.
- `pull_request_target` is forbidden unless a dedicated security review accepts it.
- Workflows must avoid interpolating untrusted PR data into shell commands.
- Third-party actions should be pinned to immutable commit SHAs before public release.
- `zizmor` and `actionlint` are planned required checks for workflow security.
- Dependency Review, OpenSSF Scorecard, cargo-deny, cargo-audit, and cargo-vet are part of the supply-chain gate.

## Current status

Configured and audit-scaffolded here; execution requires GitHub Actions or local installation of the tools.
