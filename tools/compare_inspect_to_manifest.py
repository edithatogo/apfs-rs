#!/usr/bin/env python3
"""Compare `apfs inspect --json` output with a fixture manifest.

This intentionally performs conservative, shallow checks suitable for the first
real macOS APFS fixture. It does not prove full APFS support.
"""
from __future__ import annotations
import argparse, json, sys
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("inspect_json", type=Path)
    parser.add_argument("manifest_json", type=Path)
    args = parser.parse_args()
    inspect = json.loads(args.inspect_json.read_text())
    manifest = json.loads(args.manifest_json.read_text())

    errors: list[str] = []
    if inspect.get("status") != "apfs_container_detected":
        errors.append(f"inspect status is {inspect.get('status')!r}, expected 'apfs_container_detected'")
    if not inspect.get("safety", {}).get("read_only", False):
        errors.append("inspect report does not state read_only=true")
    if inspect.get("safety", {}).get("physical_write_supported", True):
        errors.append("inspect report unexpectedly allows physical writes")
    if manifest.get("redaction", {}).get("contains_secret_material", True):
        errors.append("manifest is marked as containing secret material")
    if manifest.get("redaction", {}).get("contains_personal_data", True):
        errors.append("manifest is marked as containing personal data")

    container = inspect.get("container") or {}
    if container and container.get("magic") != "NXSB":
        errors.append(f"container magic is {container.get('magic')!r}, expected NXSB")

    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 1
    print("inspect/manifest comparison passed")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
