#!/usr/bin/env python3
"""Generate a WinFsp read-only callback matrix from the APFS-RS adapter contract."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CALLBACKS = [
    ("Init", "allow", "initialise read-only filesystem instance"),
    ("GetVolumeInfo", "allow", "report read-only volume metadata"),
    ("GetSecurityByName", "allow", "conservative read-only security metadata"),
    ("Create", "refuse", "no file creation in read-only MVP"),
    ("Open", "allow_readonly", "open existing files/directories read-only"),
    ("Read", "allow", "read file data through apfs-vfs"),
    ("Write", "refuse", "all writes refused"),
    ("Flush", "allow_noop", "no APFS media mutation"),
    ("GetFileInfo", "allow", "stat/getattr mapping"),
    ("SetBasicInfo", "refuse", "metadata mutation refused"),
    ("SetFileSize", "refuse", "truncate/extend refused"),
    ("CanDelete", "refuse", "deletion refused"),
    ("Rename", "refuse", "rename refused"),
    ("ReadDirectory", "allow", "directory listing through apfs-vfs"),
    ("GetReparsePoint", "allow_if_symlink", "symlink metadata only where supported"),
    ("SetReparsePoint", "refuse", "metadata mutation refused"),
    ("DeleteReparsePoint", "refuse", "metadata mutation refused"),
]


def main() -> int:
    rows = [{"callback": c, "decision": d, "note": n} for c, d, n in CALLBACKS]
    report = {
        "schema_version": "0.22.0",
        "adapter": "WinFsp",
        "mode": "read_only_mvp_contract",
        "callbacks": rows,
        "safety": {"writes_refused": True, "physical_write_handles": False},
    }
    (ROOT / "WINFSP_CALLBACK_MATRIX.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    md = ["# WinFsp Read-Only Callback Matrix", "", "| Callback | Decision | Note |", "|---|---|---|"]
    for row in rows:
        md.append(f"| `{row['callback']}` | `{row['decision']}` | {row['note']} |")
    md.append("\nThis is a contract for the future live adapter. It is not a live mount implementation yet.\n")
    (ROOT / "WINFSP_CALLBACK_MATRIX.md").write_text("\n".join(md), encoding="utf-8")
    print("winfsp-callback-matrix: wrote WINFSP_CALLBACK_MATRIX.md/json")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
