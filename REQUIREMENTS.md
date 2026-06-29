# APFS-RS Requirements

Document version: 0.18.0  
Status: Current MoSCoW requirements ledger  
Date: 2026-06-24

## Product objective

Build a clean-room Rust APFS implementation that starts with Windows-first read-only inspection, extraction, and mounting, then expands safely toward advanced read features and later lab-gated write support.

## MoSCoW requirements

### Must / implemented or scaffolded so far

| ID | Requirement | Current state |
|---|---|---|
| M-001 | APFS image/container detection | partial_synthetic |
| M-002 | GPT-wrapped APFS partition probe | partial_synthetic |
| M-003 | APFS Fletcher-64 checksum validation | partial_synthetic |
| M-004 | Checkpoint scan, checkpoint-map parsing, and early OMAP probe | partial_synthetic |
| M-005 | B-tree node header/TOC probe and preliminary OMAP records | partial_synthetic |
| M-006 | Single-node synthetic OMAP lookup API and CLI | partial_synthetic |
| M-007 | Checkpoint-map-backed multi-node synthetic OMAP lookup | partial_synthetic |
| M-008 | Bounded synthetic two-level OMAP traversal | partial_synthetic |
| M-009 | Object-map resolver facade and resolver-report CLI | facade_scaffold |
| M-010 | Production-shaped B-tree cursor facade | facade_scaffold |
| M-011 | Real APFS fixture readiness harness | scaffold |
| M-012 | Real-fixture feedback loop | scaffold |
| M-013 | Feedback-to-Codev/Conductor promotion | scaffold |
| M-014 | Synthetic volume-superblock probe and apfs volumes | partial_synthetic |
| M-015 | Resolver-backed mapped object read report | partial_synthetic |
| M-016 | Synthetic filesystem root-tree directory-record parser | partial_synthetic |
| M-017 | Synthetic directory listing CLI | partial_synthetic |
| M-018 | Synthetic direct-block file preview CLI | partial_synthetic |
| M-019 | Synthetic metadata/stat report CLI | partial_synthetic |
| M-020 | Synthetic safe extract-preview CLI | partial_synthetic |
| M-021 | Precompile static validation harness | scaffold |
| M-022 | Synthetic fixture oracle and smoke-test harness | implemented_python |
| M-023 | Requirements-to-Codev-to-Conductor traceability matrix generator | implemented_python |
| M-024 | Loop dashboard and backlog counter | implemented_python |
| M-025 | Cargoless release-readiness/preflight bundle | implemented_python |

### Must / remaining for Windows read-only MVP production path

| ID | Requirement | Dependency |
|---|---|---|
| MVP-R001 | Compile, lint, test, and fix the current workspace on Rust-enabled machines | Rust-enabled machine |
| MVP-R002 | Generate first real macOS APFS image and manifest | macOS |
| MVP-R003 | Run real-fixture feedback loop and convert mismatches into tasks | M-012/M-013 and real fixture |
| MVP-R004 | Correct APFS parser offsets/semantics against real APFS data | real fixture feedback |
| MVP-R005 | Full checkpoint ring reconstruction | corrected parser and real fixture |
| MVP-R006 | Production APFS object-map B-tree traversal | corrected checkpoint and B-tree parsing |
| MVP-R007 | Production APFS filesystem tree record decoding and metadata/stat mapping | production OMAP traversal |
| MVP-R008 | Production file extent resolution and extraction | production filesystem records |
| MVP-R009 | Windows WinFsp read-only mount adapter, smoke tests, and packaging baseline | stable read API |

### Should

| ID | Requirement | State |
|---|---|---|
| S-001 | Metadata mapping for timestamps, POSIX modes, symlinks, hard links | Remaining |
| S-002 | Unicode and case-sensitivity policy | Remaining |
| S-003 | Redacted diagnostics bundle | Remaining |
| S-004 | Fuzz/property/mutation/coverage hardening | Remaining |
| S-005 | Release signing, SBOM, provenance, and Windows packaging | Remaining |
| S-006 | Cargoless release/preflight bundle | Implemented scaffold in M-025 |

### Could

| ID | Requirement | State |
|---|---|---|
| C-001 | Extended attributes and resource forks | Later |
| C-002 | Sparse files and clone/reflink awareness | Later |
| C-003 | Snapshot listing and read-only snapshot views | Later |
| C-004 | Linux/macOS/ChromeOS/Android adapters | Later |
| C-005 | GUI/helper wrapper | Later |

### Won't yet

| ID | Requirement | Reason |
|---|---|---|
| W-001 | Physical-device write support | Requires image-only write lab and crash evidence. |
| W-002 | Encrypted write support | Requires security milestone and write lab. |
| W-003 | Repair/format | Requires mature read/write model. |
| W-004 | Password recovery, cracking, or access-control bypass | Explicitly out of scope. |

## Current release interpretation

v0.17.0 is a synthetic, uncompiled source package. It is suitable for continuing development and first compilation, not for end-user APFS access.


## v0.18.0 MoSCoW additions

### Should

- `M-026`: Read-only VFS facade crate for future platform adapters.
- `M-027`: Windows read-only adapter readiness scaffold and smoke-test scripts.
- `M-028`: Redacted diagnostics bundle generator from JSON reports.
- `M-029`: Fuzz target scaffold for parser entry points.
- `M-030`: Cargoless API-map and next-loop planning tools.

### Won't yet

- Production WinFsp FFI mount lifecycle.
- Raw physical-device access.
- Write support.
- Encryption bypass, repair, or format.


## v0.18.0 MoSCoW additions

### Must

- M-033 Safety case and safety-case preflight must document hazards, mitigations, evidence, and non-goals before source handoff.

### Should

- M-031 `apfs doctor --json` should aggregate implemented read-only readiness surfaces and blockers.
- M-032 CLI/API/source-metrics snapshots should be regenerated during cargoless preflight.
- M-027 redacted diagnostics export should remain host-output-only and exclude raw blocks, file contents, secrets, and full source paths.

### Won't yet

- Doctor and diagnostics export do not imply production APFS compatibility.
- Diagnostics export must not include raw APFS blocks, passwords, recovery keys, or personal full paths by default.


## v0.20.0 MoSCoW additions

### Should

- `M-034` — Unicode/case policy command scaffold.
- `M-035` — Xattr/resource-fork readiness report.
- `M-036` — Sparse/clone readiness report.
- `M-037` — Compression readiness report.
- `M-038` — Snapshot and volume-role readiness report.

### Won't yet

- Production xattr/resource fork extraction.
- Production sparse/clone extent semantics.
- Production compression decompression.
- Snapshot views or snapshot mutation.

## v0.20.0 quality addition

### Should

- `M-039` — Generate `FEATURE_READINESS.md` and `FEATURE_READINESS.json` so advanced feature scaffolds remain reviewable before Rust compilation.

## v0.20.0 version-governance addition

### Should

- `M-040` — Check current package version consistency across registries, root ledgers, Conductor history, and generated snapshots before handoff.


## v0.20.0 Handoff, platform, release, and adapter scaffolds

- **M-041** — Cargo error triage into Codev/Conductor task stubs. **Should**.
- **M-042** — Local handoff runbook and ready-for-local checklist. **Must**.
- **M-043** — Platform setup documentation for Rust, macOS APFS fixtures, Windows WinFsp, FUSE, Android, and ChromeOS. **Must**.
- **M-044** — Release signing, SBOM, provenance, installer, and winget scaffold. **Should**.
- **M-045** — FUSE/Linux/macOS/ChromeOS adapter readiness crate. **Could**.
- **M-046** — Android access layer readiness crate. **Could**.
- **M-047** — Software-encryption read readiness crate. **Should**.
- **M-048** — Image-only write-lab readiness crate. **Could**.
- **M-049** — Handoff release-preflight integration. **Should**.

These are scaffolds/readiness layers unless explicitly marked production-ready. They do not implement APFS media mutation, decryption, repair, format, or live mount lifecycle.

## v0.21.0 Local handoff candidate additions

### Must

- `M-053` — Local first-run triage checklist.
- `M-054` — Known uncompiled risks registry.
- `M-058` — Local handoff preflight integration.

### Should

- `M-050` — Reproducible Rust toolchain and Cargo QA configuration.
- `M-051` — Devcontainer and tool-version bootstrap.
- `M-052` — Pre-commit, typos, markdownlint, and TOML quality configs.
- `M-055` — Local environment doctor.
- `M-056` — Repository manifest generator.
- `M-057` — Handoff status reporter.

### Won't yet

- These handoff additions do not prove that the Rust workspace compiles.
- They do not implement APFS production traversal, Windows mounting, write support, repair, or format.


## v0.22.0 local handoff hardening

Added M-059 through M-064: local compile-loop orchestration, cargoless Cargo workspace audit, macOS fixture dry-run validation, WinFsp callback matrix, production-gap reporting, and batched-loop stop criteria. These do not reduce the 9 Windows read-only MVP production blockers because those require local Rust/macOS/Windows execution.


## v0.22.0 handoff hardening requirements

- **M-059** — Local compile loop orchestrator. MoSCoW: Should. Track: `0059-local-compile-loop-orchestrator`. Status: scaffold/tooling implemented.
- **M-060** — Cargoless Cargo workspace audit. MoSCoW: Should. Track: `0060-cargo-workspace-audit`. Status: scaffold/tooling implemented.
- **M-061** — macOS APFS fixture dry-run validator. MoSCoW: Should. Track: `0061-macos-fixture-dry-run-validator`. Status: scaffold/tooling implemented.
- **M-062** — WinFsp read-only callback contract matrix. MoSCoW: Should. Track: `0062-winfsp-callback-contract-matrix`. Status: scaffold/tooling implemented.
- **M-063** — Production gap report generator. MoSCoW: Should. Track: `0063-production-gap-report`. Status: scaffold/tooling implemented.
- **M-064** — Batched loop policy and local stop criteria. MoSCoW: Should. Track: `0064-batched-loop-policy`. Status: scaffold/tooling implemented.

## v0.23.0 Current-environment hardening requirements

| ID | MoSCoW | Requirement | Status |
|---|---|---|---|
| M-065 | Should | Current environment capability inventory: Inventory available tools and classify what checks can run before Rust/macOS/Windows. | Implemented scaffold |
| M-066 | Should | Current-environment remaining-work classifier: Separate production blockers from current-environment-completable work. | Implemented scaffold |
| M-067 | Should | Cargoless Cargo dependency graph generator: Generate a path-dependency graph without cargo metadata. | Implemented scaffold |
| M-068 | Should | Synthetic negative fixture generator: Create safe parser-refusal fixtures from synthetic NXSB images. | Implemented scaffold |
| M-069 | Should | Test/control matrix generator: Map validation commands to current, Rust, macOS, and Windows phases. | Implemented scaffold |
| M-070 | Should | Handoff archive audit generator: Generate file-level archive audit with sizes and checksums. | Implemented scaffold |

## v0.25.0 current-environment completion additions

### Should

- `M-071` — APFS offset and synthetic fixture byte-layout audit.
- `M-072` — Cargoless golden-output expectation generator.
- `M-073` — Cargoless dependency license/policy audit.
- `M-074` — Backlog issue export for local/platform blockers.
- `M-075` — Current-environment self-test orchestrator.

### Won't yet

These tools do not replace Rust compilation, real macOS APFS fixture validation, or Windows WinFsp mount testing.

## v0.25.0 current-environment completion additions

### Should

- `M-077` — APFS offset and synthetic fixture byte-layout audit.
- `M-078` — Cargoless golden-output expectation generator.
- `M-079` — Cargoless dependency license/policy audit.
- `M-080` — Backlog issue export for local/platform blockers.
- `M-081` — Current-environment self-test orchestrator.

### Won't yet

These tools do not replace Rust compilation, real macOS APFS fixture validation, or Windows WinFsp mount testing.

## v0.25.0 additional current-environment audit requirements

- `M-076` — Current-environment completion report.
- `M-071` — Markdown internal link audit.
- `M-072` — Shell and macOS fixture script safety audit.
- `M-073` — Cargoless smoke suite aggregator.
- `M-074` — Documentation index consistency audit.
- `M-075` — Fixture coverage gap report.


## v0.26.0 Local Handoff Control Requirements

### Should

- M-082: Tool capability matrix and fallback command planner should identify what can run here and what needs local/platform tools.
- M-083: Cargoless Rust static lint should identify obvious source-shape, risky-token, and duplicate-function issues before Cargo is available.
- M-084: Package integrity audit should verify SHA256SUMS and source file inventory before handoff.
- M-085: MVP blocker tasklist should convert the 9 MVP blockers into local next actions.
- M-086: Agent handoff brief should summarize current state for humans and coding agents.
- M-087: Local command plan should separate current-environment, Rust-local, macOS-fixture, and Windows-WinFsp phases.

### Won't yet

- These tools do not replace cargo test, macOS APFS fixture validation, or Windows WinFsp smoke testing.

## v0.26.0 current-environment closure additions

- `M-088` — Source debt report.
- `M-089` — Production claim guard.
- `M-090` — Handoff manifest verifier.
- `M-091` — MVP blocker dependency DAG.
- `M-092` — Local migration command generator.
- `M-093` — Current-environment final report.


## v0.27.0 quality and documentation requirements

- **M-094 Must:** strict CI quality gates are configured.
- **M-095 Must:** 90% line coverage gate is configured for Rust CI.
- **M-096 Should:** property/Hypothesis test scaffolds exist.
- **M-097 Should:** mutation-testing scaffold exists.
- **M-098 Should:** profiling benchmark scaffold exists.
- **M-099 Should:** Astro 7 documentation site scaffold exists.
- **M-100 Should:** QA readiness report and test scaffold audit exist.

## v0.27.0 quality/docs hardening requirements

### Should

- **M-094** Strict CI quality gates with format, lint, nextest, 90% coverage, fuzz, mutation, and supply-chain checks.
- **M-095** Unit, integration, E2E, property, fuzz, mutation, and profiling test strategy.
- **M-096** Property/Hypothesis-style testing scaffold.
- **M-097** Mutation testing scaffold.
- **M-098** Profiling and benchmark scaffold.
- **M-099** Astro 7 documentation site scaffold.
- **M-100** Documentation site audit and quality gate.
- **M-101** CI quality gate audit.
- **M-102** Extended current-environment tool inventory.
- **M-103** Quality and docs handoff integration.

### Current environment limitation

These requirements are configured and audited without Rust/Cargo. Enforcing the Rust gates requires local/CI execution with Rust installed.

## v0.27.0 strict quality, profiling, and documentation requirements

### Must

- Configure CI for unit, integration, and end-to-end tests.
- Configure a `cargo llvm-cov` line coverage gate of at least 90%.
- Configure property/hypothesis-style tests through Rust `proptest`, with optional Python Hypothesis tests for fixtures.
- Configure parser fuzz smoke tests.
- Configure mutation testing with `cargo mutants` for scheduled/manual CI.
- Configure cargoless checks that validate the quality-gate setup before Rust is available.

### Should

- Configure Criterion benchmarks and profiling workflows.
- Use Astro 7 with Starlight for the documentation site.
- Keep docs build separate from APFS parser/library safety.

### Won't yet

- Claim coverage has passed until the workspace is compiled and `cargo llvm-cov` runs locally or in CI.
- Claim mutation/property/fuzz results until those tools execute on a Rust-enabled machine.


## v0.28.0 Quality, Docs, and Test Evidence Requirements

- **M-104 Must/Should:** Docs package and Astro 7 package audit must verify local `astro@7.0.2` pinning, scripts, and documentation-site files.
- **M-105 Must:** GitHub workflow policy audit must verify strict CI gates, least-privilege permissions, coverage, fuzz, mutation, profiling, and docs workflows.
- **M-106 Must:** Test inventory report must cover unit, integration, end-to-end, Rust property, Python Hypothesis-style, fuzz, mutation, benchmark, and coverage scaffolds.
- **M-107 Should:** Hypothesis strategy audit must verify Python property tests remain present as a companion to Rust `proptest`.
- **M-108 Should:** Profiling budget audit must verify benchmark files, profile plan, and profiling workflow.
- **M-109 Must:** Quality gate evidence ledger must distinguish configured gates from gates not yet executed because Rust/Cargo/npm install are unavailable here.


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
- `M-119` — Executed CI, logging, profiling, and dynamic versioning hardening.

## Post-v0.29.0 CI, logging, profiling, and dynamic versioning requirements

- **M-119 Should:** Required CI must execute dynamic version metadata, profiling
  audit, release automation audit, and aggregate bleeding-edge hardening checks.
- **M-119 Should:** `apfs version --json` must expose workspace version, package
  version, git SHA, target, profile, and read-only/no-media-write metadata.
- **M-119 Should:** CLI logging must be opt-in, redacted, emitted to stderr, and
  covered by tests without polluting JSON stdout.
- **M-119 Should:** Profiling workflows must run Criterion benchmarks for both
  `apfs-core` and `apfs-types`.
- **M-119 Must:** Release automation dry-runs must keep publishing disabled until
  the accepted MVP release gates pass.
