# Plan 0007: Checkpoint-Map-Backed Multi-Node OMAP Lookup

Document version: 0.8.0  
Status: Implementing  
Codev phase: Plan

## Tasks

1. Add report struct for additional mapped OMAP B-tree leaf nodes.
2. Scan valid checkpoint maps for B-tree node mappings other than the root tree OID.
3. Parse each mapped node conservatively and keep only leaf nodes.
4. Decode synthetic OMAP key/value records from each mapped leaf.
5. Aggregate root and additional leaf records for lookup.
6. Add synthetic multi-node OMAP fixture.
7. Update CLI human output, Codev capability registry, safety gates, README, status, and changelog.

## Safety gates

- `read_only_default`.
- `bounds_checked_reads`.
- `typed_error_no_panic`.
- `checkpoint_mapped_lookup_limit`.

## Follow-up

Replace checkpoint-map-backed synthetic aggregation with true B-tree traversal after a macOS-generated APFS fixture is available.
