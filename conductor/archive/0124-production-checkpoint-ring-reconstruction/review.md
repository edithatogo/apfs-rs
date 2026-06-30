# Review: Production checkpoint ring reconstruction

## Implementation status

- Track: `0124-production-checkpoint-ring-reconstruction`.
- Capability: `M-124`.
- Metadata status: `implemented`.
- Spec and plan are present in this Conductor track.
- Codev review: `codev/reviews/124-production-checkpoint-ring-reconstruction-review.md`.

## Fixes applied

- Checkpoint-map selection now prefers the newest valid checkpoint map within the scanned ring instead of the first valid map encountered.
- The resolver now stops on the first exact match in recency order, so later exact mappings no longer overwrite the selected container OMAP or tree root.
- Regression coverage now exercises the recency-ordered checkpoint ring path in `crates/apfs-core/src/lib.rs`.

## Archive closeout

- Review status: `reviewed`.
- Archive status: `archived`.
- Safety: this closeout does not add APFS media writes, raw physical-device writes, mount-write lifecycle, encryption bypass, unsafe code, or production APFS compatibility claims.
- Evidence boundary: the implemented behavior is read-only checkpoint reconstruction only; unsupported write paths remain out of scope.
