#!/usr/bin/env python3
"""Audit that release automation is configured and actually invoked."""
from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def fail(message: str) -> None:
    print(f"release-automation-audit: ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> int:
    version = (ROOT / "VERSION").read_text(encoding="utf-8").strip()
    required = [
        "release-plz.toml",
        "dist-workspace.toml",
        "RELEASE_AUTOMATION.md",
        ".github/workflows/release-automation.yml",
    ]
    checks: list[dict[str, object]] = []
    for rel in required:
        path = ROOT / rel
        checks.append({"name": rel, "ok": path.exists() and path.read_text(encoding="utf-8").strip() != ""})
    workflow = (ROOT / ".github/workflows/release-automation.yml").read_text(encoding="utf-8")
    expected_snippets = [
        "cargo install cargo-dist",
        "cargo install release-plz",
        "dist plan --allow-dirty",
        "GIT_TOKEN: ${{ secrets.GITHUB_TOKEN }}",
        "release-plz release --dry-run --allow-dirty --config release-plz.toml",
        "cargo run -p xtask -- release-automation-audit",
    ]
    for snippet in expected_snippets:
        checks.append({"name": f"workflow invokes {snippet}", "ok": snippet in workflow})
    checks.append({"name": "no placeholder echo", "ok": "Release automation placeholder" not in workflow})
    dist_config = (ROOT / "dist-workspace.toml").read_text(encoding="utf-8")
    release_config = (ROOT / "release-plz.toml").read_text(encoding="utf-8")
    checks.append({"name": "cargo-dist workspace table", "ok": "[workspace]" in dist_config})
    checks.append({"name": "cargo-dist cargo workspace member", "ok": 'members = ["cargo:."]' in dist_config})
    checks.append({"name": "cargo-dist dist table", "ok": "[dist]" in dist_config})
    checks.append({"name": "cargo-dist version pinned", "ok": "cargo-dist-version = \"0.29.0\"" in dist_config})
    checks.append({"name": "release-plz publish disabled", "ok": "publish = false" in release_config})
    failed = [check["name"] for check in checks if not check["ok"]]
    if failed:
        fail("failed checks: " + ", ".join(str(item) for item in failed))
    report = {"schema_version": version, "checks": checks, "passed": True}
    (ROOT / "RELEASE_AUTOMATION_AUDIT.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    lines = ["# Release Automation Audit", "", "Status: `passed`.", "", "| Check | OK |", "|---|---:|"]
    for check in checks:
        lines.append(f"| {check['name']} | {str(check['ok']).lower()} |")
    (ROOT / "RELEASE_AUTOMATION_AUDIT.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("release-automation-audit: passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
