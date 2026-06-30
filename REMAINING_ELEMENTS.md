# APFS-RS Remaining Elements

Document version: 0.28.0

## Counts

- Implemented/scaffolded elements: **140**
- Remaining overall production/admin elements: **0**
- Remaining Windows read-only MVP production blockers: **0**
- Total remaining including MVP blockers: **0**
- Current-environment required items remaining: **0**

## Remaining Windows read-only MVP production blockers

None.

## Remaining broader/post-MVP production and mature-release elements

None.

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
| M-110 | GitHub Actions hardening with zizmor/actionlint policy | quality | implemented_python |
| M-111 | GitHub Actions pinning and permissions audit | quality | implemented_python |
| M-112 | cargo-vet supply-chain review policy | supply-chain | implemented_python |
| M-113 | SLSA and artifact attestation verification plan | release | implemented_python |
| M-114 | cargo-dist and release-plz automation scaffold | release | implemented_python |
| M-115 | OpenSSF Scorecard and dependency-review workflow scaffold | supply-chain | implemented_python |
| M-116 | Astro 7 documentation quality hardening | documentation | implemented_python |
| M-117 | Benchmark regression and optional CodSpeed readiness | quality | implemented_python |
| M-118 | Bleeding-edge repo hardening audit aggregator | quality | implemented_python |
| M-119 | Executed CI logging profiling and dynamic versioning hardening | quality | implemented |
| M-120 | Executed Rust workspace validation closeout | quality | implemented |
| M-121 | Real macOS APFS fixture execution closeout | fixture-readiness | implemented |
| M-122 | Real fixture feedback promotion | fixture-readiness | implemented |
| M-123 | Real APFS parser semantics correction | core-read | implemented |

## Current-environment conclusion

The remaining-work ledger is empty after reconciling the archived M-124 through M-140 closeouts with the capability registry and Conductor history.


## v0.29.0 Update

Added bleeding-edge repo hardening and automation scaffolds M-110 through M-118.

## Post-v0.29.0 hardening update

Added M-119 to execute dynamic versioning, redacted CLI logging, profiling audits,
release automation dry-runs, and Astro documentation updates in local and CI gates.


## Mature release roadmap update

Added Conductor/Codev tracks M-121 through M-140 for every remaining MVP, post-MVP, and mature-release governance item. M-120 closes the Rust workspace validation blocker with executed local and GitHub Actions evidence. M-121 closes the real macOS fixture execution step with generated sparse-image, manifest, hash, oracle, and feedback evidence. M-122 closes the real-fixture feedback promotion step by normalizing string and structured issue reports into generated task bundles. M-123 closes the real APFS parser checksum-semantics correction against the real fixture. M-124 through M-140 are now recorded as executed or archived closeouts.
