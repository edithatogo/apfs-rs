# Review: Production object-map B-tree traversal

## Implementation status

- Track: `0125-production-omap-btree-traversal`.
- Capability: `M-125`.
- Metadata status: `implemented`.
- Spec and plan are present in this Conductor track.
- Codev review: `codev/reviews/125-production-omap-btree-traversal-review.md`.

## Fixes applied

- Leaf traversal is now checkpoint-bounded: decoded leaf nodes are selected only from checkpoint maps at or below the selected OMAP checkpoint xid.
- The resolver regression now proves a newer valid checkpoint map cannot displace the selected child leaf when the older checkpoint map owns the chosen OMAP root.
- A focused regression in `crates/apfs-core/src/lib.rs` exercises the resolver path directly and verifies the selected leaf block and resolved physical address.

## Archive closeout

- Review status: `reviewed`.
- Archive status: `archived`.
- Safety: this closeout does not add APFS media writes, raw physical-device writes, mount-write lifecycle, encryption bypass, unsafe code, or production APFS compatibility claims.
- Evidence boundary: traversal remains read-only and bounded; unsupported general multi-level APFS traversal is still outside this slice.
