# Spec 0021-m020-synthetic-safe-extract-cli: M-020 Synthetic Safe Extract CLI

Document version: 0.16.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Write a bounded synthetic direct-block file preview to a host destination directory with path traversal safeguards.

## Non-goals

- Production APFS semantics.
- Raw physical-device access.
- APFS write support.
- Mounting, repair, format, compression, or encryption.

## Safety

Writes only to host output directory; never writes to APFS media; synthetic preview only.
