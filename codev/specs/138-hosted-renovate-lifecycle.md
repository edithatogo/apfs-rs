# Spec M-138: Hosted Renovate lifecycle and dependency update governance

Status: `planned_roadmap`.

## Objective

Complete the migration away from Dependabot and prove update automation is live.

## Required safety gates

- Preserve read-only default unless this is an accepted future write-governance track.
- No physical-device writes.
- No encryption bypass, password recovery, repair, or format implementation outside explicitly accepted future specs.
- Evidence must distinguish configured gates from executed gates.
