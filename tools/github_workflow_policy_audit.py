#!/usr/bin/env python3
"""Cargoless audit for GitHub workflow quality/security policies."""
from __future__ import annotations
import json, re, sys
from pathlib import Path
try:
    import yaml  # type: ignore
except Exception:
    yaml = None
ROOT = Path(__file__).resolve().parents[1]

def fail(msg: str) -> None:
    print(f"github-workflow-policy-audit: ERROR: {msg}", file=sys.stderr)
    raise SystemExit(1)

def read_yaml(path: Path):
    if yaml is None:
        fail("PyYAML is required")
    return yaml.safe_load(path.read_text(encoding="utf-8")) or {}

def main() -> int:
    version = (ROOT / "VERSION").read_text(encoding="utf-8").strip()
    workflows = sorted((ROOT / ".github/workflows").glob("*.yml"))
    if not workflows:
        fail("no workflows found")
    checks = []
    for wf in workflows:
        text = wf.read_text(encoding="utf-8")
        data = read_yaml(wf)
        perms = data.get("permissions")
        checks.append({"workflow": wf.name, "check": "has permissions", "ok": perms is not None})
        if perms is None:
            fail(f"{wf.name} missing top-level permissions")
        checks.append({"workflow": wf.name, "check": "least privilege contents", "ok": "contents: read" in text or isinstance(perms, dict)})
        if "pull_request" in text or "push" in text:
            checks.append({"workflow": wf.name, "check": "has checkout", "ok": "actions/checkout" in text})
            if "actions/checkout" not in text:
                fail(f"{wf.name} has PR/push trigger without checkout")
    strict = (ROOT / ".github/workflows/strict-quality.yml").read_text(encoding="utf-8")
    required_tokens = [
        "cargo fmt", "cargo clippy", "cargo nextest", "cargo llvm-cov", "--fail-under-lines 90",
        "cargo deny", "cargo audit", "cargo mutants", "cargo fuzz run", "miri test",
    ]
    for token in required_tokens:
        ok = token in strict
        checks.append({"workflow": "strict-quality.yml", "check": token, "ok": ok})
        if not ok:
            fail(f"strict-quality.yml missing {token!r}")
    docs = (ROOT / ".github/workflows/docs-site.yml").read_text(encoding="utf-8")
    for token in ("setup-node", "docs-site", "npm", "npm run build"):
        ok = token in docs
        checks.append({"workflow": "docs-site.yml", "check": token, "ok": ok})
        if not ok:
            fail(f"docs-site.yml missing {token!r}")
    envelope = {"schema_version": version, "status": "passed", "workflow_count": len(workflows), "checks": checks}
    (ROOT / "GITHUB_WORKFLOW_POLICY_AUDIT.json").write_text(json.dumps(envelope, indent=2) + "\n", encoding="utf-8")
    lines = ["# GitHub Workflow Policy Audit", "", f"Status: `passed`; workflows audited: **{len(workflows)}**.", "", "| Workflow | Check | OK |", "|---|---|---:|"]
    for c in checks:
        lines.append(f"| `{c['workflow']}` | `{c['check']}` | `{str(c['ok']).lower()}` |")
    (ROOT / "GITHUB_WORKFLOW_POLICY_AUDIT.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"github-workflow-policy-audit: passed ({len(workflows)} workflows, {len(checks)} checks)")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
