#!/usr/bin/env python3
"""Report local tool availability without requiring Rust to be installed."""
from __future__ import annotations

import argparse
import json
import platform
import shutil
import subprocess
import sys
from pathlib import Path

TOOLS = [
    "python3",
    "cargo",
    "rustc",
    "rustup",
    "git",
    "jq",
    "unzip",
    "zip",
    "hdiutil",
    "diskutil",
    "powershell",
    "pwsh",
    "winget",
]

ROOT = Path(__file__).resolve().parents[1]


def probe_tool(name: str) -> dict:
    path = shutil.which(name)
    result = {"name": name, "available": bool(path), "path": path, "version": None}
    if path:
        version_args = {
            "python3": [path, "--version"],
            "cargo": [path, "--version"],
            "rustc": [path, "--version"],
            "rustup": [path, "--version"],
            "git": [path, "--version"],
            "jq": [path, "--version"],
            "unzip": [path, "-v"],
            "zip": [path, "-v"],
            "hdiutil": [path, "help"],
            "diskutil": [path, "list"],
            "powershell": [path, "-Version"],
            "pwsh": [path, "-Version"],
            "winget": [path, "--version"],
        }.get(name)
        if version_args:
            try:
                proc = subprocess.run(version_args, text=True, capture_output=True, timeout=5)
                output = (proc.stdout or proc.stderr).strip().splitlines()
                result["version"] = output[0] if output else None
            except Exception as exc:  # pragma: no cover - environment dependent
                result["version_error"] = str(exc)
    return result


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--json", type=Path, help="write JSON report to path")
    args = parser.parse_args()

    report = {
        "schema_version": "0.21.0",
        "platform": platform.platform(),
        "python": sys.version.split()[0],
        "root": str(ROOT),
        "tools": [probe_tool(tool) for tool in TOOLS],
        "next_steps": [],
    }
    available = {item["name"]: item["available"] for item in report["tools"]}
    if not available.get("cargo"):
        report["next_steps"].append("Install Rust/Cargo with rustup or use the devcontainer before running cargo tests.")
    if platform.system() == "Darwin" and not available.get("hdiutil"):
        report["next_steps"].append("macOS APFS fixture generation requires hdiutil and diskutil.")
    if platform.system() == "Windows" and not (available.get("powershell") or available.get("pwsh")):
        report["next_steps"].append("Windows setup checks expect PowerShell.")

    text = json.dumps(report, indent=2)
    if args.json:
        args.json.parent.mkdir(parents=True, exist_ok=True)
        args.json.write_text(text + "\n", encoding="utf-8")
        print(f"local-env-doctor: wrote {args.json}")
    else:
        print(text)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
