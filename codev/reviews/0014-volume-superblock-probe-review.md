# Review 0013: Synthetic Volume Superblock Probe

Document version: 0.13.0  
Status: Implementation review  
Date: 2026-06-24  
Codev phase: Review

## What changed

- Added synthetic APSB-like volume-superblock parser.
- Added volume report envelope/probe types.
- Added `apfs volumes --json`.
- Added `synthetic-volume-superblock.img`.
- Added Conductor track `0013-volume-superblock-probe`.

## Safety result

No write support, mount code, raw physical-device access, encryption, repair, or format functionality was added. The new path reads image bytes only and is bounded by the existing object-map resolver facade.

## Remaining work

- Compile and test on a Rust-enabled machine.
- Validate volume-superblock offsets against a real macOS APFS fixture.
- Use the parsed volume root tree OID to begin filesystem tree parsing.
