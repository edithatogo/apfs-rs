# Platform Setup

Version: 0.20.0

## Rust host

Required first:

```bash
rustup toolchain install stable
rustup component add rustfmt clippy
cargo test --workspace
```

## macOS APFS fixture host

Needed for first real APFS fixture:

- `hdiutil`
- `diskutil`
- APFS-capable macOS
- synthetic data only

Run only the scripts under `tools/macos/` and only with disposable image files.

## Windows read-only mount host

Needed later for MVP mount testing:

- Windows 11 or supported Windows build
- Rust stable
- WinFsp runtime installed
- synthetic image fixtures first

The current Windows crate is a readiness scaffold only; no verified mount lifecycle exists yet.

## Linux/FUSE host

Needed later for cross-platform adapter work:

- Rust stable
- libfuse or platform-specific FUSE runtime
- no raw device write permissions

## Android/ChromeOS

Start with library/CLI packaging only. Do not assume block-device access, root, or FUSE availability.

## Devcontainer option

A devcontainer is available under `.devcontainer/`. It installs Rust, Python, jq, zip/unzip, and common Rust QA tools. Use it if you want a reproducible Linux-based first compile before moving to macOS/Windows-specific APFS work.
