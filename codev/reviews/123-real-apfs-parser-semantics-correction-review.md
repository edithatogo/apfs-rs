# Review M-123: Real APFS parser semantics correction

## Status

`implemented`.

## Notes

This track now has executed evidence: the APFS Fletcher-64 checksum semantics were corrected so the real macOS-generated NXSB validates, and the converted raw image advances into checkpoint scanning. The remaining checkpoint/OMAP/B-tree/filesystem calibration work stays in the later roadmap tracks.
