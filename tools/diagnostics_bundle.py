#!/usr/bin/env python3
"""Create a redacted APFS-RS diagnostics bundle from JSON reports.

This tool is intentionally APFS-media-free: it reads JSON artifacts only and writes
redacted JSON/Markdown summaries to an output directory.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path
from typing import Any

REDACT_KEYS = {
    "password",
    "passphrase",
    "recovery_key",
    "key",
    "secret",
    "token",
    "raw_bytes",
    "bytes",
    "payload",
    "file_contents",
}
PATH_HINT = re.compile(r"([A-Za-z]:\\|/Users/|/home/|/Volumes/|\\\\)")


def redact(value: Any) -> Any:
    if isinstance(value, dict):
        out: dict[str, Any] = {}
        for key, item in value.items():
            key_l = key.lower()
            if any(redact_key in key_l for redact_key in REDACT_KEYS):
                out[key] = "<redacted>"
            else:
                out[key] = redact(item)
        return out
    if isinstance(value, list):
        return [redact(item) for item in value]
    if isinstance(value, str):
        if PATH_HINT.search(value):
            return "<redacted-path>"
        if len(value) > 256:
            digest = hashlib.sha256(value.encode("utf-8", "replace")).hexdigest()
            return f"<redacted-long-string sha256:{digest}>"
        return value
    return value


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as fh:
        return json.load(fh)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", required=True, type=Path, help="Output directory")
    parser.add_argument("reports", nargs="+", type=Path, help="JSON reports to include")
    args = parser.parse_args()

    args.out.mkdir(parents=True, exist_ok=True)
    entries = []
    for report_path in args.reports:
        data = load_json(report_path)
        redacted = redact(data)
        raw = report_path.read_bytes()
        entries.append(
            {
                "source_name": report_path.name,
                "source_sha256": hashlib.sha256(raw).hexdigest(),
                "top_level_keys": sorted(data.keys()) if isinstance(data, dict) else [],
                "redacted_report": redacted,
            }
        )

    bundle = {
        "schema_version": "0.18.0",
        "tool": "diagnostics_bundle.py",
        "safety_note": "This bundle is generated from JSON reports only. It does not open APFS media, mount, decrypt, repair, format, or write to APFS sources.",
        "redaction_policy": {
            "path_like_strings": "redacted",
            "secret_like_keys": sorted(REDACT_KEYS),
            "long_strings": "replaced with sha256 digest",
        },
        "reports": entries,
    }
    (args.out / "diagnostics-bundle.json").write_text(json.dumps(bundle, indent=2) + "\n", encoding="utf-8")

    md = ["# APFS-RS Redacted Diagnostics Bundle", "", bundle["safety_note"], "", "## Reports", ""]
    for entry in entries:
        md.append(f"- `{entry['source_name']}` sha256 `{entry['source_sha256']}` keys: {', '.join(entry['top_level_keys'])}")
    md.append("")
    (args.out / "diagnostics-bundle.md").write_text("\n".join(md), encoding="utf-8")
    print(f"diagnostics-bundle: wrote {args.out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
