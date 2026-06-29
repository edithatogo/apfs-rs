# APFS-RS Test Strategy

Version: 0.27.0

## Test layers

```mermaid
flowchart TB
    Unit[Unit tests]
    Prop[Property tests: proptest / Hypothesis]
    Integration[Integration tests]
    E2E[End-to-end CLI tests]
    Fuzz[Fuzzing]
    Mutation[Mutation testing]
    Coverage[Coverage >= 90%]
    Profiling[Criterion/profiling]
    Fixture[Real APFS fixture validation]
    Unit --> Integration --> E2E --> Fixture
    Prop --> Coverage
    Fuzz --> Coverage
    Mutation --> Coverage
    Profiling --> ReleaseReadiness[Release readiness]
```

## Current setup

- Rust property tests: `crates/apfs-types/tests/property_nx_superblock.rs`.
- Core integration tests: `crates/apfs-core/tests/integration_inspect.rs`.
- CLI e2e tests: `crates/apfs-cli/tests/e2e_cli.rs`.
- Optional Python Hypothesis tests: `python_tests/test_fixture_properties.py`.
- Fuzz targets: `fuzz/fuzz_targets/`.
- Benchmark/profiling scaffold: `crates/apfs-core/benches/inspect_synthetic.rs`.
- CI quality gate workflow: `.github/workflows/quality-gates.yml`.

## Coverage target

Line coverage target: **>= 90%** for the workspace using `cargo llvm-cov` once the workspace compiles locally.
