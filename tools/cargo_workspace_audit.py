#!/usr/bin/env python3
"""Cargoless Cargo workspace audit.

This is not a replacement for Cargo. It catches common handoff mistakes before a
Rust toolchain is available.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path
try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover
    import tomli as tomllib

ROOT = Path(__file__).resolve().parents[1]


def load_toml(path: Path) -> dict:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def main() -> int:
    cargo = load_toml(ROOT / "Cargo.toml")
    members = cargo.get("workspace", {}).get("members", [])
    issues: list[str] = []
    crates = []
    for member in members:
        path = ROOT / member
        manifest = path / "Cargo.toml"
        if not manifest.exists():
            issues.append(f"workspace member {member} has no Cargo.toml")
            continue
        data = load_toml(manifest)
        name = data.get("package", {}).get("name")
        if not name:
            issues.append(f"workspace member {member} has no package.name")
        if not ((path / "src/lib.rs").exists() or (path / "src/main.rs").exists()):
            issues.append(f"workspace member {member} has no src/lib.rs or src/main.rs")
        crates.append({"member": member, "name": name})
    names = [c["name"] for c in crates if c.get("name")]
    duplicates = sorted({name for name in names if names.count(name) > 1})
    for name in duplicates:
        issues.append(f"duplicate crate package name {name}")
    report = {
        "schema_version": "0.22.0",
        "workspace_members": len(members),
        "crates": crates,
        "issues": issues,
        "status": "passed" if not issues else "failed",
    }
    (ROOT / "CARGO_WORKSPACE_AUDIT.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    md = ["# Cargo Workspace Audit", "", f"Status: `{report['status']}`", f"Workspace members: `{len(members)}`", "", "## Issues"]
    md.extend([f"- {issue}" for issue in issues] or ["- None"])
    (ROOT / "CARGO_WORKSPACE_AUDIT.md").write_text("\n".join(md) + "\n", encoding="utf-8")
    print(f"cargo-workspace-audit: {report['status']} ({len(members)} members)")
    return 0 if not issues else 1


if __name__ == "__main__":
    raise SystemExit(main())
