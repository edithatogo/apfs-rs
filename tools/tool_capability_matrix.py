#!/usr/bin/env python3
"""Inventory host tools and map them to APFS-RS validation/control capabilities."""
from __future__ import annotations
import json, shutil, subprocess
from pathlib import Path
from datetime import datetime, timezone

ROOT = Path(__file__).resolve().parents[1]
TOOLS = {
    "python3": {"category":"available_here_core","uses":["all cargoless validators", "fixture generation", "JSON/YAML/TOML parsing"]},
    "jq": {"category":"available_here_optional","uses":["JSON inspection", "release evidence spot checks"]},
    "rg": {"category":"available_here_optional","uses":["source and docs grep", "unsafe/write token scanning"]},
    "grep": {"category":"available_here_core","uses":["portable text search"]},
    "sed": {"category":"available_here_core","uses":["text transformations"]},
    "awk": {"category":"available_here_core","uses":["text summaries"]},
    "find": {"category":"available_here_core","uses":["file inventory"]},
    "zip": {"category":"available_here_core","uses":["artifact packaging"]},
    "unzip": {"category":"available_here_core","uses":["archive validation"]},
    "sha256sum": {"category":"available_here_core","uses":["checksum manifests"]},
    "git": {"category":"available_here_optional","uses":["future local version-control checks"]},
    "make": {"category":"available_here_optional","uses":["developer command wrappers"]},
    "gcc": {"category":"available_here_optional","uses":["future C shim experiments, not required now"]},
    "clang": {"category":"available_here_optional","uses":["future FFI/header checks, not required now"]},
    "node": {"category":"available_here_optional","uses":["markdown tooling if installed through npm"]},
    "npm": {"category":"available_here_optional","uses":["optional markdown/mermaid tooling"]},
    "cargo": {"category":"local_required","uses":["compile", "test", "clippy", "xtask"]},
    "rustc": {"category":"local_required","uses":["Rust compiler"]},
    "rustfmt": {"category":"local_required","uses":["Rust formatting"]},
    "cargo-nextest": {"category":"local_recommended","uses":["faster test runner"]},
    "cargo-deny": {"category":"local_recommended","uses":["license/advisory checks"]},
    "cargo-audit": {"category":"local_recommended","uses":["security advisory checks"]},
    "cargo-vet": {"category":"local_recommended","uses":["dependency review attestation"]},
    "cargo-fuzz": {"category":"local_recommended","uses":["parser fuzzing"]},
    "cargo-llvm-cov": {"category":"local_recommended","uses":["coverage"]},
    "hdiutil": {"category":"macos_required","uses":["real APFS sparseimage fixture generation"]},
    "diskutil": {"category":"macos_required","uses":["APFS formatting/oracle metadata"]},
    "fsck_apfs": {"category":"macos_recommended","uses":["APFS fixture verification"]},
    "powershell": {"category":"windows_required","uses":["Windows smoke scripts"]},
    "pwsh": {"category":"windows_or_cross_platform_optional","uses":["PowerShell scripts"]},
}

def version_of(tool: str) -> str | None:
    path = shutil.which(tool)
    if not path:
        return None
    for args in ([tool, "--version"], [tool, "-V"]):
        try:
            out = subprocess.run(args, capture_output=True, text=True, timeout=3)
            text = (out.stdout or out.stderr).strip().splitlines()
            if text:
                return text[0][:200]
        except Exception:
            continue
    return "available"

def main() -> int:
    rows = []
    for tool, meta in TOOLS.items():
        path = shutil.which(tool)
        rows.append({
            "tool": tool,
            "available": bool(path),
            "path": path,
            "version": version_of(tool),
            **meta,
        })
    available_here = [r for r in rows if r["available"] and r["category"].startswith("available_here")]
    missing_local = [r for r in rows if not r["available"] and ("local" in r["category"] or "macos" in r["category"] or "windows" in r["category"])]
    data = {
        "schema_version": "0.26.0",
        "generated_utc": datetime.now(timezone.utc).isoformat(),
        "summary": {"available_here_tools": len(available_here), "missing_platform_tools": len(missing_local), "total_tracked_tools": len(rows)},
        "tools": rows,
    }
    (ROOT / "TOOL_CAPABILITY_MATRIX.json").write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
    lines = ["# Tool Capability Matrix", "", f"Generated: {data['generated_utc']}", "", "## Summary", "", f"- Available current-environment tools: {len(available_here)}", f"- Missing local/platform tools tracked: {len(missing_local)}", "", "## Tools", "", "| Tool | Available | Category | Use | Version |", "|---|---:|---|---|---|"]
    for r in rows:
        uses = "; ".join(r["uses"])
        lines.append(f"| `{r['tool']}` | {'yes' if r['available'] else 'no'} | {r['category']} | {uses} | {r.get('version') or ''} |")
    (ROOT / "TOOL_CAPABILITY_MATRIX.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"tool-capability-matrix: wrote {len(rows)} tools, {len(available_here)} available here")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
