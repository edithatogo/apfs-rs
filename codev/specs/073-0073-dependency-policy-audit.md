# Spec M-073: Dependency Policy Audit

Document version: 0.25.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Inventory dependencies and high-risk dependency categories before local cargo tooling.

## Non-goals

- No APFS media writes.
- No raw physical-device access.
- No claim that cargoless checks replace Rust, macOS APFS, or Windows WinFsp validation.

## Acceptance

- Tool exists under `tools/`.
- Tool writes JSON and/or Markdown evidence.
- Conductor track is present.
- Capability and safety-gate registries are updated.
