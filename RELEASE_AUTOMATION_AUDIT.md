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
| workflow invokes dist plan | true |
| workflow invokes release-plz update | true |
| workflow invokes cargo run -p xtask -- release-automation-audit | true |
| no placeholder echo | true |
| cargo-dist workspace table | true |
| cargo-dist version pinned | true |
| release-plz publish disabled | true |
