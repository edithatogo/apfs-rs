# Plan M-119: Executed CI, logging, profiling, and dynamic versioning hardening

1. Add dynamic build metadata to `apfs-cli`.
2. Add redacted opt-in operational logging.
3. Add CLI tests for dynamic version metadata and logging separation.
4. Wire version, profiling, release automation, and aggregate hardening audits
   into `xtask` and required CI.
5. Expand profiling workflow coverage to both core and parser benchmarks.
6. Replace placeholder release automation with cargo-dist and release-plz dry-run
   commands.
7. Update Astro and root documentation.
8. Update Codev and Conductor ledgers.
9. Run local checks and push to the remote repository.
