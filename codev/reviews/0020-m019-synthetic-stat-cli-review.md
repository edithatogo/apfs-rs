# Review 0020-m019-synthetic-stat-cli: M-019 Synthetic Stat CLI

Document version: 0.16.0  
Status: Review  
Codev phase: Review

## What changed

Report metadata for one synthetic directory entry without implementing production APFS inode/stat decoding.

## Safety result

Run over synthetic directory fixtures only; no APFS media writes.

## Remaining work

Compile and test the Rust workspace on a Rust-enabled computer, then validate behaviour against synthetic and real macOS-generated APFS fixtures.
