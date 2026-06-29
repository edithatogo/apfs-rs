#!/usr/bin/env python3
"""Generate an actionable tasklist for the remaining MVP/local blockers."""
from __future__ import annotations
import json
from pathlib import Path
from datetime import datetime, timezone
try:
    import yaml  # type: ignore
except Exception as e:
    raise SystemExit(f"PyYAML required: {e}")
ROOT = Path(__file__).resolve().parents[1]
ACTIONS = {
    "MVP-R001": ["Run cargo fmt/test/clippy", "Use cargo_error_to_tracks.py on failures", "Fix one compiler class at a time"],
    "MVP-R002": ["Run tools/macos/create_real_apfs_fixture.sh on macOS", "Validate manifest with cargo xtask fixture-manifest-check"],
    "MVP-R003": ["Run real-fixture-feedback", "Run promote-feedback", "Review generated Codev/Conductor tasks"],
    "MVP-R004": ["Compare inspect JSON to manifest", "Correct offsets/semantics", "Add regression fixture"],
    "MVP-R005": ["Parse checkpoint descriptor ring", "Validate newest checkpoint selection", "Add corrupt/fallback fixtures"],
    "MVP-R006": ["Replace synthetic traversal internals with production B-tree cursor", "Validate OMAP lookup against real fixture"],
    "MVP-R007": ["Parse APFS volume superblock", "Decode root filesystem tree records", "Map metadata/stat fields"],
    "MVP-R008": ["Resolve file extents", "Extract byte-identical regular files", "Compare SHA-256 hashes"],
    "MVP-R009": ["Implement WinFsp adapter read-only callbacks", "Run Windows smoke tests", "Package baseline installer/portable zip"],
}

def main() -> int:
    remaining = yaml.safe_load((ROOT/'REMAINING_ELEMENTS.yaml').read_text(encoding='utf-8'))
    blockers = remaining.get('remaining_windows_readonly_mvp', [])
    enriched = []
    for b in blockers:
        item = dict(b)
        item['next_actions'] = ACTIONS.get(b.get('id'), ["Review remaining ledger and create local task"])
        item['current_environment_completable'] = False
        enriched.append(item)
    data = {"schema_version":"0.27.0", "generated_utc": datetime.now(timezone.utc).isoformat(), "remaining_mvp_blockers": len(enriched), "blockers": enriched}
    (ROOT/'MVP_BLOCKER_TASKLIST.json').write_text(json.dumps(data, indent=2)+"\n", encoding='utf-8')
    lines = ["# MVP Blocker Tasklist", "", f"Remaining MVP blockers: {len(enriched)}", ""]
    for b in enriched:
        lines.append(f"## {b['id']}: {b['title']}")
        lines.append("")
        lines.append(f"Dependency: {b.get('dependency','local/platform execution')}")
        lines.append("")
        for action in b['next_actions']:
            lines.append(f"- {action}")
        lines.append("")
    (ROOT/'MVP_BLOCKER_TASKLIST.md').write_text("\n".join(lines), encoding='utf-8')
    print(f"mvp-blocker-tasklist: wrote {len(enriched)} blockers")
    return 0

if __name__ == '__main__':
    raise SystemExit(main())
