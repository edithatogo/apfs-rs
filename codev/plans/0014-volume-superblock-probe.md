# Plan 0013: M-013 Synthetic Volume Superblock Probe

Document version: 0.13.0  
Status: Implemented in package, uncompiled  
Date: 2026-06-24  
Codev phase: Plan

## Tasks

1. Add `OBJECT_TYPE_FS` and APSB-like volume-superblock constants.
2. Add a bounded parser for a synthetic APFS volume-superblock field subset.
3. Add `VolumeReportEnvelope` and `VolumeProbeReport`.
4. Add `volume_report_in_device` and `volume_report_in_bytes`.
5. Add CLI command `apfs volumes --json <source>`.
6. Add synthetic fixture `synthetic-volume-superblock.img`.
7. Update Codev and Conductor histories.
8. Update requirements, design, runbook, and remaining-elements ledger.

## Safety gates

- `read_only_default`.
- `bounds_checked_reads`.
- `typed_error_no_panic`.
- `object_map_resolver_facade_boundary`.
- `synthetic_volume_probe_limit`.

## Validation

Rust/Cargo are unavailable in this environment, so compilation remains pending. The ZIP, fixture layout, YAML registries, and Conductor track history are validated here.
