# Spec M-139: cargo-vet and dependency policy enforcement maturation

Status: `implemented_scaffold`.

## Objective

Make dependency additions and transitive risk changes fail closed before release.

## Required safety gates

- Preserve read-only default unless this is an accepted future write-governance track.
- No physical-device writes.
- No encryption bypass, password recovery, repair, or format implementation outside explicitly accepted future specs.
- Evidence must distinguish configured gates from executed gates.
