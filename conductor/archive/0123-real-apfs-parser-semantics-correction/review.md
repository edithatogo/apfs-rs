# Review: Real APFS parser semantics correction

## Implementation status

- Track: `0123-real-apfs-parser-semantics-correction`.
- Capability: `M-123`.
- Metadata status: `implemented`.
- Spec and plan are present in this Conductor track.
- Codev review: `codev/reviews/123-real-apfs-parser-semantics-correction-review.md`.

## Fixes applied

- Corrected APFS Fletcher-64 checksum semantics so the real macOS-generated NXSB validates with the stored checksum.
- The converted raw disk image now reaches APFS container detection, checkpoint scan, and the real fixture no longer dies at the NXSB checksum boundary.
- Updated the shared synthetic-oracle checksum formula so generator and parser stay aligned.
- Regression coverage now explicitly exercises the positive checksum path, the checksum-mismatch refusal path, and the core inspect boundary in `crates/apfs-types/src/lib.rs` and `crates/apfs-core/src/lib.rs`.

## Archive closeout

- Review status: `reviewed`.
- Archive status: `archived`.
- Safety: this closeout does not add APFS media writes, raw physical-device writes, mount-write lifecycle, encryption bypass, unsafe code, or production APFS compatibility claims.
- Evidence boundary: the checksum-semantics correction is executed; deeper checkpoint/OMAP/B-tree/filesystem coverage remains the next roadmap slice in M-124 onward.
