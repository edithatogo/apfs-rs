# Spec M-122: Real fixture feedback promotion

Status: `implemented`.

## Objective

Turn real APFS evidence into reviewed parser tasks with redacted artifacts.

## Required safety gates

- Preserve read-only default unless this is an accepted future write-governance track.
- No physical-device writes.
- No encryption bypass, password recovery, repair, or format implementation outside explicitly accepted future specs.
- Evidence must distinguish configured gates from executed gates.
