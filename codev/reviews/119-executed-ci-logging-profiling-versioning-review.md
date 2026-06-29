# Review M-119: Executed CI, logging, profiling, and dynamic versioning hardening

## What changed

- Added `apfs version --json` backed by build-time workspace version, package
  version, git SHA, target, and profile metadata.
- Added opt-in redacted JSON logging through `--log-level` and `APFS_RS_LOG`.
- Added CLI tests for version metadata and logging/stdout separation.
- Wired version, profiling, benchmark, release automation, and aggregate
  hardening audits into `xtask`, required CI, and local handoff.
- Expanded profiling to run both `apfs-core` and `apfs-types` Criterion benches.
- Replaced release automation placeholder text with cargo-dist and release-plz
  dry-run commands.
- Updated Astro and root documentation.

## Safety

Logging emits to stderr only, redacts full source paths, and records explicit
read-only/no-media-write flags. The change adds no APFS media writes, mount
lifecycle, encryption bypass, crypto dependencies, or unsafe code.
