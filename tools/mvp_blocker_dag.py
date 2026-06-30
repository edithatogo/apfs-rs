#!/usr/bin/env python3
"""Generate a dependency DAG for the remaining Windows read-only MVP blockers."""
from __future__ import annotations

import json
from pathlib import Path

try:
    import yaml  # type: ignore
except Exception as exc:  # pragma: no cover
    raise SystemExit(f"PyYAML required: {exc}")

ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    ledger = yaml.safe_load((ROOT / "REMAINING_ELEMENTS.yaml").read_text(encoding="utf-8")) or {}
    blockers = ledger.get("remaining_windows_readonly_mvp", []) or []
    ordered = [
        {
            "id": item.get("id"),
            "title": item.get("title"),
            "depends_on": [],
            "environment": str(item.get("dependency") or "local-rust-macos"),
        }
        for item in blockers
    ]

    report = {
        "schema_version": "0.27.0",
        "blocker_count": len(ordered),
        "blockers": ordered,
    }
    (ROOT / "MVP_BLOCKER_DAG.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")

    md = [
        "# MVP Blocker Dependency DAG",
        "",
        f"Blockers: `{len(ordered)}`",
        "",
        "```mermaid",
        "flowchart TD",
        "```",
        "",
        "## Ordered blockers",
        "",
    ]
    if ordered:
        for item in ordered:
            md.append(f"- `{item['id']}` **{item['title']}**")
    else:
        md.append("- None")

    (ROOT / "MVP_BLOCKER_DAG.md").write_text("\n".join(md) + "\n", encoding="utf-8")
    print(f"mvp-blocker-dag: wrote {len(ordered)} blockers")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
