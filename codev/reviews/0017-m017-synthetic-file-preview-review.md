# Review 0017: M-017 Synthetic File Preview

Document version: 0.15.0  
Status: Implementation review  
Codev phase: Review

## What changed

Preview a synthetic direct-block file entry through `apfs cat --json --name` without implementing production extents.

## Safety result

No write support, raw physical-device access, repair, format, encryption bypass, or mount code was added.

## Remaining limitation

This remains a synthetic parser-development slice pending validation against macOS-generated APFS fixtures.
