# APFS-RS Production Gap Report

Implemented/scaffolded: `93`
Windows read-only MVP blockers: `9`
Broader production items: `8`

## Windows read-only MVP blockers
- **MVP-R001**: Compile, lint, test, and fix the current workspace on Rust-enabled machines — dependency: Rust-enabled machine
- **MVP-R002**: Generate first real macOS APFS image and manifest — dependency: macOS
- **MVP-R003**: Run real-fixture feedback loop and convert mismatches into tasks — dependency: M-012/M-013 and real fixture
- **MVP-R004**: Correct APFS parser offsets/semantics against real APFS data — dependency: real fixture feedback
- **MVP-R005**: Full checkpoint ring reconstruction — dependency: corrected parser and real fixture
- **MVP-R006**: Production APFS object-map B-tree traversal — dependency: corrected checkpoint and B-tree parsing
- **MVP-R007**: Production APFS filesystem tree record decoding and metadata/stat mapping — dependency: production OMAP traversal
- **MVP-R008**: Production file extent resolution and extraction — dependency: production filesystem records
- **MVP-R009**: Windows WinFsp read-only mount adapter, smoke tests, and packaging baseline — dependency: stable read API

## Beyond-MVP production items
- **POST-R007-PROD**: Production software-encryption read support
- **POST-R010-PROD**: Execute signed release, SBOM, provenance, installer, and winget publication
- **POST-R011-PROD**: Production Linux/macOS/ChromeOS/Android adapters and platform tests
- **POST-R015-PROD**: Execute image-only write lab with crash-injection evidence
- **POST-R016**: Windows write beta
- **POST-R017**: Repair
- **POST-R018**: Format
- **POST-R019**: Long-running fuzz/property/mutation/coverage hardening on CI
