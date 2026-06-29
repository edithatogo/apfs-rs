#!/usr/bin/env python3
"""Generate a cargoless Rust API surface map from source text."""
from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path.cwd()
PUB_RE = re.compile(r"^(?:pub\s+)?(?:async\s+)?(?:fn|struct|enum|trait|type)\s+([A-Za-z_][A-Za-z0-9_]*)")
PUB_EXPLICIT_RE = re.compile(r"^pub\s+(?:async\s+)?(fn|struct|enum|trait|type)\s+([A-Za-z_][A-Za-z0-9_]*)")


def scan_file(path: Path) -> dict:
    items = []
    for idx, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        stripped = line.strip()
        m = PUB_EXPLICIT_RE.match(stripped)
        if m:
            items.append({"kind": m.group(1), "name": m.group(2), "line": idx})
    return {"path": str(path), "public_items": items}


def main() -> int:
    files = sorted(Path("crates").glob("*/src/**/*.rs")) + sorted(Path("xtask/src").glob("**/*.rs"))
    report = {"schema_version": "0.18.0", "files": [scan_file(path) for path in files]}
    Path("target").mkdir(exist_ok=True)
    Path("target/rust-api-map.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    lines = ["# APFS-RS Rust API Map", ""]
    for file_report in report["files"]:
        lines.append(f"## `{file_report['path']}`")
        lines.append("")
        if not file_report["public_items"]:
            lines.append("- No public items detected by cargoless scanner.")
        for item in file_report["public_items"]:
            lines.append(f"- line {item['line']}: `{item['kind']} {item['name']}`")
        lines.append("")
    Path("RUST_API_MAP.md").write_text("\n".join(lines), encoding="utf-8")
    print(f"rust-api-map: scanned {len(files)} files")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
