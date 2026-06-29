#!/usr/bin/env python3
"""Generate handoff status counts and local-first-run readiness summary."""
from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path

try:
    import yaml  # type: ignore
except Exception:  # pragma: no cover
    yaml = None

ROOT = Path(__file__).resolve().parents[1]


def load_yaml(path: Path):
    if yaml is None:
        raise SystemExit("handoff-status: PyYAML is required")
    return yaml.safe_load(path.read_text(encoding="utf-8"))


def count_conductor_tracks() -> int:
    tracks = ROOT / "conductor/tracks"
    return len([p for p in tracks.iterdir() if p.is_dir()]) if tracks.exists() else 0


def run_optional(cmd: list[str]) -> dict:
    try:
        proc = subprocess.run(cmd, cwd=ROOT, text=True, capture_output=True, timeout=30)
        return {"command": cmd, "returncode": proc.returncode, "stdout_tail": proc.stdout.strip().splitlines()[-5:], "stderr_tail": proc.stderr.strip().splitlines()[-5:]}
    except Exception as exc:  # pragma: no cover
        return {"command": cmd, "error": str(exc)}


def build_status() -> dict:
    remaining = load_yaml(ROOT / "REMAINING_ELEMENTS.yaml")
    summary = remaining.get("summary", {})
    required_files = [
        "rust-toolchain.toml",
        ".cargo/config.toml",
        "deny.toml",
        ".config/nextest.toml",
        ".devcontainer/devcontainer.json",
        "LOCAL_FIRST_RUN.md",
        "KNOWN_UNCOMPILED_RISKS.md",
        "READY_FOR_LOCAL.md",
        "HANDOFF_STATUS.md",
        "REPO_MANIFEST.md",
    ]
    file_status = {path: (ROOT / path).exists() for path in required_files}
    blockers = remaining.get("remaining_windows_readonly_mvp", [])
    beyond = remaining.get("remaining_beyond_mvp", [])
    return {
        "schema_version": "0.21.0",
        "implemented_or_scaffolded": summary.get("implemented_or_scaffolded"),
        "remaining_windows_readonly_mvp": len(blockers),
        "remaining_beyond_mvp": len(beyond),
        "remaining_total": len(blockers) + len(beyond),
        "conductor_track_count": count_conductor_tracks(),
        "required_files": file_status,
        "ready_for_local": all(file_status.values()),
        "next_local_commands": [
            "python3 tools/precompile_static_check.py",
            "python3 tools/config_sanity_check.py",
            "python3 tools/local_env_doctor.py --json target/local-env-doctor.json",
            "cargo fmt --all -- --check",
            "cargo test --workspace",
        ],
    }


def write_markdown(status: dict) -> None:
    lines = [
        "# Handoff Status",
        "",
        "Version: 0.21.0",
        "",
        f"Implemented or scaffolded elements: **{status['implemented_or_scaffolded']}**",
        f"Remaining Windows read-only MVP blockers: **{status['remaining_windows_readonly_mvp']}**",
        f"Remaining broader/post-MVP production items: **{status['remaining_beyond_mvp']}**",
        f"Total remaining major items: **{status['remaining_total']}**",
        f"Conductor track directories: **{status['conductor_track_count']}**",
        "",
        "## Required handoff files",
        "",
    ]
    for path, exists in status["required_files"].items():
        mark = "✅" if exists else "❌"
        lines.append(f"- {mark} `{path}`")
    lines.extend(["", "## First local commands", ""])
    for command in status["next_local_commands"]:
        lines.append(f"```bash\n{command}\n```")
    lines.extend(["", "## Interpretation", "", "This is a source handoff candidate, not a compile-verified release. Rust/Cargo, macOS APFS fixture generation, and Windows/WinFsp testing remain local execution blockers."])
    (ROOT / "HANDOFF_STATUS.md").write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    status = build_status()
    if args.write:
        (ROOT / "HANDOFF_STATUS.json").write_text(json.dumps(status, indent=2) + "\n", encoding="utf-8")
        write_markdown(status)
        print("handoff-status: wrote HANDOFF_STATUS.json and HANDOFF_STATUS.md")
    else:
        print(json.dumps(status, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
