# Spec M-074: Backlog Issue Export

Document version: 0.25.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Export remaining blockers as local reviewable issue stubs.

## Non-goals

- No APFS media writes.
- No raw physical-device access.
- No claim that cargoless checks replace Rust, macOS APFS, or Windows WinFsp validation.

## Acceptance

- Tool exists under `tools/`.
- Tool writes JSON and/or Markdown evidence.
- Conductor track is present.
- Capability and safety-gate registries are updated.
