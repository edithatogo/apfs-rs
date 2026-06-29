# Spec 083: Cargoless Rust static linter

Document version: 0.26.0
Status: Implemented scaffold
Codev phase: Specify

## Goal

Implement current-environment handoff/control support for Cargoless Rust static linter.

## Non-goals

- No APFS media writes.
- No raw physical-device access.
- No encryption bypass.
- No repair or format.
- No claim that cargoless checks replace Cargo, macOS, or Windows validation.

## Acceptance

- Tool/report exists.
- Conductor track exists.
- Capability and safety gate are registered.
- Cargoless validation passes.
