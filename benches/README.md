# Benchmarks and Profiling

Run after the workspace compiles:

```bash
cargo bench --workspace
cargo flamegraph --bin apfs -- inspect --json fixtures/synthetic-btree-cursor.img
```

Initial benchmark targets:

- APFS inspect over synthetic NXSB fixture.
- GPT APFS probe.
- OMAP lookup.
- Directory listing.
- Safe extract preview.
