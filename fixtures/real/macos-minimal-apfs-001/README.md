# macOS real fixture: `macos-minimal-apfs-001`

This directory contains the first real Apple-created APFS fixture recorded for
the repository's macOS evidence track.

## Contents

- `macos-minimal-apfs-001.sparseimage` - APFS sparse image generated on macOS.
- `manifest.json` - redacted fixture manifest.
- `file-hashes.sha256` - file hash inventory for the mounted fixture content.
- `macos-oracle.redacted.txt` - macOS oracle output captured during fixture execution.

## Safety boundary

- Image-only fixture evidence.
- No raw physical-device writes.
- No encryption bypass, repair, or format behavior.

## Regeneration

Use `tools/macos/create_real_apfs_fixture.sh` on macOS to regenerate the fixture
and then run:

```bash
cargo xtask fixture-manifest-check fixtures/real/macos-minimal-apfs-001/manifest.json
python3 tools/compare_inspect_to_manifest.py target/inspect-real.json fixtures/real/macos-minimal-apfs-001/manifest.json
```

The inspect JSON input is expected to come from a macOS run of:

```bash
cargo run -p apfs-cli -- inspect --json fixtures/real/macos-minimal-apfs-001/macos-minimal-apfs-001.sparseimage
```
