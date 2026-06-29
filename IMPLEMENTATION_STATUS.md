# APFS-RS Implementation Status

Version: v0.21.0 package  
Date: 2026-06-24

## Implemented/scaffolded in this package

- Safe Rust workspace.
- Read-only image block-device abstraction and in-memory test device.
- APFS object header parsing.
- Initial `nx_superblock_t` parsing.
- APFS Fletcher-64 checksum calculation and validation.
- GPT header/partition parsing and CRC32 checks.
- Direct APFS-at-block-zero and GPT-wrapped APFS image inspection.
- Preliminary checkpoint descriptor-area scan for valid NXSB candidates.
- `checkpoint_mapping_t` and `checkpoint_map_phys_t` parsing.
- Basic `omap_phys_t` parsing and container OMAP header probe.
- Initial B-tree node header/TOC parsing with bounds checks.
- Preliminary synthetic OMAP record decoding.
- Single-node, checkpoint-map-backed multi-node, and bounded synthetic two-level OMAP lookup paths.
- Object-map resolver facade and resolver-report CLI.
- Production-shaped B-tree cursor facade and cursor-report CLI.
- macOS real APFS fixture readiness harness.
- Real-fixture feedback loop and feedback-to-track promotion scaffold.
- Synthetic APFS volume-superblock parser and `apfs volumes --json`.
- Resolver-backed mapped object read report and `apfs read-object --json`.
- Synthetic filesystem root-tree directory-record parser.
- Synthetic root directory listing and `apfs ls --json`.
- Synthetic direct-block file preview and `apfs cat --json --name <name>`.
- Codev specs/plans/reviews/resources for M-001 through M-021.
- Conductor context management tracks/history/skills for the full development history through M-021.
- Synthetic metadata/stat report and `apfs stat --json --name <name>`.
- Safe host-side synthetic extract-preview and `apfs extract --json --name <name> --dest <dir>`.
- Python/static precompile validation and `cargo xtask precompile-check`.
- Root `REQUIREMENTS.md`, `DESIGN.md`, and `REMAINING_ELEMENTS.md`.

## Current package commands

```bash
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-nxsb-block0.bin
cargo run -p apfs-cli -- volumes --json fixtures/synthetic-volume-superblock.img
cargo run -p apfs-cli -- read-object --json fixtures/synthetic-mapped-object-read.img --oid 1500 --xid 70
cargo run -p apfs-cli -- ls --json fixtures/synthetic-directory-listing.img
cargo run -p apfs-cli -- cat --json fixtures/synthetic-file-preview.img --name hello.txt
cargo run -p apfs-cli -- resolver-report --json fixtures/synthetic-resolver-facade.img
cargo run -p apfs-cli -- btree-cursor-report --json fixtures/synthetic-btree-cursor.img --oid 1500 --xid 70
cargo xtask registry-check
cargo xtask conductor-check
cargo xtask safety-check
cargo xtask precompile-check
```

## Not implemented

- Compilation/lint/test validation on a Rust-enabled computer.
- Real macOS APFS fixture validation.
- Full APFS checkpoint ring reconstruction.
- Production object-map B-tree traversal against real APFS images.
- Recursive/general APFS filesystem B-tree traversal beyond synthetic fixtures.
- Production APFS directory record decoding.
- Production file extent resolution and extraction.
- Windows WinFsp mount adapter.
- Compression.
- Encryption.
- Write support.
- Repair or format.

## Validation status

The package was created in an environment without Rust/Cargo, so Rust compilation has not been executed here. The ZIP archive, YAML registries, Conductor track history, synthetic fixture layout, and package manifest are validated with Python and `unzip -t`.

## v0.16.0 update

Added M-019 synthetic stat CLI, M-020 safe synthetic extract-preview CLI, and M-021 Python/static precompile validation. Conductor history now covers M-001 through M-021.
## v0.18.0

Added M-022 through M-025: synthetic fixture oracle, traceability matrix generator, loop dashboard/backlog counter, and cargoless release preflight. Remaining counts are now generated from `REMAINING_ELEMENTS.yaml`.


## v0.18.0 implementation update

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

## Added in v0.18.0

- M-026: Windows read-only mount-plan facade (`apfs mount-plan --json`).
- M-027: Redacted diagnostics bundle CLI (`apfs diagnostics-bundle --json`).
- M-028: CLI contract static check.
- M-029: Context integrity static check.


## v0.18.0

Added read-only doctor and redacted diagnostics export CLI scaffolds, CLI/API/source-metrics snapshots, SAFETY_CASE.md, and dynamic Conductor track validation. Rust compilation remains pending on a Rust-enabled computer.


## v0.21.0 update

Added M-034 through M-040: advanced feature readiness surfaces, feature readiness snapshots, and version consistency checks. These remain read-only/report-only scaffolds until real APFS fixtures and Rust compilation are available.


## v0.21.0 status

M-041 through M-049 are implemented as local handoff, triage, platform, release, adapter-readiness, encryption-readiness, and write-lab-readiness scaffolds. Production validation still requires Rust, macOS APFS fixture data, and later Windows + WinFsp.

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
