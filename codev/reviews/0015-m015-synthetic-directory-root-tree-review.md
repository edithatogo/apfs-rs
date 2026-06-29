# Review 0015: M-015 Synthetic Filesystem Root Tree Records

Document version: 0.15.0  
Status: Implementation review  
Codev phase: Review

## What changed

Parse bounded synthetic filesystem-directory records from a mapped volume root-tree B-tree node.

## Safety result

No write support, raw physical-device access, repair, format, encryption bypass, or mount code was added.

## Remaining limitation

This remains a synthetic parser-development slice pending validation against macOS-generated APFS fixtures.
