# Plan 0015: M-015 Synthetic Filesystem Root Tree Records

Document version: 0.15.0  
Status: Implementing  
Codev phase: Plan

## Tasks

1. Add/update synthetic fixture data.
2. Add bounded parser/report/CLI code.
3. Preserve read-only safety gates.
4. Update Codev and Conductor history.
5. Regenerate package manifests.

## Safety gates

- `read_only_default`.
- `bounds_checked_reads`.
- `typed_error_no_panic`.
- `synthetic_only_warning`.
