# Review M-125: Production object-map B-tree traversal

## Status

`implemented`.

## Notes

Checkpoint-bounded object-map traversal is now implemented for the selected checkpoint-selected OMAP root and its mapped child leaf nodes. The resolver path now excludes newer valid maps that sit outside the selected checkpoint xid, and the regression in `crates/apfs-core/src/lib.rs` proves the selected leaf block is taken from the correct checkpoint generation.
