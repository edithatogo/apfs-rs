# Spec M-056: Repository manifest generator

Document version: 0.21.0  
Status: implemented/scaffolded  
Codev phase: Specify

## Goal

Improve APFS-RS local handoff quality and validation before the first Rust-enabled compile.

## Non-goals

- APFS media writes.
- Raw physical-device access.
- Encryption bypass.
- Repair or format.
- Claiming production APFS support before local validation.

## Acceptance

- Artifact is present.
- Static/cargoless validation can check it where practical.
- Conductor track and Codev review are updated.
