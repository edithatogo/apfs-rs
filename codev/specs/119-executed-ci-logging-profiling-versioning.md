# Spec M-119: Executed CI, logging, profiling, and dynamic versioning hardening

Document version: 0.29.0  
Status: Implemented

## Goal

Promote the repo-hardening scaffolds into actively used CI and runtime surfaces:
dynamic CLI version metadata, opt-in redacted logging, enforced profiling audits,
scheduled Criterion benchmarks, release automation dry-runs, and Astro
documentation.

## Acceptance

- `apfs version --json` reports workspace version, package version, git SHA,
  target, and profile metadata.
- `--log-level` and `APFS_RS_LOG` emit redacted JSON logs to stderr without full
  source paths or APFS media contents.
- Required CI invokes version, profiling, release automation, and bleeding-edge
  audits.
- Scheduled profiling runs Criterion benchmarks for both `apfs-core` and
  `apfs-types`.
- Release automation invokes cargo-dist and release-plz dry-run planning with
  publishing disabled.
- Astro documentation describes the executed quality, logging, profiling,
  dependency, and versioning surfaces.
- No APFS media write, mount lifecycle, encryption bypass, or unsafe code is
  introduced.
