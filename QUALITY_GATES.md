# APFS-RS Quality Gates

Version: 0.27.0

APFS-RS treats CI quality as a safety boundary because filesystem parsers consume untrusted bytes and future write support can cause data loss.

## Required local/CI gates

| Gate | Required target | Status |
|---|---:|---|
| Unit tests | required on every PR | configured |
| Integration tests | required on every PR | configured |
| End-to-end CLI tests | required on every PR once Cargo is available | configured |
| Line coverage | **>= 90%** with `cargo llvm-cov` | configured, not yet executed here |
| Property / hypothesis-style tests | Rust `proptest`, optional Python Hypothesis | configured |
| Fuzz smoke | `cargo fuzz` parser targets | scaffolded |
| Mutation testing | `cargo mutants` scheduled/manual | configured |
| Profiling/benchmarks | Criterion benchmark + profiling workflow | configured |
| Cargoless checks | Python/static checks | executed here |

## Important caveat

The gates are configured in this package, but they are not yet proven until the project is run on a Rust-enabled computer.


## Coverage gate

Coverage must be >= 90% line coverage after local Rust compilation is available.
