# Spec 0038: Snapshot and volume-role readiness

Document version: 0.19.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Adds snapshot/role readiness reports without snapshot mutation or production views.

## Non-goals

- Production APFS support for this feature.
- APFS media writes.
- Mounting, repair, format, decryption, or access-control bypass.

## Acceptance

- CLI/readiness report is available.
- Output is JSON serializable.
- Safety constraints are explicit.
- Conductor track is present.
