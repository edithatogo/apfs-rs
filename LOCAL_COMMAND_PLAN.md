# Local Command Plan

Generated: 2026-06-30T15:05:55.395997+00:00

## Current Environment

| Available here | Command |
|---:|---|
| yes | `python3 tools/cargoless_smoke_suite.py` |
| yes | `python3 tools/tool_capability_matrix.py` |
| yes | `python3 tools/rust_static_lint.py` |
| yes | `python3 tools/package_integrity_audit.py` |
| yes | `python3 tools/mvp_blocker_tasklist.py` |
| yes | `python3 tools/agent_handoff_brief.py` |

## Local Rust

| Available here | Command |
|---:|---|
| yes | `cargo fmt --all -- --check` |
| yes | `cargo test --workspace` |
| yes | `cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| yes | `cargo xtask registry-check` |
| yes | `cargo xtask conductor-check` |
| yes | `cargo xtask safety-check` |

## Macos Fixture

| Available here | Command |
|---:|---|
| yes | `./tools/macos/create_real_apfs_fixture.sh` |
| yes | `cargo xtask fixture-manifest-check fixtures/real/macos-minimal-apfs-001/manifest.json` |
| yes | `cargo run -p apfs-cli -- inspect --json fixtures/real/macos-minimal-apfs-001/macos-minimal-apfs-001.sparseimage > inspect.json` |

## Windows Winfsp

| Available here | Command |
|---:|---|
| yes | `cargo run -p apfs-cli -- winfsp-callback-matrix --json` |
| no | `# install WinFsp on a dedicated Windows test VM before live mount work` |
