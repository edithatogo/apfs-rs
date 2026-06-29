#!/usr/bin/env python3
"""Static CLI contract check for APFS-RS without requiring Rust/Cargo."""
from __future__ import annotations
import re, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
EXPECTED = [
    "Version", "Inspect", "CompatibilityReport", "Doctor", "DiagnosticsExport",
    "LookupObject", "Volumes", "ResolverReport", "BtreeCursorReport",
    "ReadObject", "Ls", "Cat", "Stat", "WinfspCallbackMatrix", "MountPlan",
    "DiagnosticsBundle", "PathPolicy", "FeatureReadiness", "MetadataFeatureReport",
    "Extract",
]

def fail(msg: str) -> None:
    print(f"cli-contract-check: ERROR: {msg}", file=sys.stderr)
    raise SystemExit(1)

def main() -> int:
    text = (ROOT / "crates/apfs-cli/src/main.rs").read_text(encoding="utf-8")
    found = set(re.findall(r"^\s{4}([A-Z][A-Za-z0-9]*)\s*\{", text, flags=re.M))
    missing = [name for name in EXPECTED if name not in found]
    if missing:
        fail("missing CLI command variants: " + ", ".join(missing))
    match_missing = [name for name in EXPECTED if f"Command::{name}" not in text]
    if match_missing:
        fail("missing CLI match arms: " + ", ".join(match_missing))
    required_docs = {
        "mount-plan": "RUNBOOK.md",
        "diagnostics-bundle": "RUNBOOK.md",
        "diagnostics-export": "RUNBOOK.md",
        "doctor": "RUNBOOK.md",
        "version": "README.md",
        "stat": "RUNBOOK.md",
        "extract": "RUNBOOK.md",
        "path-policy": "RUNBOOK.md",
        "feature-readiness": "FEATURE_READINESS.md",
        "metadata-feature-report": "RUNBOOK.md",
    }
    for token, doc in required_docs.items():
        path = ROOT / doc
        if token not in path.read_text(encoding="utf-8"):
            fail(f"{doc} does not mention `{token}`")
    print(f"cli-contract-check: passed ({len(EXPECTED)} commands)")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
