# APFS-RS

Current package version: `0.21.0`

APFS-RS is a clean-room Rust implementation for APFS inspection, extraction, mounting, and eventually carefully gated write support.

**Current status:** early read-only implementation, now with Conductor and Codev context management. This is not yet a full APFS driver, mount tool, or extractor.

## Implemented in this package

- Safe Rust workspace.
- Read-only image block-device abstraction.
- APFS container superblock probe at block zero.
- GPT-wrapped APFS partition probe.
- APFS object header parsing.
- Initial `nx_superblock_t` parsing.
- APFS Fletcher-64 object checksum calculation and validation.
- GPT CRC32 validation for the primary GPT header and partition-entry array.
- Preliminary checkpoint descriptor-area NXSB candidate scan.
- Checkpoint-map block parsing and reporting.
- Checkpoint mapping entry parsing.
- Basic container object-map header probe from checkpoint-map mapping.
- Initial B-tree node header and table-of-contents parsing.
- OMAP B-tree root probe when checkpoint maps identify `om_tree_oid`.
- Preliminary OMAP leaf-record decoding from synthetic B-tree TOC offsets.
- Single-node, checkpoint-map-backed multi-node, bounded synthetic B-tree traversal, object-map resolver facade, and production-shaped B-tree cursor facade over synthetic OMAP lookup paths.
- `apfs inspect --json <source>`.
- `apfs compatibility-report --json <source>`.
- `apfs resolver-report --json <source>`.
- `apfs btree-cursor-report --json <source> --oid <oid> --xid <xid>`.
- `apfs volumes --json <source>`.
- `apfs read-object --json <source> --oid <oid> --xid <xid>`.
- `apfs ls --json <source>`.
- `apfs cat --json <source> --name <name>`.
- Synthetic fixture generation for direct NXSB, GPT-wrapped APFS, checkpoint-candidate scan, checkpoint-map/object-map probe, OMAP B-tree root probe, single-node OMAP lookup, checkpoint-map-backed multi-node OMAP lookup, bounded synthetic B-tree traversal, object-map resolver facade, B-tree cursor facade, synthetic volume superblock probe, and mapped-object read report, synthetic directory listing, and synthetic file preview.
- Codev specs/plans/reviews and machine-readable safety/capability registries.
- Conductor context management setup under `conductor/`, with mirrored skill files under `.claude/skills/` and `.gemini/skills/`.
- Root `REQUIREMENTS.md` with MoSCoW priorities.
- Root `DESIGN.md` with Mermaid diagrams.
- Root `REMAINING_ELEMENTS.md` with a quantified remaining-work ledger.
- Real-fixture feedback loop via `tools/real_fixture_feedback.py` and `cargo xtask real-fixture-feedback`.

## Still not implemented

- Full checkpoint ring reconstruction.
- General production object-map B-tree traversal and lookup against real APFS images.
- Production APFS volume enumeration against real macOS-generated images.
- Production object payload decoding beyond generic object-header/checksum reporting.
- Production B-tree traversal beyond the bounded synthetic two-level traversal/cursor fixtures.
- Production APFS directory listing beyond synthetic fixtures.
- Production file extent resolution/extraction beyond synthetic preview.
- Windows WinFsp mount adapter.
- Compression.
- Encryption.
- Write support.

## Try it

```bash
cargo test --workspace
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-nxsb-block0.bin
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-gpt-apfs.img
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-checkpoint-ring.img
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-checkpoint-map-omap.img
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-omap-btree-root.img
cargo run -p apfs-cli -- compatibility-report --json fixtures/synthetic-omap-btree-root.img
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-omap-lookup.img
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-omap-lookup.img --oid 500 --xid 50
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-omap-multinode-lookup.img
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-omap-multinode-lookup.img --oid 700 --xid 60
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-omap-multinode-lookup.img --oid 800 --xid 59
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-btree-traversal.img
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-btree-traversal.img --oid 1500 --xid 70
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-btree-traversal.img --oid 2500 --xid 70
cargo run -p apfs-cli -- resolver-report --json fixtures/synthetic-resolver-facade.img
 cargo run -p apfs-cli -- btree-cursor-report --json fixtures/synthetic-btree-cursor.img --oid 1500 --xid 70
cargo run -p apfs-cli -- volumes --json fixtures/synthetic-volume-superblock.img
cargo run -p apfs-cli -- read-object --json fixtures/synthetic-mapped-object-read.img --oid 1500 --xid 70
cargo run -p apfs-cli -- ls --json fixtures/synthetic-directory-listing.img
cargo run -p apfs-cli -- cat --json fixtures/synthetic-file-preview.img --name hello.txt
cargo run -p apfs-cli -- path-policy --json --name hello.txt
cargo run -p apfs-cli -- feature-readiness --json --feature compression
cargo run -p apfs-cli -- metadata-feature-report --json --feature xattrs
cargo run -p apfs-cli -- lookup-object --json fixtures/synthetic-resolver-facade.img --oid 1500 --xid 70
cargo xtask registry-check
cargo xtask conductor-check
cargo xtask safety-check
cargo xtask fixture-manifest-check fixtures/manifests/macos-minimal-apfs-001.template.json
cat REQUIREMENTS.md
cat DESIGN.md
cat REMAINING_ELEMENTS.md
```

The bundled fixtures are synthetic parser-development images, not complete macOS-generated APFS filesystems.


## Real APFS fixture readiness

On macOS, after the Rust workspace compiles, generate the first real APFS image-only fixture:

```bash
./tools/macos/create_real_apfs_fixture.sh
cargo xtask fixture-manifest-check fixtures/real/macos-minimal-apfs-001/manifest.json
cargo run -p apfs-cli -- inspect --json fixtures/real/macos-minimal-apfs-001/macos-minimal-apfs-001.sparseimage > inspect.json
python3 tools/compare_inspect_to_manifest.py inspect.json fixtures/real/macos-minimal-apfs-001/manifest.json
cargo xtask real-fixture-feedback inspect.json fixtures/real/macos-minimal-apfs-001/manifest.json target/real-fixture-feedback
python3 tools/real_fixture_feedback.py inspect.json fixtures/real/macos-minimal-apfs-001/manifest.json target/real-fixture-feedback-py
```

The generator creates a sparse image file and must not be pointed at physical disks or personal data.


## v0.16.0 package additions

This package adds synthetic filesystem-root and file-preview scaffolding:

```bash
cargo run -p apfs-cli -- ls --json fixtures/synthetic-directory-listing.img
cargo run -p apfs-cli -- cat --json fixtures/synthetic-file-preview.img --name hello.txt
cargo run -p apfs-cli -- path-policy --json --name hello.txt
cargo run -p apfs-cli -- feature-readiness --json --feature compression
cargo run -p apfs-cli -- metadata-feature-report --json --feature xattrs
```

These are not production APFS directory or extent readers yet. They are bounded, read-only scaffolds that exercise the planned API shape while real macOS APFS fixture validation remains pending.


## Current synthetic CLI surface

```bash
apfs inspect --json <image>
apfs compatibility-report --json <image>
apfs lookup-object --json <image> --oid <oid> --xid <xid>
apfs resolver-report --json <image>
apfs btree-cursor-report --json <image> --oid <oid> --xid <xid>
apfs volumes --json <image>
apfs read-object --json <image> --oid <oid> --xid <xid>
apfs ls --json <image>
apfs cat --json <image> --name <name>
apfs stat --json <image> --name <name>
apfs extract --json <image> --name <name> --dest <dir>
```

`stat` and `extract` are synthetic-fixture scaffolds. `extract` writes only to the requested host destination directory and never writes to APFS media.
## v0.18.0 development-loop improvements

This package adds a cargoless quality loop for constrained environments: synthetic fixture oracle validation, traceability matrix generation, a loop dashboard/backlog counter, and a release/preflight bundle. These checks are not a replacement for Cargo, but they catch repository, fixture, traceability, and safety issues before handoff to a Rust-enabled computer.


## v0.18.0 additions

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


## v0.18.0 diagnostics and loop-hardening additions

This package adds the read-only `apfs doctor` readiness report, `apfs diagnostics-export` redacted host-side bundle export, generated CLI/API/source-metrics snapshots, and `SAFETY_CASE.md`. The Conductor checker now discovers track directories dynamically, reducing the risk that historical tracks drift from hardcoded lists.

New local pre-Rust commands:

```bash
python3 tools/cli_contract_snapshot.py
python3 tools/api_surface_snapshot.py
python3 tools/source_metrics.py
python3 tools/safety_case_check.py
python3 tools/release_preflight.py --write-manifest
```

New Rust CLI surfaces once compiled:

```bash
cargo run -p apfs-cli -- doctor --json fixtures/synthetic-file-preview.img
cargo run -p apfs-cli -- diagnostics-export --json fixtures/synthetic-file-preview.img --out target/redacted-diagnostics
```


## v0.20.0 update

Adds read-only advanced-feature readiness surfaces for Unicode/case policy, xattrs/resource forks, sparse/clones, compression, and snapshots/roles. These are diagnostic scaffolds only; production APFS support still requires real fixtures and parser validation.


## v0.20.0 local handoff additions

This package adds `LOCAL_HANDOFF.md`, `PLATFORM_SETUP.md`, `READY_FOR_LOCAL.md`, `CARGO_TRIAGE.md`, release/provenance scaffolding, and adapter-readiness crates for FUSE, Android, software-encryption policy, and image-only write-lab planning.


## v0.22.0 local handoff hardening

Added M-059 through M-064: local compile-loop orchestration, cargoless Cargo workspace audit, macOS fixture dry-run validation, WinFsp callback matrix, production-gap reporting, and batched-loop stop criteria. These do not reduce the 9 Windows read-only MVP production blockers because those require local Rust/macOS/Windows execution.

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


## v0.26.0 Current-environment controls

This handoff includes final current-environment control reports: tool capability matrix, Rust static lint, local command plan, package integrity audit, MVP blocker tasklist, and agent handoff brief. These help decide what can run here versus what must run locally with Rust, macOS APFS tooling, or Windows WinFsp.

## v0.26.0 current-environment closure additions

- `M-088` — Source debt report.
- `M-089` — Production claim guard.
- `M-090` — Handoff manifest verifier.
- `M-091` — MVP blocker dependency DAG.
- `M-092` — Local migration command generator.
- `M-093` — Current-environment final report.


## v0.27.0 QA layer

The repo now includes strict CI workflows, a 90% coverage gate policy, unit/integration/E2E/property/Hypothesis/mutation/fuzz/profiling gates, dynamic CLI version metadata, redacted opt-in CLI logging, release automation dry-runs, and an Astro 7 docs site. Required GitHub Actions execute the Rust, docs, safety, versioning, profiling-audit, and release-audit checks on pull requests and `main`; heavier coverage, fuzz, mutation, and Criterion profiling run on scheduled or manual workflows.


## v0.27.0 quality/docs hardening

Added strict CI quality gate scaffolding, >=90% coverage policy, unit/integration/E2E/property/fuzz/mutation/profiling test strategy, Astro 7 documentation-site scaffold, and cargoless audits for those additions.

## v0.27.0 strict quality and Astro documentation scaffold

This package now includes strict CI/CD quality-gate scaffolding:

- unit, integration, and end-to-end tests;
- Rust `proptest` property tests and optional Python Hypothesis tests;
- parser fuzz targets;
- mutation testing with `cargo mutants`;
- `cargo llvm-cov` coverage gate configured at **>= 90%**;
- Criterion profiling scaffold;
- Astro 7 + Starlight documentation site scaffold under `docs-site/`.

These gates are configured but still require execution on a Rust/Node-enabled machine.


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

## Post-v0.29.0 CI, logging, profiling, and dynamic versioning hardening

This pass promotes the v0.29.0 scaffolds into actively used automation:

- `apfs version --json` reports dynamic workspace version, package version, git SHA, target, and profile metadata.
- `--log-level` and `APFS_RS_LOG` enable redacted JSON operational logs on stderr without exposing full source paths or APFS media contents.
- Required CI invokes version, profiling, release automation, and aggregate bleeding-edge audits.
- Scheduled profiling runs Criterion benchmarks for `apfs-core` and `apfs-types`.
- Release automation runs cargo-dist planning and release-plz local update checks with publishing disabled.
- Astro documentation describes the executed gates and local commands.
