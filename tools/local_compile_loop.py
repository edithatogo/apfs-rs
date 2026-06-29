#!/usr/bin/env python3
"""Run or plan the first local Rust compile loop.

This tool is intentionally safe: it executes only repository-local Rust quality
commands when explicitly requested and writes logs under a host-side output
directory. It never opens APFS media and never writes to APFS images.
"""
from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

COMMANDS = [
    ["cargo", "fmt", "--all", "--", "--check"],
    ["cargo", "test", "--workspace"],
    ["cargo", "clippy", "--workspace", "--all-targets", "--all-features", "--", "-D", "warnings"],
    ["cargo", "xtask", "registry-check"],
    ["cargo", "xtask", "conductor-check"],
    ["cargo", "xtask", "safety-check"],
]


def command_slug(command: list[str]) -> str:
    return "-".join(part.replace("--", "").replace("/", "-") for part in command[:3])


def run_command(command: list[str], out_dir: Path) -> dict:
    log_path = out_dir / f"{command_slug(command)}.log"
    proc = subprocess.run(command, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    log_path.write_text(proc.stdout, encoding="utf-8")
    return {
        "command": command,
        "status": "passed" if proc.returncode == 0 else "failed",
        "returncode": proc.returncode,
        "log": str(log_path),
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Run or plan APFS-RS local compile loop")
    parser.add_argument("--out-dir", default="target/local-compile-loop")
    parser.add_argument("--execute", action="store_true", help="execute cargo commands if cargo exists")
    args = parser.parse_args()

    out_dir = Path(args.out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    cargo = shutil.which("cargo")
    rustc = shutil.which("rustc")
    results = []

    if args.execute and cargo:
        for command in COMMANDS:
            results.append(run_command(command, out_dir))
            if results[-1]["status"] == "failed":
                break
    else:
        for command in COMMANDS:
            results.append({
                "command": command,
                "status": "planned",
                "reason": "cargo missing" if not cargo else "dry run; pass --execute to run",
            })

    report = {
        "schema_version": "0.22.0",
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "tool": "tools/local_compile_loop.py",
        "cargo_detected": bool(cargo),
        "rustc_detected": bool(rustc),
        "executed": bool(args.execute and cargo),
        "results": results,
        "safety": {
            "opens_apfs_media": False,
            "writes_apfs_media": False,
            "host_side_logs_only": True,
        },
    }
    (out_dir / "local-compile-loop.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    md = ["# APFS-RS Local Compile Loop", "", f"Cargo detected: `{bool(cargo)}`", f"Executed: `{report['executed']}`", "", "## Commands"]
    for item in results:
        md.append(f"- `{' '.join(item['command'])}` — {item['status']}")
    md.append("\n## Safety\n\nThis loop runs host-side Cargo checks only and never mutates APFS media.\n")
    (out_dir / "local-compile-loop.md").write_text("\n".join(md), encoding="utf-8")
    print(f"local-compile-loop: wrote {out_dir}")
    return 0 if all(r["status"] in {"passed", "planned"} for r in results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
