# Testing Infrastructure Report

Configured here; must be executed and fixed on a Rust/Node-enabled machine.

| Area | Command/file | Status |
|---|---|---|
| `unit_tests` | `cargo test --workspace` | `configured` |
| `integration_tests` | `crates/apfs-core/tests/integration_inspect.rs` | `configured` |
| `end_to_end_tests` | `crates/apfs-cli/tests/e2e_cli.rs` | `configured` |
| `property_tests` | `Rust proptest + optional Python Hypothesis` | `configured` |
| `fuzz_tests` | `cargo fuzz object_header/nx_superblock` | `configured` |
| `mutation_tests` | `cargo mutants scheduled/manual` | `configured` |
| `coverage` | `cargo llvm-cov nextest --fail-under-lines 90` | `configured` |
| `profiling` | `Criterion bench and profiling workflow` | `configured` |
| `docs_site` | `Astro 7 + Starlight scaffold` | `configured` |
