# Spec M-136: Long-running fuzz property mutation coverage hardening

Status: `implemented_scaffold`.

## Objective

Move configured hardening from scaffold/evidence runs into sustained release gates.

## Required safety gates

- Preserve read-only default unless this is an accepted future write-governance track.
- No physical-device writes.
- No encryption bypass, password recovery, repair, or format implementation outside explicitly accepted future specs.
- Evidence must distinguish configured gates from executed gates.
