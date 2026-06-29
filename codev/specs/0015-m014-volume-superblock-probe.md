# Spec 0014: M-013 Synthetic Volume Superblock Probe

Document version: 0.13.0  
Status: Implemented in package, uncompiled  
Date: 2026-06-24  
Codev phase: Specify

## Goal

Add the first read-only APFS volume-superblock probe. The current implementation resolves container filesystem OIDs through the existing object-map resolver facade and parses a synthetic APSB-like volume superblock block.

## Non-goals

- Production APFS volume enumeration against real macOS images.
- Filesystem root tree traversal.
- Directory listing.
- File extraction.
- Mounting.
- Writes, repair, format, encryption, or compression.

## Acceptance

- `apfs volumes --json fixtures/synthetic-volume-superblock.img` exists.
- Synthetic filesystem OID `1000` resolves through the current OMAP resolver to a volume superblock block.
- Volume report includes name, UUID, role, root tree OID, counts, and checksum status.
- Unresolved volume OIDs produce structured probe errors.
- The feature is marked synthetic-fixture-only until real macOS fixture validation.
