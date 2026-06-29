# Review 0007: Checkpoint-Map-Backed Multi-Node OMAP Lookup

Document version: 0.8.0  
Status: Implementation review  
Codev phase: Review

## What changed

- Added reporting for additional checkpoint-map-mapped OMAP B-tree leaf nodes.
- Added aggregation of decoded synthetic OMAP records across root and additional mapped leaves.
- Updated object lookup to search the aggregate record set.
- Added `synthetic-omap-multinode-lookup.img` fixture.
- Updated capability and safety registries.

## Safety result

The implementation remains read-only. It does not add raw physical-device access, mounting, write support, repair, format, encryption bypass, or unsafe code.

## Limitations

This is not yet true APFS B-tree traversal. The additional leaf discovery is checkpoint-map-backed and synthetic-fixture-oriented. Real APFS object-map traversal still requires internal-node interpretation, child resolution, and macOS-generated fixture validation.

## Next action

Implement a proper B-tree traversal abstraction with bounded depth, node-type checks, and child resolution against OMAP/checkpoint mappings.
