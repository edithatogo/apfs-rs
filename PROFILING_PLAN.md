
# Profiling Plan

Version: 0.27.0

## Goals

1. Keep APFS inspect fast on large images.
2. Prevent repeated metadata reads from dominating Windows read-only mount startup.
3. Measure B-tree traversal and object-map lookup before optimizing.

## Initial tools

- Criterion for deterministic microbenchmarks.
- `cargo bench` for local benchmark runs.
- `cargo flamegraph` later on Linux/macOS once available.
- Windows Performance Recorder later for WinFsp adapter profiling.

## Initial benchmarks

- `parse_nx_superblock_synthetic`.
- Future: checkpoint map parsing.
- Future: OMAP B-tree lookup.
- Future: directory listing.
- Future: file extent resolution.

## Current limitation

Profiling cannot execute in this environment because Rust/Cargo are unavailable. The benchmark scaffolds and CI workflow are present for local execution.
