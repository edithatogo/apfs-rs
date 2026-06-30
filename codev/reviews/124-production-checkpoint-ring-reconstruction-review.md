# Review M-124: Production checkpoint ring reconstruction

## Status

`implemented`.

## Notes

Checkpoint-ring reconstruction now prefers the newest valid checkpoint map in the scanned ring, and later exact mappings no longer overwrite that choice. The regression coverage in `crates/apfs-core/src/lib.rs` exercises the recency-ordered checkpoint window and confirms the container OMAP comes from the latest valid checkpoint map rather than the first valid one encountered.
