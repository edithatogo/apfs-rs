# Conductor Context Management Skill

Version: 0.29.0

Before changing this repository, read `conductor/product.md`, `conductor/tech-stack.md`, `conductor/workflow.md`, `conductor/tracks.md`, `conductor/history.md`, the active track, and matching Codev spec/plan/review.

Current historical ledger: 163 Conductor track directories through M-140, with M-121, M-122, and M-123 now executed and M-124 through M-140 remaining open roadmap tracks.

Rules:

- Preserve append-only track history.
- Do not claim production APFS support for scaffolded or synthetic-only capabilities.
- Do not claim CI coverage, mutation, fuzzing, profiling, or docs builds have passed until the relevant tools run locally/CI.
- Astro 7/Starlight documentation lives under `docs-site/` and is documentation-only.
- Do not add APFS media writes, raw physical-device access, encryption bypass, repair, format, or live mount lifecycle without accepted specs and platform validation.


## v0.28.0 quality evidence layer

Before changing test, CI, profiling, or docs-site configuration, read conductor tracks `0104-docs-package-audit` through `0109-quality-gate-evidence-ledger` and preserve the distinction between configured gates and executed gates.


## v0.29.0 note

Before modifying CI/CD, release, docs, or supply-chain automation, read the v0.29.0 tracks `0110` through `0118` and mature-release roadmap tracks `0120` through `0140` and preserve the configured-versus-executed evidence distinction. Tracks `0121-real-macos-apfs-fixture-execution` and `0122-real-fixture-feedback-promotion` now record executed evidence; keep their follow-up boundary distinct from `0123` through `0140`.
