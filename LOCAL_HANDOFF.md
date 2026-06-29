# APFS-RS Local Handoff Runbook

Version: 0.20.0

This repository has reached the point where the next high-value work requires a Rust-enabled machine, then macOS APFS fixture generation, and later Windows + WinFsp mount testing.

## First local commands

```bash
python3 tools/precompile_static_check.py
python3 tools/synthetic_fixture_oracle.py
python3 tools/context_integrity_check.py
python3 tools/release_preflight.py --write-manifest

cargo fmt --all -- --check
cargo test --workspace
cargo xtask registry-check
cargo xtask safety-check
cargo xtask conductor-check
```

## Expected first failures

The first Rust run may reveal compile errors because this package was assembled in an environment without Cargo. Triage in this order:

1. Workspace manifest/member errors.
2. Missing imports or dependency declarations.
3. Clippy/lint warnings.
4. Failing unit tests.
5. JSON/schema output mismatches.

Use:

```bash
cargo test --workspace 2>&1 | tee target/cargo-test.log
python3 tools/cargo_error_to_tracks.py target/cargo-test.log target/cargo-triage
```

## First real APFS validation

On macOS:

```bash
./tools/macos/create_real_apfs_fixture.sh
cargo run -p apfs-cli -- inspect --json fixtures/real/macos-minimal-apfs-001/macos-minimal-apfs-001.sparseimage > inspect.json
cargo xtask real-fixture-feedback inspect.json fixtures/real/macos-minimal-apfs-001/manifest.json target/real-fixture-feedback
cargo xtask promote-feedback target/real-fixture-feedback/real-fixture-feedback.json target/promoted-feedback-tasks
```

## Safety reminder

Do not add raw physical-device writes. Do not add password recovery. Do not mount read-write. The first local goal is compile/test and read-only fixture validation only.

## v0.21.0 additions

Use `LOCAL_FIRST_RUN.md` as the exact first-run checklist. It adds a local environment doctor, Cargo failure promotion, and a triage order for build/test/real-fixture feedback.

## v0.23.0 current-environment hardening

Added current environment inventory, remaining-work classifier, dependency graph, synthetic negative fixtures, test/control matrix, and archive audit.

## v0.25.0 current-environment validation

Before moving locally, run:

```bash
python3 tools/cargoless_smoke_suite.py
python3 tools/current_env_completion_report.py
```

These confirm that current-environment validation is exhausted and that remaining production work requires Rust/macOS/Windows execution.

## Current handoff version

`0.25.0`


## v0.26.0 Current-Environment Control Update

Added tool capability matrix, Rust static lint, package integrity audit, MVP blocker tasklist, agent handoff brief, and local command plan. Required current-environment-completable items remaining: 0. Production blockers remain local/platform dependent.


## v0.28.0 Quality Evidence Additions

This handoff adds docs-package, GitHub workflow, test inventory, Hypothesis, profiling budget, and quality-gate evidence audits. These are cargoless checks that validate configuration and scaffolding before Rust/Cargo/npm execution.
