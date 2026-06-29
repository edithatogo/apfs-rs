# Spec 0006: M-005 Phase 1 B-tree Node Probe

Document version: 0.5.0  
Status: Implemented  
Codev phase: Specify

## Goal

Add the first read-only APFS B-tree node parser needed before object-map lookup. The parser should read a `btree_node_phys_t`-style object header, node flags, level, key count, table-space metadata, and table-of-contents key/value offsets.

## Non-goals

- Full B-tree traversal.
- Production-grade OMAP key/value interpretation.
- Filesystem tree parsing.
- Volume enumeration.
- Directory listing.
- Write support.

## Acceptance

- B-tree node blocks are parsed with bounds checks.
- Checksum validation is reported.
- Synthetic OMAP B-tree root fixture is generated.
- `inspect --json` reports the OMAP B-tree root when checkpoint mappings identify it.
- Synthetic OMAP leaf records can be decoded from the B-tree root fixture.
- Unsupported or malformed B-tree nodes produce diagnostics, not panics.
