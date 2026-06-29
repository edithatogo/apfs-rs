#!/usr/bin/env python3
"""Generate a host-aware command plan for current environment and local handoff."""
from __future__ import annotations
import json, shutil
from pathlib import Path
from datetime import datetime, timezone
ROOT = Path(__file__).resolve().parents[1]
CURRENT_CHECKS = [
    "python3 tools/cargoless_smoke_suite.py",
    "python3 tools/tool_capability_matrix.py",
    "python3 tools/rust_static_lint.py",
    "python3 tools/package_integrity_audit.py",
    "python3 tools/mvp_blocker_tasklist.py",
    "python3 tools/agent_handoff_brief.py",
]
LOCAL_RUST = [
    "cargo fmt --all -- --check",
    "cargo test --workspace",
    "cargo clippy --workspace --all-targets --all-features -- -D warnings",
    "cargo xtask registry-check",
    "cargo xtask conductor-check",
    "cargo xtask safety-check",
]
MACOS = [
    "./tools/macos/create_real_apfs_fixture.sh",
    "cargo xtask fixture-manifest-check fixtures/real/macos-minimal-apfs-001/manifest.json",
    "cargo run -p apfs-cli -- inspect --json fixtures/real/macos-minimal-apfs-001/macos-minimal-apfs-001.sparseimage > inspect.json",
]
WINDOWS = [
    "cargo run -p apfs-cli -- winfsp-callback-matrix --json",
    "# install WinFsp on a dedicated Windows test VM before live mount work",
]

def command_available(cmd: str) -> bool:
    first = cmd.split()[0]
    if first.startswith("./"):
        return (ROOT / first[2:]).exists()
    return shutil.which(first) is not None

def main() -> int:
    phases = {
        "current_environment": [{"command": c, "available_now": command_available(c)} for c in CURRENT_CHECKS],
        "local_rust": [{"command": c, "available_now": command_available(c)} for c in LOCAL_RUST],
        "macos_fixture": [{"command": c, "available_now": command_available(c)} for c in MACOS],
        "windows_winfsp": [{"command": c, "available_now": command_available(c)} for c in WINDOWS],
    }
    data = {"schema_version":"0.27.0", "generated_utc": datetime.now(timezone.utc).isoformat(), "phases": phases}
    (ROOT / "LOCAL_COMMAND_PLAN.json").write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
    lines = ["# Local Command Plan", "", f"Generated: {data['generated_utc']}"]
    for phase, entries in phases.items():
        lines.extend(["", f"## {phase.replace('_',' ').title()}", "", "| Available here | Command |", "|---:|---|"])
        for e in entries:
            lines.append(f"| {'yes' if e['available_now'] else 'no'} | `{e['command']}` |")
    (ROOT / "LOCAL_COMMAND_PLAN.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("local-command-plan: wrote LOCAL_COMMAND_PLAN.md/json")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
