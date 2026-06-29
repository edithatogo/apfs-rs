# Spec M-126: Production filesystem tree decoding and metadata mapping

Status: `planned_roadmap`.

## Objective

Promote synthetic ls/stat scaffolds into evidence-backed production read behavior.

## Required safety gates

- Preserve read-only default unless this is an accepted future write-governance track.
- No physical-device writes.
- No encryption bypass, password recovery, repair, or format implementation outside explicitly accepted future specs.
- Evidence must distinguish configured gates from executed gates.
