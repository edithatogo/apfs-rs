# Provenance and Attestation Verification

Version: 0.29.0

APFS-RS release artifacts should be verifiable by consumers.

## Planned release evidence

- SHA-256 checksums.
- SBOM in SPDX or CycloneDX format.
- GitHub artifact attestation.
- SLSA provenance where practical.
- Signed Windows binaries before broad Windows distribution.

## Consumer verification flow

```bash
gh attestation verify apfs-rs-*.zip --repo OWNER/apfs-rs
sha256sum -c SHA256SUMS.txt
```

This is a scaffold until the first GitHub release workflow runs.
