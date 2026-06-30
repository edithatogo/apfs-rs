# Spec M-127: Production file extent resolution and extraction

Status: `implemented`.

## Objective

Promote synthetic preview/extract scaffolds to fixture-backed read-only extraction.

## Required safety gates

- Preserve read-only default unless this is an accepted future write-governance track.
- No physical-device writes.
- No encryption bypass, password recovery, repair, or format implementation outside explicitly accepted future specs.
- Evidence must distinguish configured gates from executed gates.
