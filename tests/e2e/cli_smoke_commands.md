# CLI Smoke Commands

Run after `cargo test --workspace` passes:

```bash
cargo run -p apfs-cli -- inspect --json fixtures/synthetic-nxsb-block0.bin
cargo run -p apfs-cli -- volumes --json fixtures/synthetic-volume-superblock.img
cargo run -p apfs-cli -- ls --json fixtures/synthetic-directory-listing.img
cargo run -p apfs-cli -- cat --json fixtures/synthetic-file-preview.img --name hello.txt
cargo run -p apfs-cli -- doctor --json fixtures/synthetic-file-preview.img
```
