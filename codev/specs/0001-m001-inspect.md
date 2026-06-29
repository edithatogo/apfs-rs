# Spec 0001: M-001 Read-Only APFS Inspect Slice

Document version: 0.1.0  
Status: Implementing  
Codev phase: Specify

## Goal

Implement the first safe APFS inspection slice: open an image source read-only, probe block zero for an APFS container superblock, parse basic `nx_superblock_t` fields, and emit structured JSON.

## Non-goals

- Mounting.
- Extraction.
- Encryption.
- Compression.
- Object maps.
- B-trees.
- Write support.

## Acceptance

- `apfs inspect --json <source>` exists.
- Not-APFS input produces a structured `not_apfs` report.
- Short input produces a structured refusal.
- Minimal synthetic NXSB fixture is parsed.
- No unsafe code.
- No raw physical write code.
