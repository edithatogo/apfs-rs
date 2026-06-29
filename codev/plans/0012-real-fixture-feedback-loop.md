# Plan 0012: Real-Fixture Feedback Loop

Document version: 0.12.0  
Status: Implemented scaffold  
Codev phase: Plan

## Tasks

1. Add root requirements file with MoSCoW priorities.
2. Add root design file with Mermaid diagrams.
3. Add remaining-elements ledger.
4. Add feedback script for inspect-vs-manifest comparison.
5. Add `xtask real-fixture-feedback` command.
6. Add Conductor track for M-012.
7. Update Conductor history and conductor-check expected tracks.
8. Update Codev capability and safety registries.
9. Update changelog, README, runbook, and implementation status.

## Safety gates

- `read_only_default`.
- `macos_fixture_no_personal_data`.
- `real_fixture_feedback_redacted`.
- `real_fixture_not_production_claim`.

## Done when

The package can produce a feedback report from fixture manifest and inspect output files once the first real APFS fixture is generated on macOS.
