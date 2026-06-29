#!/usr/bin/env python3
"""Generate a production gap report from REMAINING_ELEMENTS.yaml."""
from __future__ import annotations

import json
from pathlib import Path
import yaml

ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    ledger = yaml.safe_load((ROOT / "REMAINING_ELEMENTS.yaml").read_text(encoding="utf-8"))
    summary = ledger.get("summary", {})
    mvp = ledger.get("remaining_windows_readonly_mvp", [])
    beyond = ledger.get("remaining_beyond_mvp", [])
    report = {
        "schema_version": "0.22.0",
        "summary": summary,
        "windows_readonly_mvp_remaining": mvp,
        "beyond_mvp_remaining": beyond,
        "local_required": [item for item in mvp if "machine" in str(item.get("dependency", "")).lower() or "macos" in str(item.get("dependency", "")).lower() or "fixture" in str(item.get("dependency", "")).lower()],
    }
    (ROOT / "PRODUCTION_GAP_REPORT.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    md = ["# APFS-RS Production Gap Report", "", f"Implemented/scaffolded: `{summary.get('implemented_or_scaffolded')}`", f"Windows read-only MVP blockers: `{summary.get('remaining_windows_readonly_mvp')}`", f"Broader production items: `{summary.get('remaining_overall')}`", "", "## Windows read-only MVP blockers"]
    md.extend([f"- **{item['id']}**: {item['title']} — dependency: {item.get('dependency', 'unspecified')}" for item in mvp])
    md.append("\n## Beyond-MVP production items")
    md.extend([f"- **{item['id']}**: {item['title']}" for item in beyond])
    (ROOT / "PRODUCTION_GAP_REPORT.md").write_text("\n".join(md) + "\n", encoding="utf-8")
    print("production-gap-report: wrote PRODUCTION_GAP_REPORT.md/json")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
