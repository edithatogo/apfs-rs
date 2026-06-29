# Final Localisation Plan

Version: 0.22.0

The remaining production work is now local/platform dependent. The recommended order is:

1. Run `python3 tools/local_env_doctor.py --json target/local-env-doctor.json`.
2. Run `python3 tools/local_compile_loop.py --execute --out-dir target/local-compile-loop`.
3. Triage and fix Rust compilation issues.
4. On macOS, run `python3 tools/macos_fixture_dry_run.py`, then generate `macos-minimal-apfs-001`.
5. Run `apfs inspect --json` on the real fixture and feed output into `real_fixture_feedback.py`.
6. Promote mismatches into Codev/Conductor task stubs.
7. Correct parser semantics against real APFS data.
8. Implement production checkpoint/OMAP/filesystem tree traversal.
9. Only then start the Windows WinFsp live adapter.
