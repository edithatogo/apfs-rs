## [0.27.0] - QA and documentation hardening

- Added strict CI quality gates, 90% coverage policy, property/Hypothesis tests, mutation testing scaffold, profiling benchmark scaffold, Astro 7 docs-site scaffold, and QA readiness audit.
- Added cargoless checks for quality gates, docs-site config, and test-scaffold presence.

# APFS-RS Codev Changelog

## [0.23.0] - 2026-06-25

### Added

- M-065 current environment capability inventory.
- M-066 current-environment remaining-work classifier.
- M-067 cargoless Cargo dependency graph generator.
- M-068 synthetic negative fixture generator.
- M-069 test/control matrix generator.
- M-070 handoff archive audit generator.

### Validation

- Added more cargoless checks that can run before Rust/Cargo are available.
- Preserved full Conductor development history through M-070.


## v0.22.0

- Added M-059 through M-064 for local compile-loop orchestration, Cargo workspace audit, macOS fixture dry-run validation, WinFsp callback matrix, production-gap reporting, and batched-loop stop criteria.
- Updated capability and safety-gate registries for local handoff hardening.
- Preserved read-only/no-media-mutation safety posture.

# APFS-RS Codev Changelog

## v0.21.0

- Added M-050 through M-058 local handoff candidate hardening.
- Added reproducible Rust toolchain and Cargo QA configs.
- Added devcontainer, pre-commit, typos, markdownlint, TOML, and editor configs.
- Added local environment doctor, handoff status reporter, repository manifest generator, and known uncompiled risks registry.
- Updated Codev and Conductor histories through track 0058.
- Integrated the new handoff checks into the cargoless release preflight.

## v0.18.0

- Added M-031 doctor/readiness CLI.
- Added M-032 CLI/API/source metrics snapshots.
- Added M-033 safety case and safety-case preflight.
- Updated Conductor validation to discover tracks dynamically.
- Added redacted diagnostics export through CLI.

## v0.18.0

- Added M-026 Windows read-only mount-plan facade.
- Added M-027 redacted diagnostics bundle CLI.
- Added M-028 CLI contract static check.
- Added M-029 context integrity static check.
- Updated Conductor historical tracks through 0029.


## v0.17.0

- Added M-022 synthetic fixture oracle and smoke-test harness.
- Added M-023 traceability matrix generation across requirements, Codev, and Conductor.
- Added M-024 loop dashboard and machine-readable remaining-elements ledger.
- Added M-025 cargoless release-readiness/preflight bundle.
- Updated Conductor tracks through 0025 and expanded the conductor context-management skill instructions.
- Ran Python/static validation, synthetic fixture oracle checks, traceability generation, loop dashboard generation, and release preflight with SHA-256 manifest regeneration.

## v0.16.0

- Added synthetic `apfs stat` metadata report over the synthetic directory scaffold.
- Added safe host-side synthetic `apfs extract` preview that writes only to a requested destination directory, never to APFS media.
- Added `tools/precompile_static_check.py` and `cargo xtask precompile-check` for environments where Rust/Cargo is unavailable.
- Added Codev and Conductor tracks for M-019 through M-021.
- Fixed a duplicate parser-development function name in `apfs-types` that static validation would flag before compilation.


## v0.15.0

### Added

- Consolidated the v0.13 feedback-promotion history and v0.14 volume/mapped-object work into one Conductor ledger.
- Added M-016 synthetic filesystem root-tree directory-record parser.
- Added M-017 synthetic directory listing CLI with `apfs ls`.
- Added M-018 synthetic direct-block file preview with `apfs cat --name`.
- Added/updated Codev specs, plans, reviews, capability registry entries, safety gates, Conductor tracks, and remaining-elements ledger through M-018.

### Safety

- All new functionality is read-only and synthetic-fixture-only.
- No raw physical-device access, mount code, write support, repair, format, compression, or encryption bypass was added.

## v0.14.0

- Added synthetic APFS volume-superblock probing and mapped-object read reporting.

## v0.13.0

- Added real-fixture feedback promotion into generated Codev/Conductor task stubs.

## v0.12.0

- Added real-fixture feedback loop.

## v0.11.0

- Added real APFS fixture readiness harness.

## v0.10.0

- Added production-shaped B-tree cursor facade.

## v0.9.0

- Added object-map resolver facade and resolver-report CLI.

## v0.8.0

- Added bounded synthetic two-level OMAP B-tree traversal.

## v0.7.0

- Added checkpoint-map-backed synthetic multi-node OMAP lookup.

## v0.6.0

- Added single-node synthetic OMAP lookup CLI.

## v0.5.0

- Added B-tree node header/TOC probing.

## v0.4.0

- Added checkpoint-map parsing and early OMAP header probe.

## v0.3.0

- Added APFS Fletcher-64 checksum validation and checkpoint candidate scanning.

## v0.2.0

- Added GPT-wrapped APFS partition probe.

## v0.1.0

- Added initial Rust workspace and `apfs inspect --json` scaffold.


## v0.20.0

- Added M-034 through M-038 advanced feature readiness/policy scaffolds.
- Added `apfs-features` crate and CLI-facing path/feature readiness surfaces.
- Updated Codev and Conductor history through track 0038.


## v0.20.0

Added M-041 through M-049 for local handoff, cargo triage, platform setup, release/provenance scaffolding, adapter readiness, encryption readiness, and image-only write-lab readiness.

## v0.25.0

- Added M-071 through M-075 current-environment completion tooling.
- Added APFS offset audit, golden-output generator, dependency policy audit, backlog issue export, and current-environment self-test orchestrator.
- Updated Conductor history through tracks 0071-0075.
- Confirmed no production APFS blockers remain completable without Rust/macOS/Windows execution.

## v0.25.0

- Added M-077 through M-081 current-environment completion tooling.
- Added APFS offset audit, golden-output generator, dependency policy audit, backlog issue export, and current-environment self-test orchestrator.
- Confirmed no production APFS blockers are completable in this sandbox without Rust/macOS/Windows.


## [0.26.0] - 2026-06-25

### Added

- M-082 tool capability matrix and fallback command planner.
- M-083 cargoless Rust static lint.
- M-084 package integrity audit.
- M-085 MVP blocker tasklist generator.
- M-086 agent handoff brief generator.
- M-087 local command plan generator.

### Safety

- Reaffirmed that all remaining production blockers require local Rust/macOS/Windows validation and cannot be honestly completed in this environment.

## v0.27.0

- Added M-094 through M-101 for strict CI/CD quality gates, >=90% coverage configuration, property/hypothesis-style tests, mutation/fuzz/profiling scaffolds, testing infrastructure reporting, and Astro 7/Starlight documentation site adoption.
- Added docs-site scaffold and docs-site cargoless validation.
- Added quality-gate cargoless validation and CI workflow templates.
- Preserved full Conductor history through M-101.


## v0.28.0

Added M-104 through M-109 for docs package audit, GitHub workflow policy audit, test inventory reporting, Hypothesis strategy audit, profiling budget audit, and quality gate evidence reporting.


## v0.29.0

Added M-110 through M-118 for GitHub Actions hardening, action pinning/permissions audit, cargo-vet supply-chain policy, provenance verification, cargo-dist/release-plz automation, Scorecard/Dependency Review, Astro 7 docs hardening, benchmark regression readiness, and the bleeding-edge repo audit aggregator.
