# Plan: Strict CI/CD quality gates with >=90% coverage target

1. Keep configuration read-only and source-only.
2. Validate with cargoless checks here.
3. Execute Rust/Node tooling locally or in CI.
4. Feed failures back into Codev and Conductor tasks.
