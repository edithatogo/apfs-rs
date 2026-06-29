# Plan 0001: Implement M-001 Inspect

Document version: 0.1.0  
Status: Implementing  
Codev phase: Plan

## Tasks

1. Create safe Rust workspace.
2. Add `apfs-types` parser for `obj_phys_t` and the first fields of `nx_superblock_t`.
3. Add read-only image block device.
4. Add `apfs-core` inspect report.
5. Add `apfs-cli inspect --json`.
6. Add `xtask registry-check` and `safety-check`.
7. Add CI templates.
8. Add review.

## Safety gates

- `read_only_default`.
- `bounds_checked_reads`.
- `typed_error_no_panic`.
- `unsafe_without_review`.
