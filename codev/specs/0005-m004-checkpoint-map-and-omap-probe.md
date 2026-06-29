# Spec 0005: M-004 Checkpoint Map and Container Object-Map Header Probe

Document version: 0.4.0  
Status: Implementing  
Codev phase: Specify

## Goal

Extend read-only inspection from checkpoint `NXSB` candidate scanning to parsing checkpoint-map blocks and resolving the container object-map header when a valid checkpoint map provides a mapping for `nx_omap_oid`.

## Scope

- Parse `checkpoint_mapping_t` entries.
- Parse `checkpoint_map_phys_t` blocks.
- Validate APFS object checksums for checkpoint-map blocks.
- Report checkpoint maps and mappings in `inspect --json`.
- Parse `omap_phys_t` header fields for the container object map when a mapping is available.

## Non-goals

- B-tree node parsing.
- Full object-map lookup.
- Volume enumeration.
- Directory listing.
- File extraction.
- Write support.

## Acceptance

- Synthetic checkpoint-map/object-map fixture is generated.
- `inspect --json` reports at least one checkpoint map and one OMAP header for that fixture.
- Invalid or absent checkpoint maps produce diagnostic notes, not panics.
- No raw-device access and no write support are added.
