# Plan 0014: M-014 Resolver-Backed Mapped Object Read Report

Document version: 0.15.0  
Status: Implemented in package, uncompiled  
Date: 2026-06-24  
Codev phase: Plan

## Tasks

1. Add mapped-object read report types.
2. Add `read_mapped_object_in_device` and `read_mapped_object_in_bytes`.
3. Route lookups through the existing object-map resolver facade.
4. Read the resolved physical block with bounded read-only block access.
5. Parse generic APFS object header and validate Fletcher checksum.
6. Add CLI command `apfs read-object --json`.
7. Add synthetic fixture `synthetic-mapped-object-read.img`.
8. Update Codev and Conductor histories.

## Safety gates

- `read_only_default`.
- `bounds_checked_reads`.
- `typed_error_no_panic`.
- `object_map_resolver_facade_boundary`.
- `mapped_object_read_limit`.

## Validation

Rust/Cargo are unavailable in this environment, so compilation remains pending. The ZIP, fixture structure, YAML registries, and Conductor track history are validated here.
