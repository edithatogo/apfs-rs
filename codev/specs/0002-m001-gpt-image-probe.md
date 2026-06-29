# Spec 0002: GPT-Wrapped APFS Image Probe

Document version: 0.2.0  
Status: Implementing  
Codev phase: Specify

## Goal

Extend `apfs inspect --json` so it can inspect both direct APFS container images and whole-disk GPT images that contain an APFS partition type entry.

## Non-goals

- Raw physical disk access.
- Checkpoint selection.
- Volume enumeration.
- Mounting.
- Extraction.
- Write support.

## Acceptance

- Direct synthetic `NXSB` fixture is detected at offset zero.
- GPT synthetic image is parsed using a 512-byte-sector GPT assumption.
- APFS partition type GUID is detected.
- APFS container superblock is parsed at the partition first LBA.
- JSON includes layout, APFS offset, GPT header, partitions, and warnings.
