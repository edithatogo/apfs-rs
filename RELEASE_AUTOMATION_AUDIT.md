# Release Automation Audit

Status: `passed`.

| Check | OK |
|---|---:|
| release-plz.toml | true |
| dist-workspace.toml | true |
| RELEASE_AUTOMATION.md | true |
| .github/workflows/release-automation.yml | true |
| workflow invokes cargo install cargo-dist | true |
| workflow invokes cargo install release-plz | true |
| workflow invokes dist plan --allow-dirty | true |
| workflow invokes GIT_TOKEN: ${{ secrets.GITHUB_TOKEN }} | true |
| workflow invokes release-plz release --dry-run --allow-dirty --config release-plz.toml | true |
| workflow invokes cargo run -p xtask -- release-automation-audit | true |
| no placeholder echo | true |
| cargo-dist workspace table | true |
| cargo-dist cargo workspace member | true |
| cargo-dist dist table | true |
| cargo-dist version pinned | true |
| release-plz publish disabled | true |
