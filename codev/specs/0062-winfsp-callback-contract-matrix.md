# Spec M-062: WinFsp read-only callback contract matrix

Document version: 0.22.0
Status: Implemented scaffold
Codev phase: Specify

## Goal

Add winfsp read-only callback contract matrix as a safe local handoff improvement.

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
