# Tool Capability Matrix

Generated: 2026-06-30T15:24:52.984303+00:00

## Summary

- Available current-environment tools: 16
- Missing local/platform tools tracked: 5

## Tools

| Tool | Available | Category | Use | Version |
|---|---:|---|---|---|
| `python3` | yes | available_here_core | all cargoless validators; fixture generation; JSON/YAML/TOML parsing | Python 3.14.6 |
| `jq` | yes | available_here_optional | JSON inspection; release evidence spot checks | jq-1.7.1-apple |
| `rg` | yes | available_here_optional | source and docs grep; unsafe/write token scanning | ripgrep 15.1.0 (rev af60c2de9d) |
| `grep` | yes | available_here_core | portable text search | grep (BSD grep, GNU compatible) 2.6.0-FreeBSD |
| `sed` | yes | available_here_core | text transformations | sed: illegal option -- - |
| `awk` | yes | available_here_core | text summaries | awk version 20200816 |
| `find` | yes | available_here_core | file inventory | find: illegal option -- - |
| `zip` | yes | available_here_core | artifact packaging | Copyright (c) 1990-2008 Info-ZIP - Type 'zip "-L"' for software license. |
| `unzip` | yes | available_here_core | archive validation | caution:  both -n and -o specified; ignoring -o |
| `sha256sum` | yes | available_here_core | checksum manifests | sha256sum (Darwin) 1.0 |
| `git` | yes | available_here_optional | future local version-control checks | git version 2.53.0 |
| `make` | yes | available_here_optional | developer command wrappers | GNU Make 3.81 |
| `gcc` | yes | available_here_optional | future C shim experiments, not required now | Apple clang version 21.0.0 (clang-2100.1.1.101) |
| `clang` | yes | available_here_optional | future FFI/header checks, not required now | Apple clang version 21.0.0 (clang-2100.1.1.101) |
| `node` | yes | available_here_optional | markdown tooling if installed through npm | v26.0.0 |
| `npm` | yes | available_here_optional | optional markdown/mermaid tooling | 11.12.1 |
| `cargo` | yes | local_required | compile; test; clippy; xtask | cargo 1.95.0 (f2d3ce0bd 2026-03-21) (Homebrew) |
| `rustc` | yes | local_required | Rust compiler | rustc 1.95.0 (59807616e 2026-04-14) (Homebrew) |
| `rustfmt` | yes | local_required | Rust formatting | rustfmt 1.9.0 |
| `cargo-nextest` | yes | local_recommended | faster test runner | cargo-nextest 0.9.136 |
| `cargo-deny` | yes | local_recommended | license/advisory checks | cargo-deny 0.19.7 |
| `cargo-audit` | no | local_recommended | security advisory checks |  |
| `cargo-vet` | no | local_recommended | dependency review attestation |  |
| `cargo-fuzz` | no | local_recommended | parser fuzzing |  |
| `cargo-llvm-cov` | no | local_recommended | coverage |  |
| `hdiutil` | yes | macos_required | real APFS sparseimage fixture generation | hdiutil: --version: verb not recognized |
| `diskutil` | yes | macos_required | APFS formatting/oracle metadata | diskutil: did not recognize verb "--version"; type "diskutil" for a list |
| `fsck_apfs` | yes | macos_recommended | APFS fixture verification | fsck_apfs: unrecognized option `--version' |
| `powershell` | no | windows_required | Windows smoke scripts |  |
| `pwsh` | yes | windows_or_cross_platform_optional | PowerShell scripts | PowerShell 7.6.2 |
