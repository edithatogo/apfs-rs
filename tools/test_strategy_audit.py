#!/usr/bin/env python3
"""Audit unit/integration/e2e/property/fuzz/mutation/profiling scaffolds."""
from __future__ import annotations
import json, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
REQUIRED = [
    "TEST_STRATEGY.md",
    "QUALITY_GATES.md",
    "tests/README.md",
    "tests/integration/README.md",
    "tests/e2e/README.md",
    "tests/property/README.md",
    "fuzz/Cargo.toml",
    "fuzz/fuzz_targets/object_header.rs",
    "fuzz/fuzz_targets/nx_superblock.rs",
    "mutants.toml",
    "benches/README.md",
    "profiling/README.md",
    "profiling/profile_plan.json",
]

def fail(msg: str) -> None:
    print(f"test-strategy-audit: ERROR: {msg}", file=sys.stderr)
    raise SystemExit(1)

def main() -> int:
    missing = [p for p in REQUIRED if not (ROOT / p).exists() or not (ROOT / p).read_text(encoding="utf-8", errors="ignore").strip()]
    if missing:
        fail("missing/empty files: " + ", ".join(missing))
    text = (ROOT / "QUALITY_GATES.md").read_text(encoding="utf-8")
    for term in ["Unit", "Integration", "End-to-end", "Coverage", ">= 90", "Mutation", "Property", "Profiling"]:
        if term not in text:
            fail(f"QUALITY_GATES.md missing {term}")
    result = {"schema_version": "0.27.0", "status": "passed", "required_files": REQUIRED}
    (ROOT / "TEST_STRATEGY_AUDIT.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    (ROOT / "TEST_STRATEGY_AUDIT.md").write_text(
        "# Test Strategy Audit\n\n"
        "Status: passed. Unit, integration, E2E, property, fuzz, mutation, coverage, "
        "and profiling scaffolds are present.\n",
        encoding="utf-8",
    )
    print("test-strategy-audit: passed")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
