#!/usr/bin/env python3
"""Generate a compact context brief for humans and future coding agents."""
from __future__ import annotations
import json
from pathlib import Path
from datetime import datetime, timezone
try:
    import yaml  # type: ignore
except Exception as e:
    raise SystemExit(f"PyYAML required: {e}")
ROOT = Path(__file__).resolve().parents[1]

def maybe(path: str, max_chars: int = 600) -> str:
    p = ROOT / path
    if not p.exists():
        return ""
    txt = p.read_text(encoding='utf-8').strip()
    return txt[:max_chars] + ("..." if len(txt) > max_chars else "")

def main() -> int:
    version = (ROOT/'VERSION').read_text(encoding='utf-8').strip()
    rem = yaml.safe_load((ROOT/'REMAINING_ELEMENTS.yaml').read_text(encoding='utf-8'))
    caps = yaml.safe_load((ROOT/'codev/resources/capabilities.yaml').read_text(encoding='utf-8')).get('capabilities', {})
    tracks = sorted(p.name for p in (ROOT/'conductor/tracks').iterdir() if p.is_dir())
    data = {
        "schema_version": "0.27.0",
        "generated_utc": datetime.now(timezone.utc).isoformat(),
        "version": version,
        "capabilities": len(caps),
        "conductor_tracks": len(tracks),
        "remaining_summary": rem.get('summary', {}),
        "read_first": ["AGENTS.md", "LOCAL_HANDOFF.md", "REQUIREMENTS.md", "DESIGN.md", "REMAINING_ELEMENTS.md", "conductor/tracks.md", "codev/CHANGELOG.md"],
        "first_commands_here": ["python3 tools/cargoless_smoke_suite.py", "python3 tools/tool_capability_matrix.py", "python3 tools/rust_static_lint.py"],
        "first_commands_local": ["cargo fmt --all -- --check", "cargo test --workspace", "cargo clippy --workspace --all-targets --all-features -- -D warnings"],
    }
    (ROOT/'AGENT_HANDOFF_BRIEF.json').write_text(json.dumps(data, indent=2)+"\n", encoding='utf-8')
    lines = ["# Agent Handoff Brief", "", f"Version: {version}", f"Generated: {data['generated_utc']}", "", "## Current state", "", f"- Capabilities/scaffolds: {len(caps)}", f"- Conductor tracks: {len(tracks)}", f"- Remaining MVP blockers: {rem.get('summary',{}).get('remaining_windows_readonly_mvp')}", f"- Required current-environment-completable items remaining: {rem.get('summary',{}).get('required_current_environment_items_remaining')}", "", "## Read first"]
    lines.extend(f"- `{p}`" for p in data['read_first'])
    lines.extend(["", "## Current-environment commands"])
    lines.extend(f"```bash\n{cmd}\n```" for cmd in data['first_commands_here'])
    lines.extend(["", "## First local Rust commands"])
    lines.extend(f"```bash\n{cmd}\n```" for cmd in data['first_commands_local'])
    lines.extend(["", "## Safety summary", "", "No APFS media writes, raw physical-device access, encryption bypass, repair, format, or live mount lifecycle are implemented in this pack."])
    (ROOT/'AGENT_HANDOFF_BRIEF.md').write_text("\n".join(lines)+"\n", encoding='utf-8')
    print("agent-handoff-brief: wrote AGENT_HANDOFF_BRIEF.md/json")
    return 0

if __name__ == '__main__':
    raise SystemExit(main())
