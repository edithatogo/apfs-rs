# APFS-RS v0.16 Local Runbook

This package was prepared in an environment without Rust/Cargo. Use this runbook after unzipping on a Rust-enabled computer.

## 1. Enter the workspace

```bash
cd apfs-rs-impl-v0.16
```

## 2. Run baseline checks

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo xtask registry-check
cargo xtask safety-check
cargo xtask precompile-check
```

If `cargo fmt` reports formatting changes, run:

```bash
cargo fmt --all
```

then re-run the checks.

## 3. Regenerate synthetic fixtures

```bash
python3 tools/make_synthetic_fixtures.py
```

The fixtures are synthetic parser-development images. They are not full APFS filesystems.

## 4. Try inspection commands

```bash
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-nxsb-block0.bin
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-gpt-apfs.img
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-checkpoint-ring.img
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-checkpoint-map-omap.img
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-omap-btree-root.img
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-omap-lookup.img
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-omap-multinode-lookup.img
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-btree-traversal.img
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-resolver-facade.img
```

## 5. Try lookup commands

Single-node synthetic lookup:

```bash
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-omap-lookup.img --oid 500 --xid 50
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-omap-lookup.img --oid 500 --xid 49
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-omap-lookup.img --oid 999 --xid 50
```

Expected behaviour:

```text
oid 500, xid 50 -> physical block 20
oid 500, xid 49 -> physical block 22
oid 999, xid 50 -> not found
```

Checkpoint-map-backed multi-node synthetic lookup:

```bash
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-omap-multinode-lookup.img --oid 700 --xid 60
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-omap-multinode-lookup.img --oid 800 --xid 59
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-omap-multinode-lookup.img --oid 800 --xid 60
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-omap-multinode-lookup.img --oid 999 --xid 60
```

Expected behaviour:

```text
oid 700, xid 60 -> physical block 25
oid 800, xid 59 -> physical block 27
oid 800, xid 60 -> physical block 28
oid 999, xid 60 -> not found
```

## 6. First fixes to make after compilation

Because this archive has not been compiled here, expect possible Rust formatting or compiler issues. Fix in this order:

1. Formatting.
2. Compiler errors.
3. Clippy warnings if you enable clippy.
4. Unit test failures.
5. Registry/safety-check failures.

Do not start Windows mounting, encryption, compression, or writes until the current read-only parser slices compile and pass tests.


Bounded synthetic B-tree traversal lookup:

```bash
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-btree-traversal.img --oid 1500 --xid 70
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-btree-traversal.img --oid 2500 --xid 70
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-btree-traversal.img --oid 42 --xid 70
```

Expected behaviour:

```text
oid 1500, xid 70 -> selected child 111, physical block 33
oid 2500, xid 70 -> selected child 112, physical block 35
oid 42, xid 70 -> selected child 110, not found in that selected leaf
```


## 7. Try resolver facade commands

```bash
cargo run -p apfs-cli -- resolver-report --json fixtures/synthetic-resolver-facade.img
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-resolver-facade.img --oid 1500 --xid 70
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-resolver-facade.img --oid 2500 --xid 70
```

Expected behaviour:

```text
resolver-report -> available, bounded_synthetic_two_level_traversal
oid 1500, xid 70 -> physical block 33 via resolver facade
oid 2500, xid 70 -> physical block 35 via resolver facade
```


## 8. Try B-tree cursor facade commands

```bash
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-btree-cursor.img
cargo run -p apfs-cli -- btree-cursor-report --json fixtures/synthetic-btree-cursor.img --oid 1500 --xid 70
cargo run -p apfs-cli -- btree-cursor-report --json fixtures/synthetic-btree-cursor.img --oid 2500 --xid 70
```

Expected behaviour:

```text
oid 1500, xid 70 -> synthetic cursor selects child 111 and physical block 33
oid 2500, xid 70 -> synthetic cursor selects child 112 and physical block 35
```

## 9. Conductor context management

The repo includes Conductor-compatible persistent context under `conductor/`. If using Gemini CLI Conductor, install the extension separately and run status/review commands from the workspace root. Without the extension, agents can still read the same Markdown files directly.


## 10. Real APFS fixture readiness on macOS

After the baseline Rust checks pass on macOS, create the first real APFS image-only fixture:

```bash
./tools/macos/create_real_apfs_fixture.sh
cargo xtask fixture-manifest-check fixtures/real/macos-minimal-apfs-001/manifest.json
cargo run -p apfs-cli -- inspect --json fixtures/real/macos-minimal-apfs-001/macos-minimal-apfs-001.sparseimage > inspect.json
python3 tools/compare_inspect_to_manifest.py inspect.json fixtures/real/macos-minimal-apfs-001/manifest.json
```

Do not use personal disks. The script creates a sparse image file and records redacted oracle output only.

## 11. Conductor full-history check

The Conductor context now includes a track for every implementation slice from M-001 through M-014.

```bash
cargo xtask conductor-check
```

This check verifies root Conductor context files and all historical track directories.


## v0.12 real-fixture feedback

After generating a real macOS APFS fixture and running `apfs inspect --json`, create a feedback packet:

```bash
cargo xtask real-fixture-feedback inspect.json fixtures/real/macos-minimal-apfs-001/manifest.json target/real-fixture-feedback
python3 tools/real_fixture_feedback.py inspect.json fixtures/real/macos-minimal-apfs-001/manifest.json target/real-fixture-feedback-py
```

The feedback loop reads JSON artifacts only. It does not open, mount, decrypt, repair, format, or write APFS media.


## v0.13 synthetic volume-superblock probe

Try the synthetic volume report path:

```bash
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-volume-superblock.img
cargo run -p apfs-cli -- volumes --json fixtures/synthetic-volume-superblock.img
just volume-example
```

Expected behaviour:

```text
volumes -> available
filesystem oid 1000 -> volume name SyntheticHD, root tree oid 2000
```

This remains synthetic-fixture-only until real macOS APFS volume-superblock offsets and semantics are validated.

## M-014 mapped-object read report

```bash
cargo run -p apfs-cli -- read-object --json fixtures/synthetic-mapped-object-read.img --oid 1500 --xid 70
cargo run -p apfs-cli -- read-object --json fixtures/synthetic-mapped-object-read.img --oid 2500 --xid 70
```


## Synthetic directory and file-preview commands

```bash
cargo run -p apfs-cli -- ls --json fixtures/synthetic-directory-listing.img
cargo run -p apfs-cli -- cat --json fixtures/synthetic-file-preview.img --name hello.txt
```

Expected synthetic behaviour:

```text
ls -> entries including hello.txt and Documents
cat --name hello.txt -> bounded preview: Hello from APFS-RS synthetic file preview!
```

These commands use synthetic record layouts only. They are API scaffolds for future production APFS filesystem tree and extent work.

## v0.16 synthetic stat/extract examples

```bash
cargo run -p apfs-cli -- stat --json fixtures/synthetic-file-preview.img --name hello.txt
rm -rf target/synthetic-extract
cargo run -p apfs-cli -- extract --json fixtures/synthetic-file-preview.img --name hello.txt --dest target/synthetic-extract
```

`extract` writes only to the requested host destination directory. It rejects path separators and parent-directory traversal in the synthetic file name and never writes to APFS media.

## Precompile validation without Rust/Cargo

```bash
python3 tools/precompile_static_check.py
```

When Rust is available, run the wrapper too:

```bash
cargo xtask precompile-check
```
## v0.17.0 cargoless validation loop

Run these before handing the package to a Rust-enabled machine:

```bash
python3 tools/precompile_static_check.py
python3 tools/synthetic_fixture_oracle.py
python3 tools/traceability_matrix.py
python3 tools/loop_dashboard.py
python3 tools/release_preflight.py --write-manifest
```

Then run the Rust loop once Cargo is available.


## v0.18.0 commands

- Added `apfs-vfs` read-only facade crate.
- Added `apfs-win` Windows readiness scaffold.
- Added redacted diagnostics bundle tooling.
- Added fuzz target scaffolds.
- Added cargoless Rust API map and next-loop planning tools.

Useful commands:

```bash
python3 tools/diagnostics_bundle.py --out target/diagnostics fixtures/diagnostics/sample-inspect.json
python3 tools/rust_api_map.py
python3 tools/next_loop_plan.py
python3 tools/windows_readiness_check.py
```

## Windows mount planning scaffold

This command produces a read-only WinFsp mount preflight plan. It does not mount anything yet.

```bash
cargo run -p apfs-cli -- mount-plan --json fixtures/synthetic-file-preview.img --mountpoint X:
```

## Redacted diagnostics bundle

This command exports redacted inspect/resolver/volume/directory metadata for bug reports and agent workflows. It does not include file contents, raw blocks, secrets, or full directory names.

```bash
cargo run -p apfs-cli -- diagnostics-bundle --json fixtures/synthetic-file-preview.img
```

## Cargoless contract checks

```bash
python3 tools/cli_contract_check.py
python3 tools/context_integrity_check.py
```


## v0.18.0 cargoless loop

```bash
python3 tools/precompile_static_check.py
python3 tools/synthetic_fixture_oracle.py
python3 tools/cli_contract_snapshot.py
python3 tools/api_surface_snapshot.py
python3 tools/source_metrics.py
python3 tools/safety_case_check.py
python3 tools/release_preflight.py --write-manifest
```

Once Rust is available, also run:

```bash
cargo xtask cli-contract
cargo xtask api-surface-snapshot
cargo xtask source-metrics
cargo xtask safety-case-check
```

## v0.18 doctor and diagnostics-export

```bash
cargo run -p apfs-cli -- doctor --json fixtures/synthetic-file-preview.img
cargo run -p apfs-cli -- diagnostics-export --json fixtures/synthetic-file-preview.img --out target/redacted-diagnostics
```

`doctor` aggregates read-only readiness and blocker information. `diagnostics-export` writes a redacted host-side bundle and never writes to APFS media.

## v0.20.0 feature readiness commands

```bash
python3 tools/feature_readiness_snapshot.py
cargo run -p apfs-cli -- path-policy --json --name hello.txt
cargo run -p apfs-cli -- feature-readiness --json --feature compression
cargo run -p apfs-cli -- metadata-feature-report --json --feature xattrs
```

## v0.20.0 version and readiness validation

```bash
python3 tools/feature_readiness_snapshot.py
python3 tools/version_consistency_check.py
```


## v0.20.0 first local loop

```bash
python3 tools/handoff_readiness_check.py
python3 tools/release_scaffold_check.py
python3 tools/cargo_error_to_tracks.py fixtures/feedback/sample-cargo-error.log target/cargo-triage-sample
```


## v0.22.0 local handoff hardening

Added M-059 through M-064: local compile-loop orchestration, cargoless Cargo workspace audit, macOS fixture dry-run validation, WinFsp callback matrix, production-gap reporting, and batched-loop stop criteria. These do not reduce the 9 Windows read-only MVP production blockers because those require local Rust/macOS/Windows execution.

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

## v0.26.0 current-environment closure additions

- `M-088` — Source debt report.
- `M-089` — Production claim guard.
- `M-090` — Handoff manifest verifier.
- `M-091` — MVP blocker dependency DAG.
- `M-092` — Local migration command generator.
- `M-093` — Current-environment final report.



## v0.27.0 quality/docs hardening

Added strict CI quality gate scaffolding, >=90% coverage policy, unit/integration/E2E/property/fuzz/mutation/profiling test strategy, Astro 7 documentation-site scaffold, and cargoless audits for those additions.

## v0.27.0 strict local quality loop

Once Rust and Node are available:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace --all-features
cargo llvm-cov nextest --workspace --all-features --fail-under-lines 90 --summary-only
cargo test -p apfs-types --test property_nx_superblock
cargo test -p apfs-core --test integration_inspect
cargo test -p apfs-cli --test e2e_cli
cargo bench -p apfs-core --bench inspect_synthetic
cd docs-site && npm install && npm run build
```


## v0.28.0 Quality Evidence Additions

This handoff adds docs-package, GitHub workflow, test inventory, Hypothesis, profiling budget, and quality-gate evidence audits. These are cargoless checks that validate configuration and scaffolding before Rust/Cargo/npm execution.


## v0.29.0 Repo Hardening and Automation

- `M-110` — GitHub Actions hardening with zizmor/actionlint policy.
- `M-111` — GitHub Actions pinning and permissions audit.
- `M-112` — cargo-vet supply-chain review policy.
- `M-113` — SLSA and artifact attestation verification plan.
- `M-114` — cargo-dist and release-plz automation scaffold.
- `M-115` — OpenSSF Scorecard and dependency-review workflow scaffold.
- `M-116` — Astro 7 documentation quality hardening.
- `M-117` — Benchmark regression and optional CodSpeed readiness.
- `M-118` — Bleeding-edge repo hardening audit aggregator.
