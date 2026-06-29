#!/usr/bin/env python3
"""Summarise configured quality gates and what still requires local/CI execution."""
from __future__ import annotations
import json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]

def main() -> int:
    version = (ROOT / "VERSION").read_text(encoding="utf-8").strip()
    gates = [
        {"gate": "format", "configured": True, "requires_local_execution": True, "command": "cargo fmt --all -- --check"},
        {"gate": "clippy", "configured": True, "requires_local_execution": True, "command": "cargo clippy --workspace --all-targets --all-features -- -D warnings"},
        {"gate": "unit/integration/e2e", "configured": True, "requires_local_execution": True, "command": "cargo nextest run --workspace --all-features"},
        {"gate": ">=90% coverage", "configured": True, "requires_local_execution": True, "command": "cargo llvm-cov nextest --workspace --all-features --fail-under-lines 90"},
        {"gate": "mutation", "configured": True, "requires_local_execution": True, "command": "cargo mutants --workspace"},
        {"gate": "fuzz smoke", "configured": True, "requires_local_execution": True, "command": "cargo fuzz run object_header -- -max_total_time=60"},
        {"gate": "Hypothesis-style Python tests", "configured": True, "requires_local_execution": False, "command": "pytest python_tests"},
        {"gate": "profiling", "configured": True, "requires_local_execution": True, "command": "cargo bench"},
        {"gate": "Astro 7 docs", "configured": True, "requires_local_execution": True, "command": "cd docs-site && npm install && npm run build"},
    ]
    envelope = {"schema_version": version, "status": "configured_not_executed", "gates": gates}
    (ROOT / "QUALITY_GATE_EVIDENCE.json").write_text(json.dumps(envelope, indent=2) + "\n", encoding="utf-8")
    lines = ["# Quality Gate Evidence", "", "This file distinguishes configured gates from gates that have been executed. Rust, Cargo and npm build execution must happen locally or in CI.", "", "| Gate | Configured | Requires local/CI execution | Command |", "|---|---:|---:|---|"]
    for g in gates:
        lines.append(f"| {g['gate']} | `{str(g['configured']).lower()}` | `{str(g['requires_local_execution']).lower()}` | `{g['command']}` |")
    (ROOT / "QUALITY_GATE_EVIDENCE.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"quality-gate-evidence: wrote {len(gates)} gates")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
