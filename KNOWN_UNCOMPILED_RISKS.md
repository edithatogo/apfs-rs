# Known Uncompiled Risks

Version: 0.21.0

This file records likely issues to check once Rust/Cargo are available. It prevents false confidence from cargoless validation.

## Rust compilation risks

- Workspace lints may surface warnings as errors because `.cargo/config.toml` denies warnings.
- Generated/scaffold crates may need missing dependency declarations after the first real `cargo check`.
- CLI subcommand names and argument parsing may need minor clap fixes.
- Some scaffolds intentionally return readiness reports and may trigger clippy `must_use` or `uninlined_format_args` warnings.
- `jsonschema` API compatibility should be checked against the locked version selected by Cargo.

## APFS semantic risks

- Synthetic fixture offsets may not match real APFS metadata produced by current macOS.
- Checkpoint-map parsing is intentionally partial and must be corrected against real fixtures.
- B-tree traversal currently supports synthetic shapes only.
- Filesystem directory/file records are synthetic scaffolds, not production APFS file-system records.
- Fletcher checksum implementation must be validated against real APFS objects.

## Platform risks

- macOS fixture generation depends on `hdiutil`, `diskutil`, and local APFS behaviour.
- Windows read-only mount depends on WinFsp installation and API binding decisions.
- GitHub-hosted runners may not support privileged mount smoke tests.
- Devcontainer setup may need Docker Desktop or compatible container runtime.

## Handoff triage order

1. Run `python3 tools/local_env_doctor.py --json target/local-env-doctor.json`.
2. Run `python3 tools/precompile_static_check.py`.
3. Run `cargo fmt --all -- --check`.
4. Run `cargo test --workspace`.
5. Promote Cargo failures with `tools/cargo_error_to_tracks.py`.
6. Generate a real macOS fixture and run the feedback loop.
7. Promote real-fixture mismatches to Codev/Conductor tracks.
