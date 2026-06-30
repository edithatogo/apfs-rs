# APFS-RS Tech Stack

Conductor version: 0.13.0

## Language

Rust, safe by default. Core crates use `#![forbid(unsafe_code)]`.

## Workspace crates

- `apfs-types`: endian-safe on-disk types and parsers.
- `apfs-blockdev`: read-only source abstraction.
- `apfs-core`: APFS inspection, checkpoint, OMAP, resolver, and cursor logic.
- `apfs-cli`: CLI commands and JSON output.
- `apfs-vfs`: read-only VFS boundary and traversal guards.
- `apfs-fuse`: FUSE read-only adapter readiness scaffold.
- `apfs-android`: Android read-only adapter readiness scaffold.
- `apfs-crypto`: encryption readiness and policy helpers.
- `apfs-write-lab`: write-lab readiness scaffold.
- `apfs-test`: test helper placeholder.
- `xtask`: repository automation and policy checks.

## Context systems

- Codev SPIR under `codev/`.
- Conductor context-driven development under `conductor/`.
- Agent instructions under `AGENTS.md`, `CLAUDE.md`, `.github/copilot-instructions.md`, `.claude/skills/`, and `.gemini/skills/`.

## Validation tools expected on a development machine

- `cargo fmt`
- `cargo test`
- `cargo clippy`
- `cargo xtask registry-check`
- `cargo xtask safety-check`
- Python 3 for synthetic fixture generation

## Not yet present

- WinFsp production adapter.
- FUSE production adapter.
- Android production adapter.
- Compression crates.
- Write transaction crates.
