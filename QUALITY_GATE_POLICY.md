
# Quality Gate Policy

Version: 0.27.0

APFS-RS is filesystem software; its quality gates are intentionally stricter than ordinary CLI projects.

## Configured gates

| Gate | Status | Execution environment |
|---|---|---|
| Unit tests | Configured in Rust crates | Requires Cargo |
| Integration tests | Configured in crate `tests/` dirs | Requires Cargo |
| End-to-end CLI tests | Configured under `crates/apfs-cli/tests` | Requires Cargo |
| Property tests | Configured with Rust `proptest` and Python Hypothesis | Requires Cargo/Python deps |
| Fuzz tests | Configured with `cargo-fuzz` | Requires Cargo |
| Mutation tests | Configured with `cargo-mutants` | Requires Cargo |
| Coverage | Configured with `cargo-llvm-cov` and `--fail-under-lines 90` | Requires Cargo |
| Profiling | Configured with Criterion benchmark scaffold | Requires Cargo |
| Cargoless checks | Configured and runnable here | Runs now |
| CI gates | Configured in ci.yml, strict-quality.yml | Requires Cargo |
| Supply-chain checks | Configured in supply-chain.yml | Requires Cargo |
| Release/provenance gates | Configured in release.yml, provenance-verify.yml | Requires Cargo |
| Docs site build | Configured in docs-site.yml | Requires Node.js |

## Coverage threshold

The target threshold is **90% line coverage** for the Rust workspace. The CI workflow `coverage.yml` is configured to enforce:

```bash
cargo llvm-cov nextest --workspace --all-features --fail-under-lines 90
```

This cannot be executed in the current environment because Cargo is unavailable. It must be run locally or in GitHub Actions after the first compile pass.

## Mutation testing

`cargo-mutants` is configured as a scheduled/manual quality gate. Surviving mutants in parser, VFS, extraction, or write-lab code must be reviewed before a Windows read-only MVP release.

## Hypothesis/property testing

Rust property tests use `proptest`. Python tooling/fixture properties use Hypothesis under `tools/tests/`.

## CD/Deployment gates

CD gates protect release and deployment quality. They are configured as GitHub Actions workflows that run on version tags and workflow dispatch:

| Gate | Workflow | Trigger | Requires |
|------|----------|---------|----------|
| Release preflight | `release.yml` | Tag `apfs-rs-v*` | Cargo |
| Release automation | `release-automation.yml` | Tag `v*` | Cargo |
| Provenance verification | `provenance-verify.yml` | Release published | Cargo |
| Supply-chain audit | `supply-chain.yml` | PR, schedule | Cargo |
| Workflow security | `workflow-security.yml` | PR, schedule | actionlint/zizmor |
| Docs build quality | `docs-quality.yml` | PR, push | Node.js, Python |

These gates cannot be executed in the current environment because they require Cargo, Node.js, or GitHub-hosted runners. They must pass before any production release artifact is published.

## Combined CI/CD quality gate diagram

```
CI gates (PR/push):
  ┌─ ci.yml: fmt, clippy, test, registry, conductor, safety
  ├─ quality-gates.yml: coverage ≥90%, nextest, fuzz, mutation
  ├─ strict-quality.yml: full suite + deny + audit + mutants
  ├─ coverage.yml: llvm-cov ≥90%
  ├─ fuzz.yml: scheduled fuzz smoke
  ├─ mutation.yml: scheduled mutation tests
  ├─ profiling.yml: criterion benchmarks
  ├─ supply-chain.yml: dependency review, scorecard, cargo-vet
  └─ workflow-security.yml: actionlint, zizmor

CD gates (tag/release):
  ├─ release.yml: preflight checks + attestation
  ├─ release-automation.yml: cargo-dist + release-plz
  └─ provenance-verify.yml: attestation verification

Supporting gates:
  ├─ docs-site.yml: Astro build
  ├─ docs-quality.yml: package audit + link audit
  ├─ local-handoff.yml: cargoless smoke + config sanity
  └─ python-property.yml: Hypothesis tests
```
