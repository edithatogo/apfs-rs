# Review 0008: Bounded Synthetic B-tree Traversal

Document version: 0.8.0  
Status: Implementation review  
Codev phase: Review

## What changed

Added a bounded synthetic two-level OMAP B-tree traversal layer:

- root/index record parsing from non-leaf B-tree nodes;
- child selection by `(max_oid, max_xid)` range records;
- selected leaf decoding;
- traversal details included in `lookup-object --json`;
- synthetic bounded traversal fixture.

## What this enables

The lookup path can now model the next APFS object-map step more realistically than checkpoint-map leaf aggregation. It selects the leaf that should contain the requested object instead of searching every decoded leaf.

## Limitations

This is still synthetic and bounded. It is not full APFS B-tree traversal and should not yet be claimed to work against production APFS images.

## Safety result

No write, raw-device, mount, encryption, repair, or format code was added.
