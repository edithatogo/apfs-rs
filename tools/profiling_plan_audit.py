#!/usr/bin/env python3
"""Audit profiling and benchmark planning files."""
from __future__ import annotations
import json, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]

def fail(msg: str) -> None:
    print(f"profiling-plan-audit: ERROR: {msg}", file=sys.stderr)
    raise SystemExit(1)

def main() -> int:
    plan_path = ROOT / "profiling/profile_plan.json"
    if not plan_path.exists():
        fail("missing profiling/profile_plan.json")
    plan = json.loads(plan_path.read_text(encoding="utf-8"))
    profiles = plan.get("profiles", [])
    if len(profiles) < 3:
        fail("expected at least three profiling targets")
    for item in profiles:
        if "cargo run" not in item.get("command", ""):
            fail(f"profile item lacks cargo run command: {item}")
    (ROOT / "PROFILING_PLAN_AUDIT.json").write_text(json.dumps({"schema_version": "0.27.0", "status": "passed", "profiles": profiles}, indent=2) + "\n", encoding="utf-8")
    (ROOT / "PROFILING_PLAN_AUDIT.md").write_text("# Profiling Plan Audit\n\nStatus: passed. Profiling plan has local cargo-run release targets.\n", encoding="utf-8")
    print("profiling-plan-audit: passed")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
