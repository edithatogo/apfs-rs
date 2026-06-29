# Supply Chain Policy

Version: 0.29.0

APFS-RS consumes untrusted filesystem images, so dependency provenance matters.

## Required before public release

- `cargo deny check` for licenses, bans, advisories, and duplicate control.
- `cargo audit` for RustSec advisories.
- `cargo vet` for human-reviewed supply-chain trust records.
- GitHub Dependency Review for pull requests.
- OpenSSF Scorecard on a schedule.
- SBOM generation and artifact attestations on release.

## Review tiers

- Runtime dependencies: full dependency review.
- Dev/test dependencies: lightweight review unless used in release workflows.
- Crypto, FFI, compression, filesystem mount, and write-lab dependencies: security-maintainer review.
