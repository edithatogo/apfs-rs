# Plan 0006: Single-Node OMAP Lookup

Document version: 0.8.0  
Status: Implemented in package  
Codev phase: Plan

## Tasks

1. Add an `OmapLookup` result structure.
2. Add a single-node record lookup helper.
3. Attach lookup samples to the OMAP B-tree root report.
4. Add `lookup_object_in_device` and `lookup_object_in_bytes` APIs.
5. Add `apfs lookup-object --json`.
6. Add a synthetic fixture with multiple versions of one object ID.
7. Update capability registry, safety gates, README, status, and review.

## Safety gates

- Read-only default.
- Bounded single-node lookup.
- Typed error/no panic.
- No physical write path.
