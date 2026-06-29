# Spec 0034: Unicode and case-sensitivity policy CLI

Document version: 0.19.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Adds apfs path-policy --json and a host-facing Unicode/case-sensitivity policy scaffold.

## Non-goals

- Production APFS support for this feature.
- APFS media writes.
- Mounting, repair, format, decryption, or access-control bypass.

## Acceptance

- CLI/readiness report is available.
- Output is JSON serializable.
- Safety constraints are explicit.
- Conductor track is present.
