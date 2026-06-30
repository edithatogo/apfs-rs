# Spec M-137: Branch protection and required-check governance

Status: `implemented_scaffold`.

## Objective

Make CI policy enforceable at the repository admin layer.

## Required safety gates

- Preserve read-only default unless this is an accepted future write-governance track.
- No physical-device writes.
- No encryption bypass, password recovery, repair, or format implementation outside explicitly accepted future specs.
- Evidence must distinguish configured gates from executed gates.
