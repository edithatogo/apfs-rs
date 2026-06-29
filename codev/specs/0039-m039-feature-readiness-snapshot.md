# Spec 0039: Advanced feature readiness snapshot

Document version: 0.19.0  
Status: Implemented Python/tooling  
Codev phase: Specify

## Goal

Generates FEATURE_READINESS.md/json to summarize advanced read/post-MVP readiness scaffolds before Rust compilation.

## Non-goals

- Claiming production APFS feature support.
- Opening, mounting, decrypting, repairing, formatting, or writing APFS media.

## Acceptance

- `tools/feature_readiness_snapshot.py` writes `FEATURE_READINESS.md` and `FEATURE_READINESS.json`.
- Release preflight runs the snapshot.
- Conductor track is present.
