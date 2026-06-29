# Spec 0070: Handoff archive audit generator

Document version: 0.23.0
Status: Implemented scaffold
Codev phase: Specify

## Goal

Generate file-level archive audit with sizes and checksums.

## Non-goals

- No APFS media writes.
- No raw physical-device access.
- No claim that cargoless checks replace `cargo test`.

## Acceptance

- Tooling exists and can run in the current sandbox.
- Capability and Conductor history are updated.
- Output artifacts are regenerated for handoff.
