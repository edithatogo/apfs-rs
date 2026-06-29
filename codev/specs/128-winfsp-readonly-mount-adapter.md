# Spec M-128: Windows WinFsp read-only mount adapter and packaging

Status: `planned_roadmap`.

## Objective

Deliver Windows-first read-only mount experience over the validated VFS facade.

## Required safety gates

- Preserve read-only default unless this is an accepted future write-governance track.
- No physical-device writes.
- No encryption bypass, password recovery, repair, or format implementation outside explicitly accepted future specs.
- Evidence must distinguish configured gates from executed gates.
