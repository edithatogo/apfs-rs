# Spec 0022-m021-precompile-static-validation: M-021 Precompile Static Validation

Document version: 0.16.0  
Status: Implemented scaffold  
Codev phase: Specify

## Goal

Add Python/static checks to catch repository, registry, JSON, Conductor, duplicate-symbol, unsafe, and raw-write hazards before Rust compilation.

## Non-goals

- Production APFS semantics.
- Raw physical-device access.
- APFS write support.
- Mounting, repair, format, compression, or encryption.

## Safety

Validation-only; no APFS media access.
