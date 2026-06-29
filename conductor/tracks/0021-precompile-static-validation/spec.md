# M-021 Precompile Static Validation

## Goal

Provide non-Rust static validation for environments where Cargo cannot run.

## Safety

Read-only with respect to APFS media. No raw-device write, mount, repair, format, compression, or encryption behaviour is added.
