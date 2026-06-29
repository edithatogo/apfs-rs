# Spec 0068: Synthetic negative fixture generator

Document version: 0.23.0
Status: Implemented scaffold
Codev phase: Specify

## Goal

Create safe parser-refusal fixtures from synthetic NXSB images.

## Non-goals

- No APFS media writes.
- No raw physical-device access.
- No claim that cargoless checks replace `cargo test`.

## Acceptance

- Tooling exists and can run in the current sandbox.
- Capability and Conductor history are updated.
- Output artifacts are regenerated for handoff.
