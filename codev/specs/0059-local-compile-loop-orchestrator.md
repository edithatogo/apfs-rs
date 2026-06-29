# Spec M-059: Local compile loop orchestrator

Document version: 0.22.0
Status: Implemented scaffold
Codev phase: Specify

## Goal

Add local compile loop orchestrator as a safe local handoff improvement.

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
