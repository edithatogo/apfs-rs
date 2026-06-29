#!/usr/bin/env python3
"""Validate the macOS APFS fixture generator before running it on macOS."""
from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "tools/macos/create_real_apfs_fixture.sh"
FORBIDDEN = [
    r"diskutil\s+eraseDisk",
    r"diskutil\s+partitionDisk",
    r"/dev/disk\d+",
    r"sudo\s+",
]
REQUIRED = ["hdiutil create", "-type SPARSE", "-fs APFS", "hdiutil attach", "hdiutil detach", "redaction"]


def main() -> int:
    text = SCRIPT.read_text(encoding="utf-8")
    issues = []
    for pattern in FORBIDDEN:
        if re.search(pattern, text):
            issues.append(f"forbidden risky pattern present: {pattern}")
    for token in REQUIRED:
        if token not in text:
            issues.append(f"expected safe fixture token missing: {token}")
    report = {
        "schema_version": "0.22.0",
        "script": str(SCRIPT.relative_to(ROOT)),
        "status": "passed" if not issues else "failed",
        "issues": issues,
        "safety": {
            "expected_image_only": True,
            "physical_disk_patterns_forbidden": True,
            "requires_macos": True,
        },
    }
    (ROOT / "MACOS_FIXTURE_DRY_RUN.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    md = ["# macOS Fixture Dry Run", "", f"Status: `{report['status']}`", "", "## Issues"]
    md.extend([f"- {issue}" for issue in issues] or ["- None"])
    md.append("\nThe generator is intended to create sparse APFS images only; it must not target physical disks.\n")
    (ROOT / "MACOS_FIXTURE_DRY_RUN.md").write_text("\n".join(md), encoding="utf-8")
    print(f"macos-fixture-dry-run: {report['status']}")
    return 0 if not issues else 1


if __name__ == "__main__":
    raise SystemExit(main())
