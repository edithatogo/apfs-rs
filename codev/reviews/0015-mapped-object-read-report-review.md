# Review 0014: Resolver-Backed Mapped Object Read Report

Document version: 0.15.0  
Status: Implementation review  
Date: 2026-06-24  
Codev phase: Review

## What changed

- Added mapped-object read report types.
- Added `read_mapped_object_in_device` and `read_mapped_object_in_bytes`.
- Added CLI command `apfs read-object --json`.
- Added `synthetic-mapped-object-read.img` with signed generic object blocks.
- Added capability and safety-gate entries for M-014.
- Added Conductor track `0014-mapped-object-read-report`.

## Safety result

No write support, raw physical-device access, mount code, encryption, repair, or format functionality was added. The new path reads image bytes only and reports object headers/checksums without payload interpretation.

## Remaining work

- Compile and test on a Rust-enabled machine.
- Validate mapped object reads against a real macOS APFS fixture.
- Use this generic read boundary for future volume, filesystem tree, and object payload decoders.
