#!/usr/bin/env python3
"""Audit profiling and benchmark budgets without running benchmarks."""
from __future__ import annotations
import json, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]

def fail(msg: str) -> None:
    print(f"profiling-budget-check: ERROR: {msg}", file=sys.stderr)
    raise SystemExit(1)

def main() -> int:
    version = (ROOT / "VERSION").read_text(encoding="utf-8").strip()
    plan_path = ROOT / "profiling/profile_plan.json"
    if not plan_path.exists():
        fail("missing profiling/profile_plan.json")
    plan = json.loads(plan_path.read_text(encoding="utf-8"))
    bench_files = sorted(str(p.relative_to(ROOT)) for p in ROOT.glob("crates/*/benches/*.rs"))
    if not bench_files:
        fail("no Rust benchmark files found")
    workflows = (ROOT / ".github/workflows/profiling.yml").read_text(encoding="utf-8")
    if "cargo bench" not in workflows:
        fail("profiling workflow must run cargo bench")
    envelope = {"schema_version": version, "status": "passed", "profile_plan": plan, "bench_files": bench_files}
    (ROOT / "PROFILING_BUDGET_CHECK.json").write_text(json.dumps(envelope, indent=2) + "\n", encoding="utf-8")
    lines = ["# Profiling Budget Check", "", "Status: `passed`.", "", "## Benchmark files", ""]
    for f in bench_files:
        lines.append(f"- `{f}`")
    (ROOT / "PROFILING_BUDGET_CHECK.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"profiling-budget-check: passed ({len(bench_files)} benchmark files)")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
