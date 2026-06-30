# APFS-RS Production Gap Report

Implemented/scaffolded: `140`
Windows read-only MVP blockers: `5`
Broader production items: `17`

## Windows read-only MVP blockers
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
- **ADMIN-R001**: Branch protection and required-check governance
- **ADMIN-R002**: Hosted Renovate lifecycle and dependency update governance
- **ADMIN-R003**: cargo-vet and dependency policy enforcement maturation
- **ADMIN-R004**: Mature release readiness dashboard and release train
