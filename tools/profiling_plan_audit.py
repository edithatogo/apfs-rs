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
    if plan.get("schema_version") != (ROOT / "VERSION").read_text(encoding="utf-8").strip():
        fail("profiling plan schema_version must match VERSION")
    if len(profiles) < 5:
        fail("expected at least five profiling targets")
    names = {item.get("name") for item in profiles}
    for required_name in {"inspect_synthetic_nxsb", "lookup_synthetic_resolver", "directory_listing_synthetic", "parser_microbench_nxsb", "inspect_microbench_core"}:
        if required_name not in names:
            fail(f"missing profiling target {required_name}")
    for item in profiles:
        command = item.get("command", "")
        if "cargo run" not in command and "cargo bench" not in command:
            fail(f"profile item lacks cargo run or cargo bench command: {item}")
    workflow = (ROOT / ".github/workflows/profiling.yml").read_text(encoding="utf-8")
    for snippet in ("cargo bench -p apfs-core --bench inspect_synthetic", "cargo bench -p apfs-types --bench nx_superblock_bench"):
        if snippet not in workflow:
            fail(f"profiling workflow does not invoke {snippet}")
    (ROOT / "PROFILING_PLAN_AUDIT.json").write_text(json.dumps({"schema_version": "0.29.0", "status": "passed", "profiles": profiles}, indent=2) + "\n", encoding="utf-8")
    (ROOT / "PROFILING_PLAN_AUDIT.md").write_text("# Profiling Plan Audit\n\nStatus: passed. Profiling plan has local cargo-run release targets and scheduled Criterion benches.\n", encoding="utf-8")
    print("profiling-plan-audit: passed")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
