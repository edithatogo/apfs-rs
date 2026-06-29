# macOS APFS Fixture Tools

These tools generate **synthetic APFS fixtures inside disk image files only**. They must not operate on physical disks.

## First real APFS fixture

On a macOS machine with `hdiutil`, `diskutil`, and `shasum` available:

```bash
./tools/macos/create_real_apfs_fixture.sh
```

The script creates a small APFS sparse image, mounts it, writes deterministic synthetic files, records hashes and macOS oracle command output, detaches the image, and writes a redacted fixture manifest.

The generated fixture is intended for parser validation only. It must not contain personal data or secrets.
