#!/usr/bin/env python3
"""Verify SHA256SUMS and source-handoff artifact integrity without Cargo."""
from __future__ import annotations
import hashlib, json, sys
from pathlib import Path
from datetime import datetime, timezone
ROOT = Path(__file__).resolve().parents[1]
EXCLUDE_PARTS = {
    ".astro",
    ".git",
    "__pycache__",
    "apfs-rs-impl-v0.29",
    "dist",
    "node_modules",
    "target",
}
EXCLUDE_FILES = {"SHA256SUMS.txt", "PACKAGE_INTEGRITY_AUDIT.md", "PACKAGE_INTEGRITY_AUDIT.json"}
EXCLUDE_FILES = {"SHA256SUMS.txt", "PACKAGE_INTEGRITY_AUDIT.json", "PACKAGE_INTEGRITY_AUDIT.md", "HANDOFF_MANIFEST_VERIFY.json", "HANDOFF_MANIFEST_VERIFY.md"}

def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1024*1024), b''):
            h.update(chunk)
    return h.hexdigest()

def load_sums(path: Path) -> dict[str,str]:
    sums = {}
    if not path.exists():
        return sums
    for line in path.read_text(encoding='utf-8').splitlines():
        if not line.strip():
            continue
        digest, rel = line.split(None, 1)
        sums[rel.strip().lstrip('*')] = digest
    return sums

def main() -> int:
    sums = load_sums(ROOT / 'SHA256SUMS.txt')
    files = []
    for p in sorted(ROOT.rglob('*')):
        if p.is_file() and not any(part in EXCLUDE_PARTS for part in p.relative_to(ROOT).parts):
            rel = str(p.relative_to(ROOT))
            if rel in EXCLUDE_FILES:
                continue
            files.append(rel)
    missing = sorted(set(files) - set(sums))
    stale = []
    for rel in sorted(set(files) & set(sums)):
        if sha256(ROOT / rel) != sums[rel]:
            stale.append(rel)
    extra = sorted(set(sums) - set(files))
    data = {"schema_version":"0.27.0", "generated_utc": datetime.now(timezone.utc).isoformat(), "file_count": len(files), "missing": missing, "stale": stale, "extra": extra}
    (ROOT / "PACKAGE_INTEGRITY_AUDIT.json").write_text(json.dumps(data, indent=2)+"\n", encoding='utf-8')
    lines = ["# Package Integrity Audit", "", f"Files tracked: {len(files)}", "", f"Missing SHA entries: {len(missing)}", f"Stale SHA entries: {len(stale)}", f"Extra SHA entries: {len(extra)}", ""]
    if missing or stale or extra:
        for name, values in (("Missing", missing), ("Stale", stale), ("Extra", extra)):
            if values:
                lines.append(f"## {name}")
                lines.extend(f"- `{v}`" for v in values[:200])
                if len(values) > 200:
                    lines.append(f"- ... {len(values)-200} more")
                lines.append("")
    else:
        lines.append("SHA256SUMS is consistent with current source files.")
    (ROOT / "PACKAGE_INTEGRITY_AUDIT.md").write_text("\n".join(lines)+"\n", encoding='utf-8')
    if missing or stale or extra:
        print(f"package-integrity-audit: ERROR missing={len(missing)} stale={len(stale)} extra={len(extra)}", file=sys.stderr)
        return 1
    print(f"package-integrity-audit: passed ({len(files)} files)")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
