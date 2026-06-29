# Local First Run

Version: 0.21.0

This repository is a source handoff candidate. It has not been compile-verified in the sandbox environment because Rust/Cargo were unavailable there.

## Step 0: inspect the environment

```bash
python3 tools/local_env_doctor.py --json target/local-env-doctor.json
python3 tools/precompile_static_check.py
python3 tools/config_sanity_check.py
python3 tools/handoff_status.py --write
```

## Step 1: install Rust if needed

Recommended first path:

```bash
rustup toolchain install stable
rustup component add rustfmt clippy
```

Alternative: open the devcontainer and let it install the Rust toolchain and helper tools.

## Step 2: run compile/lint/tests

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

If this fails, save the log and create task stubs:

```bash
cargo test --workspace 2>&1 | tee target/cargo-test.log
python3 tools/cargo_error_to_tracks.py target/cargo-test.log target/cargo-triage
```

## Step 3: run current synthetic checks

```bash
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-nxsb-block0.bin
cargo run -p apfs-cli -- doctor --json fixtures/synthetic-file-preview.img
cargo run -p apfs-cli -- stat --json fixtures/synthetic-file-preview.img --name hello.txt
```

## Step 4: generate the first real APFS fixture on macOS

Only use synthetic sparse images. Do not point scripts at physical disks.

```bash
./tools/macos/create_real_apfs_fixture.sh
cargo xtask fixture-manifest-check fixtures/real/macos-minimal-apfs-001/manifest.json
cargo run -p apfs-cli -- inspect --json fixtures/real/macos-minimal-apfs-001/macos-minimal-apfs-001.sparseimage > inspect.json
cargo xtask real-fixture-feedback inspect.json fixtures/real/macos-minimal-apfs-001/manifest.json target/real-fixture-feedback
cargo xtask promote-feedback target/real-fixture-feedback/real-fixture-feedback.json target/promoted-feedback-tasks
```

## Step 5: triage order

1. Build failures.
2. Test failures.
3. Synthetic fixture mismatches.
4. Real APFS fixture mismatch feedback.
5. Windows/WinFsp readiness.

Do not begin write support until the image-only write-lab spec is accepted and real read-only validation is stable.
