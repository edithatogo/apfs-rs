# Review M-123: Real APFS parser semantics correction

## Status

`implemented`.

## Notes

This track now has executed evidence: the APFS Fletcher-64 checksum semantics were corrected so the real macOS-generated NXSB validates, and the converted raw image advances into checkpoint scanning. The direct regression coverage lives in `crates/apfs-types/src/lib.rs` (`parses_minimal_nx_superblock_fields_and_checksum`, `detects_checksum_mismatch`, and `apfs_fletcher64_matches_real_superblock_style_checksum`) plus `crates/apfs-core/src/lib.rs` (`detects_apfs_container` and `refuses_bad_checksum`). The remaining checkpoint/OMAP/B-tree/filesystem calibration work stays in the later roadmap tracks.
