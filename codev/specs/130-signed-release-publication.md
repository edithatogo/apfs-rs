# Spec M-130: Signed release SBOM provenance installer and winget publication

Status: `implemented_scaffold`.

## Objective

Provide the smallest safe release-publication readiness scaffold for a publishable, auditable distribution process.

## Required safety gates

- Preserve read-only default unless this is an accepted future write-governance track.
- No physical-device writes.
- No encryption bypass, password recovery, repair, or format implementation outside explicitly accepted future specs.
- Evidence must distinguish configured gates from executed gates.
