#!/usr/bin/env python3
"""Generate the next safe APFS-RS implementation loop plan."""
from __future__ import annotations

import json
from pathlib import Path

import yaml

ORDER = [
    "MVP-R001",
    "MVP-R002",
    "MVP-R003",
    "MVP-R004",
    "MVP-R005",
    "MVP-R006",
    "MVP-R007",
    "MVP-R008",
    "MVP-R009",
]


def main() -> int:
    data = yaml.safe_load(Path("REMAINING_ELEMENTS.yaml").read_text(encoding="utf-8"))
    remaining = data.get("remaining_windows_readonly_mvp", [])
    post = data.get("remaining_beyond_mvp", [])
    external = {"MVP-R001", "MVP-R002", "MVP-R003", "MVP-R004"}
    local_safe = [item for item in remaining if item.get("id") not in external]
    support_safe = [item for item in post if item.get("id") in {"POST-R001", "POST-R002", "POST-R003", "POST-R008", "POST-R009", "POST-R010"}]
    batch = local_safe[:2] + support_safe[:3]
    report = {
        "schema_version": "0.18.0",
        "strategy": "Prefer local, read-only, non-media-mutating slices until Rust/macOS validation is available.",
        "blocked_external_actions": [item for item in remaining if item.get("id") in external],
        "recommended_next_batch": batch,
        "safety_note": "Do not implement raw writes, encryption bypass, repair, format, or physical mounts in the local-only loop.",
    }
    Path("target").mkdir(exist_ok=True)
    Path("target/next-loop-plan.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    lines = ["# APFS-RS Next Loop Plan", "", report["strategy"], "", "## Recommended next batch", ""]
    for item in batch:
        lines.append(f"- `{item.get('id')}` — {item.get('title')}")
    lines.extend(["", "## Blocked external actions", ""])
    for item in report["blocked_external_actions"]:
        lines.append(f"- `{item.get('id')}` — {item.get('title')} ({item.get('dependency', 'external')})")
    lines.append("")
    Path("NEXT_LOOP_PLAN.md").write_text("\n".join(lines), encoding="utf-8")
    print(f"next-loop-plan: recommended {len(batch)} items")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
