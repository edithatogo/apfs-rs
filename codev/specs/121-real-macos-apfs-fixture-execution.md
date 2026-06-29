# Spec M-121: Real macOS APFS fixture execution

Status: `planned_roadmap`.

## Objective

Produce real fixture evidence for container, checkpoint, OMAP, and filesystem parser calibration.

## Required safety gates

- Preserve read-only default unless this is an accepted future write-governance track.
- No physical-device writes.
- No encryption bypass, password recovery, repair, or format implementation outside explicitly accepted future specs.
- Evidence must distinguish configured gates from executed gates.
