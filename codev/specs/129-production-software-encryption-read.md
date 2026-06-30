# Spec M-129: Production software-encryption read support

Status: `implemented_scaffold`.

## Objective

Provide the smallest safe readiness scaffold for legitimate user-supplied unlock material on read-only encrypted APFS images.

## Required safety gates

- Preserve read-only default unless this is an accepted future write-governance track.
- No physical-device writes.
- No encryption bypass, password recovery, repair, or format implementation outside explicitly accepted future specs.
- Evidence must distinguish configured gates from executed gates.
