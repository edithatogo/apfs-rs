# Next Loop Plan

Document version: 0.25.0

## Recommended next environment

Move to a Rust-enabled machine. Run:

```bash
python3 tools/current_env_selftest.py
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

If Cargo fails, use:

```bash
cargo test --workspace 2>&1 | tee target/cargo-test.log
python3 tools/cargo_error_to_tracks.py target/cargo-test.log target/cargo-triage
```
