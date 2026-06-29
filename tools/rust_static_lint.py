#!/usr/bin/env python3
"""Cargoless Rust source lint for high-risk patterns and obvious source-shape issues."""
from __future__ import annotations
import json, re, sys
from pathlib import Path
from datetime import datetime, timezone
ROOT = Path(__file__).resolve().parents[1]
RISK_TOKENS = {
        "GENERIC_WRITE": "Windows raw write token before write-beta gate",
    "FILE_WRITE_DATA": "Windows file write data token before write-beta gate",
    "CreateFileW": "raw Windows open API requires review if added",
    "write_at_device": "raw-device write-like function token",
    "raw_write": "raw write-like function token",
    "force_write": "write-bypass-like token",
}
ALLOWLIST_PATH_PARTS = {"RUST_STATIC_LINT", "safety-gates.yaml", "safety-refusal-matrix", "unsafe-code-policy"}

def is_allowed(path: Path, token: str) -> bool:
    text = str(path)
    if token == "unsafe" and ("AGENTS.md" in text or "SAFETY" in text or "safety" in text or "known" in text.lower()):
        return True
    return any(part in text for part in ALLOWLIST_PATH_PARTS)

def strip_comments_and_strings(src: str) -> str:
    # Deliberately simple heuristic; this is a precompile guard, not a parser.
    src = re.sub(r'//.*', '', src)
    src = re.sub(r'"(?:\\.|[^"\\])*"', '""', src)
    return src

def main() -> int:
    findings = []
    rust_files = sorted(ROOT.glob("crates/**/*.rs")) + sorted(ROOT.glob("xtask/**/*.rs"))
    for path in rust_files:
        text = path.read_text(encoding="utf-8")
        stripped = strip_comments_and_strings(text)
        if stripped.count("{") != stripped.count("}"):
            findings.append({"path": str(path.relative_to(ROOT)), "severity":"error", "message":"brace count mismatch"})
        fns = re.findall(r'(?m)^\s*(?:pub\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(', stripped)
        seen = set()
        for fn in fns:
            if fn in seen and not fn.startswith("test"):
                findings.append({"path": str(path.relative_to(ROOT)), "severity":"warning", "message": f"duplicate function name in file: {fn}"})
            seen.add(fn)
        # Detect actual unsafe blocks/functions, but allow #![forbid(unsafe_code)] and text in helper names.
        if re.search(r'(?m)^\s*unsafe\s+(?:fn|impl|extern|\{)', stripped) or re.search(r'\bunsafe\s*\{', stripped):
            findings.append({"path": str(path.relative_to(ROOT)), "severity": "error", "token": "unsafe_block", "message": "actual unsafe block/function requires explicit review"})
        for token, msg in RISK_TOKENS.items():
            if token in stripped and not is_allowed(path, token):
                severity = "error" if token in {"GENERIC_WRITE", "FILE_WRITE_DATA", "write_at_device", "raw_write", "force_write"} else "warning"
                findings.append({"path": str(path.relative_to(ROOT)), "severity": severity, "token": token, "message": msg})
    data = {"schema_version":"0.27.0", "generated_utc": datetime.now(timezone.utc).isoformat(), "rust_files": len(rust_files), "findings": findings}
    (ROOT / "RUST_STATIC_LINT.json").write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")
    lines = ["# Rust Static Lint", "", f"Rust files scanned: {len(rust_files)}", "", "| Severity | Path | Message |", "|---|---|---|"]
    if findings:
        for f in findings:
            lines.append(f"| {f['severity']} | `{f['path']}` | {f['message']} |")
    else:
        lines.append("| ok | — | No high-risk cargoless findings. |")
    (ROOT / "RUST_STATIC_LINT.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    errors = [f for f in findings if f["severity"] == "error"]
    if errors:
        print(f"rust-static-lint: ERROR {len(errors)} error findings", file=sys.stderr)
        return 1
    print(f"rust-static-lint: ok ({len(rust_files)} files, {len(findings)} findings)")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
