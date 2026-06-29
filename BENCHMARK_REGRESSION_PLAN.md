# Benchmark Regression Plan

Version: 0.29.0

APFS-RS should prevent read-path performance regressions before Windows mounting is exposed.

## Benchmark classes

- Parser microbenchmarks for object header, NXSB, checkpoint map, OMAP, B-tree node parsing.
- Synthetic fixture end-to-end inspect/lookup/list/cat benchmarks.
- Future real APFS fixture read benchmarks.
- Windows mount smoke timing once WinFsp is implemented.

## Planned gates

- Criterion benchmark baselines.
- Optional CodSpeed/GitHub PR regression reporting.
- Profiling artifacts for release candidates.

## Executed gates

- Scheduled CI runs `cargo bench -p apfs-core --bench inspect_synthetic`.
- Scheduled CI runs `cargo bench -p apfs-types --bench nx_superblock_bench`.
- Required CI runs profiling plan and budget audits so benchmark coverage cannot silently disappear.
