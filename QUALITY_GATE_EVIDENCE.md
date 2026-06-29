# Quality Gate Evidence

This file distinguishes configured gates from gates that have been executed. Rust, Cargo and npm build execution must happen locally or in CI.

| Gate | Configured | Requires local/CI execution | Command |
|---|---:|---:|---|
| format | `true` | `true` | `cargo fmt --all -- --check` |
| clippy | `true` | `true` | `cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| unit/integration/e2e | `true` | `true` | `cargo nextest run --workspace --all-features` |
| >=90% coverage | `true` | `true` | `cargo llvm-cov nextest --workspace --all-features --fail-under-lines 90` |
| mutation | `true` | `true` | `cargo mutants --workspace` |
| fuzz smoke | `true` | `true` | `cargo fuzz run object_header -- -max_total_time=60` |
| Hypothesis-style Python tests | `true` | `false` | `pytest python_tests` |
| profiling | `true` | `true` | `cargo bench` |
| Astro 7 docs | `true` | `true` | `cd docs-site && npm install && npm run build` |
