# CI, Testing, Mutation, Property, and Profiling Plan

Document version: 0.27.0

## Answer to “is strict CI set up?”

Configured: yes. Verified by local Rust execution: not yet, because this environment lacks `cargo` and `rustc`.

## Required after local checkout

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace --all-features
cargo llvm-cov nextest --workspace --all-features --fail-under-lines 90
cargo mutants --workspace
cargo fuzz run object_header -- -max_total_time=60
cargo fuzz run nx_superblock -- -max_total_time=60
cargo bench --workspace
```

## Testing layers

- Unit tests: parser and safety helpers.
- Integration tests: synthetic fixture library APIs.
- End-to-end tests: CLI commands against synthetic and later real APFS fixtures.
- Property/Hypothesis-style tests: Rust `proptest` first; Python Hypothesis optional for cargoless fixture/report checks.
- Mutation: `cargo-mutants`.
- Profiling: `cargo bench`, Criterion, flamegraph/hyperfine.
