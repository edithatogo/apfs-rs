# Spec M-060: Cargoless Cargo workspace audit

Document version: 0.22.0
Status: Implemented scaffold
Codev phase: Specify

## Goal

Add cargoless cargo workspace audit as a safe local handoff improvement.

## Non-goals

- APFS media writes.
- Raw physical-device access.
- Live mounting.
- Encryption bypass.
- Repair or format.

## Acceptance

- Tooling/docs exist.
- Conductor track exists.
- Cargoless validation can exercise the artifact where applicable.
