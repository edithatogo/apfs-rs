# Spec M-131: Linux macOS ChromeOS Android read-only adapters

Status: `planned_roadmap`.

## Objective

Expand validated read-only access beyond Windows after core parser maturity.

## Required safety gates

- Preserve read-only default unless this is an accepted future write-governance track.
- No physical-device writes.
- No encryption bypass, password recovery, repair, or format implementation outside explicitly accepted future specs.
- Evidence must distinguish configured gates from executed gates.
