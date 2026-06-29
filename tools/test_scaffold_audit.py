
#!/usr/bin/env python3
"""Audit that unit, integration, E2E, property, mutation, fuzz, and profiling scaffolds exist."""
from __future__ import annotations
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CHECKS = {
    "unit_tests_present": lambda: any("#[test]" in p.read_text(errors="replace") for p in ROOT.glob("crates/*/src/*.rs")),
    "integration_tests_present": lambda: any(ROOT.glob("crates/*/tests/*.rs")),
    "e2e_tests_present": lambda: any(ROOT.glob("crates/apfs-cli/tests/*.rs")),
    "property_tests_present": lambda: any("proptest!" in p.read_text(errors="replace") for p in ROOT.glob("crates/**/*.rs")),
    "python_hypothesis_tests_present": lambda: any("from hypothesis" in p.read_text(errors="replace") for p in ROOT.glob("tools/tests/*.py")),
    "mutation_config_present": lambda: (ROOT / ".config/cargo-mutants.toml").exists(),
    "fuzz_targets_present": lambda: any(ROOT.glob("fuzz/fuzz_targets/*.rs")),
    "profiling_bench_present": lambda: any(ROOT.glob("crates/*/benches/*.rs")),
    "coverage_90_gate_present": lambda: "--fail-under-lines 90" in (ROOT / ".github/workflows/coverage.yml").read_text(errors="replace"),
}

def main() -> int:
    checks = {name: func() for name, func in CHECKS.items()}
    report = {"status": "passed" if all(checks.values()) else "failed", "checks": checks,
              "note": "Presence audit only; execution requires Rust/Python dev dependencies locally."}
    (ROOT / "TEST_SCAFFOLD_AUDIT.json").write_text(json.dumps(report, indent=2) + "\n")
    lines = ["# Test Scaffold Audit", "", f"Status: `{report['status']}`", "", "## Checks"]
    for k, v in checks.items(): lines.append(f"- {k}: `{v}`")
    lines += ["", report["note"]]
    (ROOT / "TEST_SCAFFOLD_AUDIT.md").write_text("\n".join(lines) + "\n")
    print(f"test-scaffold-audit: {report['status']}")
    return 0 if report["status"] == "passed" else 1

if __name__ == "__main__":
    raise SystemExit(main())
