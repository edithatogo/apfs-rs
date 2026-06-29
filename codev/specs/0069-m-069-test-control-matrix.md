# Spec 0069: Test/control matrix generator

Document version: 0.23.0
Status: Implemented scaffold
Codev phase: Specify

## Goal

Map validation commands to current, Rust, macOS, and Windows phases.

## Non-goals

- No APFS media writes.
- No raw physical-device access.
- No claim that cargoless checks replace `cargo test`.

## Acceptance

- Tooling exists and can run in the current sandbox.
- Capability and Conductor history are updated.
- Output artifacts are regenerated for handoff.
