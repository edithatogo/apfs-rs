# Spec 0014: M-013 Real-Fixture Feedback Promotion to Codev/Conductor Tracks

Document version: 0.13.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Convert `real-fixture-feedback.json` issues into actionable Codev and Conductor task stubs, so the first macOS-generated APFS fixture can drive implementation work without losing context.

## Requirements

- Add `tools/promote_feedback_to_tracks.py`.
- Add `cargo xtask promote-feedback <feedback.json> <out-dir>`.
- Generate Codev spec/plan/review stubs for each blocker/error/warning issue.
- Generate Conductor track directories with `metadata.json`, `spec.md`, and `plan.md`.
- Generate `task-index.json` and `task-index.md`.
- Keep all generated output in the requested output directory.
- Preserve read-only and redacted-artifact safety posture.

## Non-goals

- Automatically modifying the source tree.
- Opening APFS images.
- Mounting APFS.
- Decrypting APFS.
- Repairing, formatting, or writing APFS media.
- Claiming real APFS compatibility.

## Acceptance

- Promotion script exists and is executable.
- `xtask` command exists.
- Conductor historical track exists for this capability.
- Conductor check expects the M-013 track.
- Remaining-elements ledger is updated.
