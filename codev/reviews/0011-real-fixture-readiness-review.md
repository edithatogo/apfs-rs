# Review 0011: Real APFS Fixture Readiness Harness

Document version: 0.11.0  
Status: Implementation review  
Codev phase: Review

## What changed

Added a macOS-only image fixture harness and manifest validation path:

- `tools/macos/create_real_apfs_fixture.sh`.
- `tools/macos/README.md`.
- `tools/compare_inspect_to_manifest.py`.
- `fixtures/real/README.md`.
- `fixtures/manifests/macos-minimal-apfs-001.template.json`.
- `apfs-test` fixture manifest structs and validation helper.
- `xtask fixture-manifest-check`.

## Safety result

The fixture generator creates an APFS sparse image only. It does not target physical disks and must not be used with personal data.

## Known limitations

The harness cannot be executed in this environment because macOS tools and Rust/Cargo are unavailable. It is ready to run once the package is moved to a Mac/Rust-enabled computer.

## Next action

Generate `macos-minimal-apfs-001` on macOS and use it to validate the current APFS inspect path against real Apple-created metadata.
