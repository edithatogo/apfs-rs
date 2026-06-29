# APFS-RS Remaining Elements

Document version: 0.28.0

## Counts

- Implemented/scaffolded elements: **123**
- Remaining overall production/admin elements: **9**
- Remaining Windows read-only MVP production blockers: **5**
- Total remaining including MVP blockers: **17**
- Current-environment required items remaining: **0**

## Remaining Windows read-only MVP production blockers

| ID | Track | Title | Dependency |
|---|---|---|---|
| MVP-R005 | M-124 | Full checkpoint ring reconstruction | corrected parser and real fixture |
| MVP-R006 | M-125 | Production APFS object-map B-tree traversal | corrected checkpoint and B-tree parsing |
| MVP-R007 | M-126 | Production APFS filesystem tree record decoding and metadata/stat mapping | production OMAP traversal |
| MVP-R008 | M-127 | Production file extent resolution and extraction | production filesystem records |
| MVP-R009 | M-128 | Windows WinFsp read-only mount adapter, smoke tests, and packaging baseline | stable read API |

## Remaining broader/post-MVP production and mature-release elements

| ID | Track | Title |
|---|---|---|
| POST-R007-PROD | M-129 | Production software-encryption read support |
| POST-R010-PROD | M-130 | Execute signed release, SBOM, provenance, installer, and winget publication |
| POST-R011-PROD | M-131 | Production Linux/macOS/ChromeOS/Android adapters and platform tests |
| POST-R015-PROD | M-132 | Execute image-only write lab with crash-injection evidence |
| POST-R016 | M-133 | Windows write beta |
| POST-R017 | M-134 | Repair |
| POST-R018 | M-135 | Format |
| POST-R019 | M-136 | Long-running fuzz/property/mutation/coverage hardening on CI |
| ADMIN-R001 | M-137 | Branch protection and required-check governance |
| ADMIN-R002 | M-138 | Hosted Renovate lifecycle and dependency update governance |
| ADMIN-R003 | M-139 | cargo-vet and dependency policy enforcement maturation |
| ADMIN-R004 | M-140 | Mature release readiness dashboard and release train |

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

All required current-environment-completable controls are scaffolded. Remaining production work requires Rust/Cargo, macOS APFS tooling, Windows/WinFsp, or long-running CI execution.


## v0.29.0 Update

Added bleeding-edge repo hardening and automation scaffolds M-110 through M-118.

## Post-v0.29.0 hardening update

Added M-119 to execute dynamic versioning, redacted CLI logging, profiling audits,
release automation dry-runs, and Astro documentation updates in local and CI gates.


## Mature release roadmap update

Added Conductor/Codev tracks M-121 through M-140 for every remaining MVP, post-MVP, and mature-release governance item. M-120 closes the Rust workspace validation blocker with executed local and GitHub Actions evidence. M-121 closes the real macOS fixture execution step with generated sparse-image, manifest, hash, oracle, and feedback evidence. M-122 closes the real-fixture feedback promotion step by normalizing string and structured issue reports into generated task bundles. M-123 closes the real APFS parser checksum-semantics correction against the real fixture. M-124 through M-140 remain reviewed and open.
