#!/usr/bin/env python3
"""Check that the known-uncompiled-risk ledger has required sections."""
from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REQUIRED_HEADINGS = [
    "# Known Uncompiled Risks",
    "## Rust compilation risks",
    "## APFS semantic risks",
    "## Platform risks",
    "## Handoff triage order",
]


def main() -> int:
    path = ROOT / "KNOWN_UNCOMPILED_RISKS.md"
    if not path.exists():
        print("known-risk-check: ERROR: KNOWN_UNCOMPILED_RISKS.md missing", file=sys.stderr)
        return 1
    text = path.read_text(encoding="utf-8")
    missing = [heading for heading in REQUIRED_HEADINGS if heading not in text]
    if missing:
        print("known-risk-check: ERROR: missing headings: " + ", ".join(missing), file=sys.stderr)
        return 1
    print("known-risk-check: passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
