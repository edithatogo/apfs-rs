# Profiling Workspace

Use this directory for profiling notes, flamegraphs, and benchmark outputs that should not be committed wholesale.

Configured benchmark:

```bash
cargo bench -p apfs-core --bench inspect_synthetic
```

Optional local tools:

```bash
cargo install flamegraph
cargo flamegraph -p apfs-cli -- inspect --json fixtures/synthetic-file-preview.img
```
