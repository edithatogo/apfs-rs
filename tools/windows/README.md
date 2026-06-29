# Windows Read-Only Mount Readiness

This directory contains Windows-side scaffolding for the future WinFsp read-only adapter.

Current status: **readiness scaffold only**. There is no production WinFsp FFI adapter yet.

Safety rules:

- Do not open raw devices for writing.
- Do not enable write-like callbacks.
- Do not mount unsupported, encrypted, damaged, Fusion, or unknown-feature APFS states.
- The MVP adapter must translate Windows callbacks into `apfs-vfs` read-only operations.

Planned checks:

```powershell
pwsh ./tools/windows/adapter_readiness_check.ps1
pwsh ./tools/windows/smoke_readonly_mount.ps1 -Image .\fixtures\synthetic-btree-cursor.img -MountLetter X
```

The smoke script is intentionally a placeholder until the WinFsp FFI adapter exists.
