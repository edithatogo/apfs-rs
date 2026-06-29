# Spec 0040: Version consistency checker

Document version: 0.19.0  
Status: Implemented Python/tooling

## Goal

Adds a cargoless version consistency check and generated VERSION_CONSISTENCY.md/json snapshot.

## Acceptance

- `tools/version_consistency_check.py` validates current version markers.
- `VERSION_CONSISTENCY.md/json` are generated.
- Release preflight includes the check.
