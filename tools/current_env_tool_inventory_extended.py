#!/usr/bin/env python3
"""Extended current-environment tool inventory."""
from __future__ import annotations
import json, shutil, subprocess
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
TOOLS = [
    "python3", "jq", "rg", "grep", "sed", "awk", "find", "zip", "unzip", "sha256sum", "git", "make", "gcc", "clang", "node", "npm",
    "cargo", "rustc", "hdiutil", "diskutil", "fsck_apfs", "pwsh", "powershell", "shellcheck", "yamllint", "markdownlint", "typos", "taplo",
]

def version(cmd: str) -> str | None:
    path = shutil.which(cmd)
    if path is None:
        return None
    for args in ([cmd, "--version"], [cmd, "-version"], [cmd, "-v"]):
        try:
            out = subprocess.run(args, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=5)
            line = out.stdout.strip().splitlines()[0] if out.stdout.strip() else "available"
            return line
        except Exception:
            pass
    return "available"

def main() -> int:
    inventory = {cmd: {"available": shutil.which(cmd) is not None, "version": version(cmd)} for cmd in TOOLS}
    available = [k for k, v in inventory.items() if v["available"]]
    missing = [k for k, v in inventory.items() if not v["available"]]
    payload = {"schema_version": "0.27.0", "available": available, "missing": missing, "tools": inventory}
    (ROOT / "CURRENT_ENV_TOOL_INVENTORY_EXTENDED.json").write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    md = ["# Current Environment Tool Inventory Extended", "", "## Available", ""]
    md.extend(f"- `{t}`: {inventory[t]['version']}" for t in available)
    md.extend(["", "## Missing", ""])
    md.extend(f"- `{t}`" for t in missing)
    (ROOT / "CURRENT_ENV_TOOL_INVENTORY_EXTENDED.md").write_text("\n".join(md) + "\n", encoding="utf-8")
    print(f"current-env-tool-inventory-extended: available={len(available)} missing={len(missing)}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
