# Local Compile Loop

Version: 0.22.0

Use this when the package first lands on a Rust-enabled computer.

```bash
python3 tools/local_compile_loop.py --out-dir target/local-compile-loop
python3 tools/local_compile_loop.py --execute --out-dir target/local-compile-loop
```

The first command plans the compile loop. The second executes the Cargo checks if `cargo` is installed.

If a Cargo command fails:

```bash
python3 tools/cargo_error_to_tracks.py target/local-compile-loop/<failing-log>.log target/cargo-triage
```

This generates reviewable Codev/Conductor task stubs rather than trying to hide the failure.
