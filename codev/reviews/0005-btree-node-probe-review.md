# Review 0005: B-tree Node Probe

Document version: 0.5.0  
Status: Review  
Codev phase: Review

## What changed

- Added initial APFS B-tree node parsing in `apfs-types`.
- Added B-tree root reporting to the OMAP inspection path in `apfs-core`.
- Added synthetic `synthetic-omap-btree-root.img` fixture.
- Added preliminary OMAP record decoding for synthetic B-tree leaf offsets.
- Added fixture-generation support for synthetic B-tree root nodes.

## Current capability

The implementation can now report that a checkpoint map names the container OMAP and that the OMAP names a mapped B-tree root node. It reports node flags, level, key count, table-space metadata, TOC entries, checksum status, and preliminary OMAP records for the synthetic fixture.

## Still missing

- Interpreting OMAP B-tree keys and values.
- Binary search through B-tree nodes.
- Traversing child nodes.
- Resolving arbitrary object IDs through the object map.

## Safety result

No write support, mount support, raw physical-device access, encryption, or low-level memory-risk code was added.
