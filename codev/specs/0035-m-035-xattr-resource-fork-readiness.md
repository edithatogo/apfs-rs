# Spec 0035: Xattr and resource fork readiness

Document version: 0.19.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Adds readiness reports for xattrs/resource forks without claiming production extraction.

## Non-goals

- Production APFS support for this feature.
- APFS media writes.
- Mounting, repair, format, decryption, or access-control bypass.

## Acceptance

- CLI/readiness report is available.
- Output is JSON serializable.
- Safety constraints are explicit.
- Conductor track is present.
