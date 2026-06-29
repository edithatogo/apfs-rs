# Plan 0004: Checkpoint Map and OMAP Header Probe

Document version: 0.4.0  
Status: Implementing  
Codev phase: Plan

## Tasks

1. Add `checkpoint_mapping_t` parser to `apfs-types`.
2. Add `checkpoint_map_phys_t` parser to `apfs-types`.
3. Add `omap_phys_t` parser to `apfs-types`.
4. Extend checkpoint descriptor scan to collect checkpoint-map blocks.
5. Locate a checkpoint mapping for `nx_omap_oid`.
6. Read the mapped object block read-only.
7. Parse and report the container OMAP header.
8. Add a synthetic fixture with a checkpoint map and OMAP block.
9. Update Codev review/status/changelog.

## Safety gates

- `read_only_default`.
- `bounds_checked_reads`.
- `typed_error_no_panic`.
- `unsafe_without_review`.
