# Review 0001: Bootstrap Implementation Review

Document version: 0.1.0  
Status: Initial implementation review  
Codev phase: Review

## What changed

Started real Rust implementation scaffolding for M-001:

- Safe workspace.
- Read-only image block device.
- APFS object/header and container-superblock parser for first fields.
- CLI inspect command.
- `xtask` policy checks.

## What is still missing

- Checksum validation.
- Checkpoint selection.
- GPT/APFS partition offset discovery.
- Object maps.
- Volumes.
- B-trees.
- File extraction.
- Windows mounting.

## Safety result

No write support was added. No unsafe code was added.
