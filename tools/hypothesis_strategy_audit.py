#!/usr/bin/env python3
"""Audit Python Hypothesis-style property-test scaffolding."""
from __future__ import annotations
import json, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]

def fail(msg: str) -> None:
    print(f"hypothesis-strategy-audit: ERROR: {msg}", file=sys.stderr)
    raise SystemExit(1)

def main() -> int:
    version = (ROOT / "VERSION").read_text(encoding="utf-8").strip()
    reqs = (ROOT / "python_tests/requirements-dev.txt").read_text(encoding="utf-8")
    if "hypothesis" not in reqs.lower():
        fail("python_tests/requirements-dev.txt must include hypothesis")
    test_files = sorted((ROOT / "python_tests").glob("test*.py"))
    checks = []
    for path in test_files:
        text = path.read_text(encoding="utf-8")
        uses_hypothesis = "hypothesis" in text or "@given" in text
        checks.append({"file": str(path.relative_to(ROOT)), "uses_hypothesis": uses_hypothesis})
    if not any(c["uses_hypothesis"] for c in checks):
        fail("no Hypothesis-style tests detected")
    envelope = {"schema_version": version, "status": "passed", "checks": checks}
    (ROOT / "HYPOTHESIS_STRATEGY_AUDIT.json").write_text(json.dumps(envelope, indent=2) + "\n", encoding="utf-8")
    lines = ["# Hypothesis Strategy Audit", "", "Status: `passed`.", "", "| File | Uses Hypothesis |", "|---|---:|"]
    for c in checks:
        lines.append(f"| `{c['file']}` | `{str(c['uses_hypothesis']).lower()}` |")
    (ROOT / "HYPOTHESIS_STRATEGY_AUDIT.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"hypothesis-strategy-audit: passed ({len(checks)} python test files)")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
