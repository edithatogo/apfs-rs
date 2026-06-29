#!/usr/bin/env python3
"""Validate Windows read-only mount readiness scaffolding without invoking WinFsp."""
from __future__ import annotations

from pathlib import Path

REQUIRED = [
    "crates/apfs-vfs/Cargo.toml",
    "crates/apfs-vfs/src/lib.rs",
    "crates/apfs-win/Cargo.toml",
    "crates/apfs-win/src/lib.rs",
    "tools/windows/README.md",
    "tools/windows/smoke_readonly_mount.ps1",
    "tools/windows/adapter_readiness_check.ps1",
]

FORBIDDEN_TERMS = ["GENERIC_WRITE", "FILE_WRITE_DATA", "CreateFileW", "DELETE_ON_CLOSE"]


def main() -> int:
    missing = [path for path in REQUIRED if not Path(path).is_file()]
    if missing:
        raise SystemExit(f"windows-readiness-check: missing {missing}")
    for path in [Path("crates/apfs-win/src/lib.rs"), Path("tools/windows/smoke_readonly_mount.ps1")]:
        text = path.read_text(encoding="utf-8")
        for term in FORBIDDEN_TERMS:
            if term in text:
                raise SystemExit(f"windows-readiness-check: forbidden term {term} in {path}")
    print(f"windows-readiness-check: passed ({len(REQUIRED)} required files)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
