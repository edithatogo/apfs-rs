# Spec 0012: M-011 Real APFS Fixture Readiness Harness

Document version: 0.11.0  
Status: Implemented scaffold, pending macOS execution  
Codev phase: Specify

## Goal

Prepare the project to generate and validate the first real macOS-created APFS fixture without using personal disks, raw physical writes, or unsafe operations.

## Requirements

- Provide a macOS script that creates a synthetic APFS disk image.
- Create deterministic synthetic files inside the image.
- Record SHA-256 file hashes.
- Capture redacted macOS oracle output.
- Write a fixture manifest with feature flags and redaction metadata.
- Provide a comparison script for `apfs inspect --json` output versus the manifest.
- Keep all fixture operations image-only.

## Non-goals

- Physical-disk access.
- Windows mounting.
- File extraction implementation.
- Broad APFS compatibility claims.

## Acceptance

On macOS:

```bash
./tools/macos/create_real_apfs_fixture.sh
cargo run -p apfs-cli -- inspect --json fixtures/real/macos-minimal-apfs-001/macos-minimal-apfs-001.sparseimage > inspect.json
python3 tools/compare_inspect_to_manifest.py inspect.json fixtures/real/macos-minimal-apfs-001/manifest.json
cargo xtask fixture-manifest-check fixtures/real/macos-minimal-apfs-001/manifest.json
```
