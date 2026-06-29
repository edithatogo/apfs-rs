# APFS-RS Profiling Plan

Version: 0.27.0

## Goals

Profiling should be used to keep read-only APFS inspection and future extraction fast without weakening safety checks.

## Configured tools

- Criterion benchmark: `crates/apfs-core/benches/inspect_synthetic.rs`.
- Profiling workflow: `.github/workflows/profiling.yml`.
- Optional local tools once Rust is available: `cargo flamegraph`, `samply`, `hyperfine`.

## Initial benchmark

```bash
cargo bench -p apfs-core --bench inspect_synthetic
```

## Release gate

Performance regressions should be reviewed before Windows read-only MVP releases, especially for object-map lookup, B-tree traversal, and extraction paths.
