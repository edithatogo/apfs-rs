# Review 0004: Checkpoint Map and OMAP Header Probe

Document version: 0.4.0  
Status: Initial review  
Codev phase: Review

## What changed

Implemented the next read-only inspection slice:

- Parsed `checkpoint_mapping_t`.
- Parsed `checkpoint_map_phys_t` and checksum status.
- Extended checkpoint descriptor scanning to identify map blocks as well as NXSB candidates.
- Parsed `omap_phys_t` header fields when a checkpoint map points to the container object map.
- Added a synthetic `synthetic-checkpoint-map-omap.img` fixture.

## What remains missing

- Full checkpoint ring reconstruction.
- Object-map B-tree lookup.
- Volume superblock lookup.
- B-tree parsing.
- Directory/file read logic.

## Safety review

The implementation remains read-only. No write APIs, raw physical-device access, or encryption behaviour were added.

## Validation caveat

Rust/Cargo is not available in the packaging environment, so compilation and tests still need to run on a Rust-enabled machine.
