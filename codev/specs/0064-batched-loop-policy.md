# Spec M-064: Batched loop policy and local stop criteria

Document version: 0.22.0
Status: Implemented scaffold
Codev phase: Specify

## Goal

Add batched loop policy and local stop criteria as a safe local handoff improvement.

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
