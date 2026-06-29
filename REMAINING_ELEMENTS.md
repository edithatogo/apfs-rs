# APFS-RS Remaining Elements

Document version: 0.28.0

## Counts

- Implemented/scaffolded elements: **109**
- Remaining overall production elements: **8**
- Remaining Windows read-only MVP production blockers: **9**
- Total remaining including MVP blockers: **17**
- Current-environment required items remaining: **0**

## Remaining Windows read-only MVP production blockers

| ID | Title | Dependency |
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

## Remaining broader/post-MVP production elements

| ID | Title |
|---|---|
| POST-R007-PROD | Production software-encryption read support |
| POST-R010-PROD | Execute signed release, SBOM, provenance, installer, and winget publication |
| POST-R011-PROD | Production Linux/macOS/ChromeOS/Android adapters and platform tests |
| POST-R015-PROD | Execute image-only write lab with crash-injection evidence |
| POST-R016 | Windows write beta |
| POST-R017 | Repair |
| POST-R018 | Format |
| POST-R019 | Long-running fuzz/property/mutation/coverage hardening on CI |

## Recent implemented or scaffolded elements

| ID | Title | Category | Status |
|---|---|---|---|
| M-090 | Handoff manifest verifier | handoff | implemented_python |
| M-091 | MVP blocker dependency DAG | planning | implemented_python |
| M-092 | Local migration command generator | handoff | implemented_python |
| M-093 | Current-environment final report | handoff | implemented_python |
| M-094 | Strict CI/CD quality gates with >=90% coverage target | quality | configured_not_executed |
| M-095 | Unit integration e2e property mutation fuzz test scaffolds | quality | configured_not_executed |
| M-096 | Profiling and benchmark scaffold | quality | configured_not_executed |
| M-097 | Astro 7 and Starlight documentation site scaffold | documentation | configured_not_executed |
| M-098 | Docs-site cargoless validation and CI workflow | documentation | implemented_python |
| M-099 | Testing infrastructure report generator | quality | implemented_python |
| M-100 | Quality gate static checker | quality | implemented_python |
| M-101 | Documentation build policy and Astro 7 adoption decision | documentation | documented |
| M-102 | Extended current-environment tool inventory | handoff | implemented_python_scaffold |
| M-103 | Quality and docs handoff integration | handoff | implemented_python_scaffold |
| M-104 | Docs package and Astro 7 package audit | documentation | implemented_python |
| M-105 | GitHub workflow policy audit for strict quality gates | quality | implemented_python |
| M-106 | Test inventory report across unit/integration/e2e/property/fuzz/mutation/profiling | quality | implemented_python |
| M-107 | Hypothesis strategy audit for Python property tests | quality | implemented_python |
| M-108 | Profiling budget and benchmark audit | quality | implemented_python |
| M-109 | Quality gate evidence ledger | handoff-quality | implemented_python |

## Current-environment conclusion

All required current-environment-completable controls are scaffolded. Remaining production work requires Rust/Cargo, macOS APFS tooling, Windows/WinFsp, or long-running CI execution.


## v0.29.0 Update

Added bleeding-edge repo hardening and automation scaffolds M-110 through M-118.
