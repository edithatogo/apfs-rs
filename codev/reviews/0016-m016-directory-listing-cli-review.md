# Review 0016: M-016 Synthetic Directory Listing CLI

Document version: 0.15.0  
Status: Implementation review  
Codev phase: Review

## What changed

Expose the synthetic root-directory report through `apfs ls --json`.

## Safety result

No write support, raw physical-device access, repair, format, encryption bypass, or mount code was added.

## Remaining limitation

This remains a synthetic parser-development slice pending validation against macOS-generated APFS fixtures.
