# Review 0013: Feedback-to-Track Promotion

Document version: 0.13.0  
Status: Implemented scaffold review  
Codev phase: Review

## What changed

Added a read-only task-promotion tool that turns real-fixture feedback JSON into Codev and Conductor task stubs.

## Evidence

- `tools/promote_feedback_to_tracks.py` added.
- `cargo xtask promote-feedback <feedback.json> <out-dir>` command added.
- Conductor track `0013-feedback-to-conductor-tracks` added.
- Capability registry and safety gates updated.

## Safety result

The tool reads JSON artifacts only and writes generated planning/task files only to a caller-selected output directory. It does not open, mount, decrypt, repair, format, or write APFS media.

## Follow-up

When a real APFS fixture is generated on macOS, run feedback and promotion together, then manually review generated tasks before copying any generated stubs into the source tree.
