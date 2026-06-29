# Spec M-072: Golden Output Expectations

Document version: 0.25.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Generate cargoless expected status outputs for synthetic fixtures.

## Non-goals

- No APFS media writes.
- No raw physical-device access.
- No claim that cargoless checks replace Rust, macOS APFS, or Windows WinFsp validation.

## Acceptance

- Tool exists under `tools/`.
- Tool writes JSON and/or Markdown evidence.
- Conductor track is present.
- Capability and safety-gate registries are updated.
