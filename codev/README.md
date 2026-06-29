# APFS-RS Codev Workspace

Document version: 0.18.0  
Status: Active implementation workspace

This implementation workspace is Codev-driven. Every implemented/scaffolded capability has a Codev spec, plan, review, capability-registry entry, and matching Conductor track.

## Implemented/scaffolded capability history

- M-001: APFS image/container detection.
- M-002: GPT-wrapped APFS partition probe.
- M-003: APFS Fletcher-64 checksum validation.
- M-004: checkpoint candidate scan, checkpoint-map parsing, early OMAP probe.
- M-005: B-tree node header/TOC probe and preliminary OMAP records.
- M-006: single-node synthetic OMAP lookup.
- M-007: checkpoint-map-backed multi-node synthetic OMAP lookup.
- M-008: bounded synthetic two-level OMAP traversal.
- M-009: object-map resolver facade.
- M-010: production-shaped B-tree cursor facade.
- M-011: real APFS fixture readiness harness.
- M-012: real-fixture feedback loop.
- M-013: feedback-to-Codev/Conductor promotion.
- M-014: synthetic volume-superblock probe and `apfs volumes`.
- M-015: resolver-backed mapped object read report and `apfs read-object`.
- M-016: synthetic filesystem root-tree directory-record parser.
- M-017: synthetic directory listing CLI and `apfs ls`.
- M-018: synthetic direct-block file preview and `apfs cat`.
- M-019: synthetic metadata/stat report and `apfs stat`.
- M-020: synthetic safe extract-preview CLI and `apfs extract`.
- M-021: Python/static precompile validation harness and `xtask precompile-check`.

## Rules

- Every capability must map to `codev/resources/capabilities.yaml`.
- Every safety-sensitive path must map to `codev/resources/safety-gates.yaml`.
- All runtime APFS operations are read-only.
- Write support requires a future accepted write-lab spec.
- Conductor tracks must preserve the full development history.


## v0.18.0 current ledger

Implementation/support history now covers `M-001` through `M-030`, including VFS, Windows-readiness, diagnostics, fuzz scaffold, API map, and next-loop tooling.


## v0.18.0

Codev now tracks M-031 through M-033 for doctor readiness, CLI/API/source metrics, and safety-case preflight.


## v0.20.0

Added M-041 through M-049 for local handoff, cargo triage, platform setup, release/provenance scaffolding, adapter readiness, encryption readiness, and image-only write-lab readiness.
