# Review M-071: APFS Offset and Fixture Byte-Layout Audit

Document version: 0.25.0  
Status: Implemented scaffold review  
Codev phase: Review

## What changed

Added APFS Offset and Fixture Byte-Layout Audit as part of the current-environment completion pass.

## Safety

The change is host-side/cargoless and does not open, mount, decrypt, repair, format, or mutate APFS media.

## Remaining validation

Rust/Cargo, real macOS APFS fixtures, and Windows/WinFsp validation still require a local/platform environment.
