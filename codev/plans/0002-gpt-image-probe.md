# Plan 0002: GPT-Wrapped APFS Image Probe

Document version: 0.2.0  
Status: Implementing  
Codev phase: Plan

## Tasks

1. Add GPT header and partition-entry parsers to `apfs-types`.
2. Add APFS GPT type GUID detection.
3. Add range-read image block device to `apfs-blockdev`.
4. Update `apfs-core` to inspect block zero first, then GPT APFS partition offsets.
5. Update CLI to use read-only image block device.
6. Add synthetic fixture generator for direct and GPT-wrapped probes.
7. Update capability registry and review notes.

## Safety gates

- `read_only_default`.
- `bounds_checked_reads`.
- `typed_error_no_panic`.
- `no_physical_write_path`.
