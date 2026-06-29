# MVP Blocker Tasklist

Remaining MVP blockers: 9

## MVP-R001: Compile, lint, test, and fix the current workspace on Rust-enabled machines

Dependency: Rust-enabled machine

- Run cargo fmt/test/clippy
- Use cargo_error_to_tracks.py on failures
- Fix one compiler class at a time

## MVP-R002: Generate first real macOS APFS image and manifest

Dependency: macOS

- Run tools/macos/create_real_apfs_fixture.sh on macOS
- Validate manifest with cargo xtask fixture-manifest-check

## MVP-R003: Run real-fixture feedback loop and convert mismatches into tasks

Dependency: M-012/M-013 and real fixture

- Run real-fixture-feedback
- Run promote-feedback
- Review generated Codev/Conductor tasks

## MVP-R004: Correct APFS parser offsets/semantics against real APFS data

Dependency: real fixture feedback

- Compare inspect JSON to manifest
- Correct offsets/semantics
- Add regression fixture

## MVP-R005: Full checkpoint ring reconstruction

Dependency: corrected parser and real fixture

- Parse checkpoint descriptor ring
- Validate newest checkpoint selection
- Add corrupt/fallback fixtures

## MVP-R006: Production APFS object-map B-tree traversal

Dependency: corrected checkpoint and B-tree parsing

- Replace synthetic traversal internals with production B-tree cursor
- Validate OMAP lookup against real fixture

## MVP-R007: Production APFS filesystem tree record decoding and metadata/stat mapping

Dependency: production OMAP traversal

- Parse APFS volume superblock
- Decode root filesystem tree records
- Map metadata/stat fields

## MVP-R008: Production file extent resolution and extraction

Dependency: production filesystem records

- Resolve file extents
- Extract byte-identical regular files
- Compare SHA-256 hashes

## MVP-R009: Windows WinFsp read-only mount adapter, smoke tests, and packaging baseline

Dependency: stable read API

- Implement WinFsp adapter read-only callbacks
- Run Windows smoke tests
- Package baseline installer/portable zip
