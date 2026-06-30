# Spec M-135: APFS format governance and refusal model

Status: `implemented_scaffold`.

## Objective

Ensure format support remains explicitly out of scope until governed separately.

## Required safety gates

- Preserve read-only default unless this is an accepted future write-governance track.
- No physical-device writes.
- No encryption bypass, password recovery, repair, or format implementation outside explicitly accepted future specs.
- Evidence must distinguish configured gates from executed gates.
