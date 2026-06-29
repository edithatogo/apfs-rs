# Plan: Executed CI, logging, profiling, and dynamic versioning hardening

1. Add dynamic CLI version metadata.
2. Add opt-in redacted JSON logging.
3. Cover versioning and logging with CLI tests.
4. Wire version/profiling/release/bleeding-edge audits into `xtask` and CI.
5. Expand profiling workflow to both core and parser Criterion benches.
6. Replace release automation placeholder with cargo-dist/release-plz dry-run commands.
7. Update Astro/root documentation.
8. Validate locally and in GitHub Actions.
