# Review 0022-m021-precompile-static-validation: M-021 Precompile Static Validation

Document version: 0.16.0  
Status: Review  
Codev phase: Review

## What changed

Add Python/static checks to catch repository, registry, JSON, Conductor, duplicate-symbol, unsafe, and raw-write hazards before Rust compilation.

## Safety result

Validation-only; no APFS media access.

## Remaining work

Compile and test the Rust workspace on a Rust-enabled computer, then validate behaviour against synthetic and real macOS-generated APFS fixtures.
