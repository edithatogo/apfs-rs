# Release Automation Scaffold

Version: 0.29.0

The repository is prepared for a Rust release pipeline using cargo-dist and release-plz after local compilation succeeds.

## Intended workflow

1. Release-plz opens version/changelog PRs from conventional commits.
2. cargo-dist plans and builds release artifacts.
3. GitHub Actions generate checksums, SBOM, and attestations.
4. Windows signing and winget publication run only after read-only MVP validation.

No production release is claimed until Rust tests, real APFS fixtures, and platform smoke tests pass.
