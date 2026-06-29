# Profiling Workspace

Use this directory for profiling notes, flamegraphs, and benchmark outputs that should not be committed wholesale.

Configured benchmark:

```bash
cargo bench -p apfs-core --bench inspect_synthetic
cargo bench -p apfs-types --bench nx_superblock_bench
```

Optional local tools:

```bash
cargo install flamegraph
cargo flamegraph -p apfs-cli -- inspect --json fixtures/synthetic-file-preview.img
```

CI runs the Criterion benches on a schedule and uploads `target/criterion` plus
the profiling audit artifacts. Release candidates should keep the synthetic
fixture targets above under review before any real-fixture profiling is promoted.
