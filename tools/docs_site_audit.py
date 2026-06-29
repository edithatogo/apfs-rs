#!/usr/bin/env python3
"""Audit Astro documentation-site scaffold without installing npm packages."""
from __future__ import annotations
import json, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]

def fail(msg: str) -> None:
    print(f"docs-site-audit: ERROR: {msg}", file=sys.stderr)
    raise SystemExit(1)

def main() -> int:
    pkg_path = ROOT / "docs-site/package.json"
    if not pkg_path.exists():
        fail("missing docs-site/package.json")
    pkg = json.loads(pkg_path.read_text(encoding="utf-8"))
    astro = pkg.get("devDependencies", {}).get("astro")
    if astro != "7.0.2":
        fail(f"docs-site must pin astro 7.0.2 for this handoff; found {astro!r}")
    required = [
        "docs-site/astro.config.mjs",
        "docs-site/tsconfig.json",
        "docs-site/src/layouts/Base.astro",
        "docs-site/src/pages/index.astro",
        "docs-site/src/pages/quality.astro",
        "docs-site/src/pages/conductor.astro",
        "docs-site/src/pages/handoff/index.astro",
        "docs-site/src/pages/test-strategy.astro",
        "docs-site/src/pages/quality-evidence.astro",
    ]
    missing = [p for p in required if not (ROOT / p).exists() or not (ROOT / p).read_text(encoding="utf-8").strip()]
    if missing:
        fail("missing docs-site files: " + ", ".join(missing))
    version = (ROOT / "VERSION").read_text(encoding="utf-8").strip()
    result = {"schema_version": version, "status": "passed", "astro_version": astro, "required_files": required}
    (ROOT / "DOCS_SITE_AUDIT.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    (ROOT / "DOCS_SITE_AUDIT.md").write_text(
        "# Docs Site Audit\n\n"
        "Status: passed. Astro 7 documentation-site scaffold is present and pinned to astro@7.0.2.\n",
        encoding="utf-8",
    )
    print("docs-site-audit: passed")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
