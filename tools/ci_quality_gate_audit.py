#!/usr/bin/env python3
"""Cargoless audit for strict CI quality-gate configuration."""
from __future__ import annotations
import json, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]

def fail(msg: str) -> None:
    print(f"ci-quality-gate-audit: ERROR: {msg}", file=sys.stderr)
    raise SystemExit(1)

def main() -> int:
    workflow = ROOT / ".github/workflows/strict-quality.yml"
    if not workflow.exists():
        fail("missing strict-quality workflow")
    text = workflow.read_text(encoding="utf-8")
    required_terms = [
        "cargo fmt", "cargo clippy", "cargo nextest", "cargo llvm-cov", "--fail-under-lines 90",
        "cargo deny", "cargo audit", "cargo mutants", "cargo fuzz", "miri test",
    ]
    missing = [term for term in required_terms if term not in text]
    if missing:
        fail("strict-quality workflow missing: " + ", ".join(missing))
    cov = (ROOT / ".config/cargo-llvm-cov.toml").read_text(encoding="utf-8")
    if "minimum-lines = 90" not in cov and "fail-under-lines 90" not in cov:
        fail("coverage config does not enforce or document 90% threshold")
    if not (ROOT / "mutants.toml").exists():
        fail("missing mutants.toml")
    result = {"schema_version": "0.27.0", "status": "passed", "required_terms": required_terms}
    (ROOT / "CI_QUALITY_GATES.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    (ROOT / "CI_QUALITY_GATES.md").write_text(
        "# CI Quality Gates Audit\n\n"
        "Status: passed. Strict CI scaffolding includes format, lint, tests, 90% coverage, "
        "fuzz, mutation, Miri, and supply-chain gates.\n",
        encoding="utf-8",
    )
    print("ci-quality-gate-audit: passed")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
