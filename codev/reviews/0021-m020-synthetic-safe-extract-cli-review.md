# Review 0021-m020-synthetic-safe-extract-cli: M-020 Synthetic Safe Extract CLI

Document version: 0.16.0  
Status: Review  
Codev phase: Review

## What changed

Write a bounded synthetic direct-block file preview to a host destination directory with path traversal safeguards.

## Safety result

Writes only to host output directory; never writes to APFS media; synthetic preview only.

## Remaining work

Compile and test the Rust workspace on a Rust-enabled computer, then validate behaviour against synthetic and real macOS-generated APFS fixtures.
