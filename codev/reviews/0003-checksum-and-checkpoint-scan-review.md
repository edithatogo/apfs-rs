# Review 0003: Checksum Validation and Checkpoint Candidate Scan

Document version: 0.3.0  
Status: Implementation review  
Codev phase: Review

## What changed

- Added APFS Fletcher-64 checksum calculation and validation.
- Added GPT CRC32 validation for header and partition entries.
- Added refusal on APFS container superblock checksum mismatch.
- Added preliminary bounded checkpoint descriptor-area NXSB candidate scan.
- Added synthetic checkpoint-ring fixture generation.

## What remains incomplete

- Checkpoint map parsing.
- Ring-buffer wrap semantics beyond simple bounded linear scan.
- Latest checkpoint selection using checkpoint-map blocks.
- Real macOS-generated fixture validation.

## Safety result

No write support or raw-device access was added. The scan is bounded by `MAX_CHECKPOINT_SCAN_BLOCKS`.
