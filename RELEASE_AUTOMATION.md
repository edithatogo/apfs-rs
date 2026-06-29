# Release Automation

Version: 0.29.0

The repository runs a dry-run Rust release pipeline using cargo-dist and release-plz after local compilation succeeds. Publishing remains disabled until the accepted APFS read-only MVP release gates pass.

## Intended workflow

1. Release-plz checks local version/changelog updates with `publish = false`.
2. cargo-dist runs `dist plan` against the workspace `dist-workspace.toml`.
3. GitHub Actions generate checksums and attestations in the release preflight workflow.
4. Windows signing and winget publication run only after read-only MVP validation.

No production release is claimed until Rust tests, real APFS fixtures, and platform smoke tests pass.
