# APFS-RS Product Guidelines

Conductor version: 0.13.0

## Language and user-facing guidance

- Be explicit about what is implemented, experimental, synthetic-only, and unsupported.
- Never say simply “supports APFS” without feature and fixture qualifiers.
- Prefer “refused for safety” over ambiguous failure messages.
- For JSON output, include stable schema versions and structured diagnostics.

## Safety copy

Use these phrases consistently:

- “Read-only by default.”
- “No physical-device write support.”
- “Synthetic parser-development fixture, not a complete APFS filesystem.”
- “Checksum bytes parsed and validated where implemented.”
- “General APFS B-tree traversal is not implemented yet.”

## Release notes

Every release note must include:

- Implemented capabilities.
- Unsupported capabilities.
- Fixture coverage.
- Safety posture.
- Commands to run.
- Validation limitations.


## Current ledger note

The Conductor history now spans M-001 through M-021, including synthetic stat, safe extract-preview, and precompile static validation.
