# Review 0001: Bootstrap Scaffolding & E2E Validation Review

Document version: 0.1.1  
Status: Accepted  
Codev phase: Review

## What changed

Validated and hardened the initial scaffolding for M-001:
- Resolved dev-dependency issues in [crates/apfs-cli/Cargo.toml](file:///Volumes/PortableSSD/apfs-rs/crates/apfs-cli/Cargo.toml) by introducing explicit declarations for `assert_cmd`, `predicates`, and `tempfile`.
- Resolved path-resolution bugs in E2E validation test [e2e_synthetic_cli.rs](file:///Volumes/PortableSSD/apfs-rs/crates/apfs-cli/tests/e2e_synthetic_cli.rs) using workspace relative paths.
- Confirmed compliance with the safety gates of the project.

## What is still missing

- Checksum validation logic integration.
- B-tree indexing and layout traversal.

## Safety result

Confirmed:
- No raw device writes.
- Strictly read-only behaviour.
