# Plan 0005: B-tree Node Probe

Document version: 0.5.0  
Status: Implemented  
Codev phase: Plan

## Tasks

1. Add APFS object type constants for B-tree and B-tree node objects.
2. Add `BTreeTableSpace`, `BTreeTocEntry`, and `BTreeNode` data types.
3. Implement safe B-tree node header and TOC parsing.
4. Add bounds checks for table-space offsets and lengths.
5. Update core OMAP report to resolve and parse the OMAP tree root when checkpoint maps provide a mapping.
6. Add preliminary OMAP record decoding for synthetic leaf records.
7. Add synthetic fixture with OMAP and B-tree root objects.
8. Update Codev registries and reviews.

## Safety gates

- `bounds_checked_reads`.
- `typed_error_no_panic`.
- `read_only_default`.
- `no_write_path`.
