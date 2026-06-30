# MVP Blocker Tasklist

Remaining MVP blockers: 5

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
