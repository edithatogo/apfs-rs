# Spec 0037: Compression readiness

Document version: 0.19.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Adds compression readiness reports for zlib/lzvn/lzfse without enabling decompression.

## Non-goals

- Production APFS support for this feature.
- APFS media writes.
- Mounting, repair, format, decryption, or access-control bypass.

## Acceptance

- CLI/readiness report is available.
- Output is JSON serializable.
- Safety constraints are explicit.
- Conductor track is present.
