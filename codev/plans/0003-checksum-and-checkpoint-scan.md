# Plan 0003: Checksum Validation and Checkpoint Candidate Scan

Document version: 0.3.0  
Status: Implementing  
Codev phase: Plan

## Tasks

1. Implement APFS Fletcher-64 checksum calculation.
2. Add checksum fields to `ContainerSuperblock` JSON.
3. Refuse checksum mismatch.
4. Add GPT header and partition-entry CRC32 validation.
5. Update synthetic fixtures to include valid object checksums and GPT CRCs.
6. Add a synthetic checkpoint descriptor-area fixture.
7. Add bounded checkpoint candidate scan.
8. Update Codev registries and reviews.

## Safety gates

- `read_only_default`.
- `bounds_checked_reads`.
- `checksum_refusal`.
- `scan_limit`.
