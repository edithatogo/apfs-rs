#!/usr/bin/env python3
"""Check Codev, Conductor, requirements, and remaining-ledger alignment."""
from __future__ import annotations
import json, re, sys
from pathlib import Path
try:
    import yaml  # type: ignore
except Exception:
    yaml = None
ROOT = Path(__file__).resolve().parents[1]

def fail(msg: str) -> None:
    print(f"context-integrity-check: ERROR: {msg}", file=sys.stderr)
    raise SystemExit(1)

def load_yaml(path: Path):
    if yaml is None:
        fail("PyYAML is required")
    return yaml.safe_load(path.read_text(encoding="utf-8"))

def main() -> int:
    capabilities = load_yaml(ROOT / "codev/resources/capabilities.yaml").get("capabilities", {})
    remaining = load_yaml(ROOT / "REMAINING_ELEMENTS.yaml")
    implemented_ids = {item["id"] for item in remaining.get("implemented", [])}
    cap_ids = set(capabilities)
    missing = sorted(cap_ids - implemented_ids)
    if missing:
        fail("capabilities missing from remaining implemented ledger: " + ", ".join(missing))
    requirements = (ROOT / "REQUIREMENTS.md").read_text(encoding="utf-8")
    for cap_id in sorted(cap_ids):
        if cap_id not in requirements:
            fail(f"{cap_id} missing from REQUIREMENTS.md")
    tracks_text = (ROOT / "conductor/tracks.md").read_text(encoding="utf-8")
    history_text = (ROOT / "conductor/history.md").read_text(encoding="utf-8")
    track_dirs = sorted(p.name for p in (ROOT / "conductor/tracks").iterdir() if p.is_dir())
    for track in track_dirs:
        base = ROOT / "conductor/tracks" / track
        for name in ("metadata.json", "spec.md", "plan.md", "review.md"):
            path = base / name
            if not path.exists() or not path.read_text(encoding="utf-8").strip():
                fail(f"missing or empty {path.relative_to(ROOT)}")
        metadata = json.loads((base / "metadata.json").read_text(encoding="utf-8"))
        if metadata.get("track_id") != track:
            fail(f"track_id mismatch for {track}")
        if metadata.get("review_status") != "reviewed":
            fail(f"{track} metadata review_status is not reviewed")
        if metadata.get("archive_status") != "archived" or metadata.get("archived") is not True:
            fail(f"{track} metadata archive_status is not archived")
        if "## Archive closeout" not in (base / "review.md").read_text(encoding="utf-8"):
            fail(f"{track} review.md missing Archive closeout section")
        if track not in tracks_text:
            fail(f"{track} missing from conductor/tracks.md")
        if track != "0000-project-context" and track not in history_text:
            fail(f"{track} missing from conductor/history.md")
    print(f"context-integrity-check: passed ({len(cap_ids)} capabilities, {len(track_dirs)} tracks)")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
