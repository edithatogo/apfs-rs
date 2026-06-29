# Spec 0020-m019-synthetic-stat-cli: M-019 Synthetic Stat CLI

Document version: 0.16.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Report metadata for one synthetic directory entry without implementing production APFS inode/stat decoding.

## Non-goals

- Production APFS semantics.
- Raw physical-device access.
- APFS write support.
- Mounting, repair, format, compression, or encryption.

## Safety

Run over synthetic directory fixtures only; no APFS media writes.
