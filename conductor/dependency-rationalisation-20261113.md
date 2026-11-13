# Rust dependency rationalisation — apfs-rs (2026-11-13)

## Repository-specific decisions

- Verified: workspace already standardises Serde, Anyhow, Clap, Thiserror, Proptest and Criterion; no blanket adoption required.
- Retain filesystem-specific block-device, VFS, crypto and write-lab implementations, strict read/write safety and fuzz fixtures. Regex or Polars are not substitutes for binary APFS layout parsing.
- Evaluate Bytes only for measured streaming/buffer lifetime improvement; Crossbeam/Rayon only for bounded independent block verification after deterministic ordering and capacity controls.
- Extend existing Proptest for malformed container records, checksums, offset arithmetic, crash consistency and read-only/write-lab separation. Extend Criterion on fixed synthetic image fixtures.
- Tokio/Reqwest/SQLx/PyO3/Ratatui/Syn/Quote are opt-in at adapter or macro boundaries only; no async runtime in synchronous core by policy alone.

## Evidence and implementation gates

- Status: **planned / scope registered**; no runtime substitution or CI/benchmark success is asserted by this file.
- Reconcile existing Conductor tracks before new work; preserve domain-specific APIs, schema versions, privacy and public CLI behaviour.
- Write failing contract tests before code changes, then run native targeted/full tests, fmt/clippy and relevant integration/supply-chain checks at exact SHA.
- Use Cargo feature/duplicate inventories plus security, licence and MSRV assessments, and fixed-fixture performance baselines before adoption.
- Record adopt / retain / reject / defer decisions and rollback; only close after verified evidence.
