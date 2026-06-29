# Review 0006: Single-Node OMAP Lookup

Document version: 0.8.0  
Status: Package review  
Codev phase: Review

## What changed

Added the first object lookup API over decoded synthetic OMAP records:

- `OmapLookup` result type.
- `lookup_omap_record` helper.
- Lookup samples in the inspect report.
- `lookup_object_in_device` and `lookup_object_in_bytes` in `apfs-core`.
- `apfs lookup-object --json` command.
- `synthetic-omap-lookup.img` fixture.

## Safety result

The lookup is read-only and bounded to decoded records from one synthetic B-tree root/leaf node. It does not traverse arbitrary APFS B-trees yet and does not write to source media.

## Limitations

This is not a production OMAP lookup. It does not walk internal B-tree nodes, does not handle multiple leaf nodes, and has not been validated against a real macOS-generated APFS image.

## Next step

Implement general B-tree search scaffolding: compare keys, select child pointers from internal nodes, and descend to the target leaf with depth and cycle guards.
