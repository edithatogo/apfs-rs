#!/usr/bin/env python3
"""Generate small negative synthetic fixtures for APFS parser refusal paths."""
from __future__ import annotations
import argparse, hashlib, json, shutil
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]


def sha256(path: Path) -> str:
    h = hashlib.sha256(); h.update(path.read_bytes()); return h.hexdigest()


def manifest(fixture_id: str, path: Path, expected: str) -> dict:
    return {
        "schema_version": (ROOT / "VERSION").read_text(encoding="utf-8").strip(),
        "fixture_id": fixture_id,
        "source_type": "synthetic_negative_apfs_image",
        "filename": path.name,
        "sha256": sha256(path),
        "apfs_features": {"synthetic": True, "negative_fixture": True},
        "capability_ids": ["M-001", "M-003", "M-065"],
        "expected_result": expected,
        "redaction": {"contains_personal_data": False, "contains_secret_material": False},
    }


def generate(out_dir: Path) -> list[dict]:
    out_dir.mkdir(parents=True, exist_ok=True)
    base = ROOT / "fixtures/synthetic-nxsb-block0.bin"
    if not base.exists():
        raise SystemExit(f"missing base fixture {base}")
    results = []
    corrupt = out_dir / "negative-corrupt-nxsb-magic.bin"
    data = bytearray(base.read_bytes())
    if len(data) < 36:
        raise SystemExit("base fixture too short")
    data[32:36] = b"NOPE"
    corrupt.write_bytes(data)
    results.append(manifest("negative-corrupt-nxsb-magic", corrupt, "not_apfs"))
    trunc = out_dir / "negative-truncated-nxsb.bin"
    trunc.write_bytes(base.read_bytes()[:64])
    results.append(manifest("negative-truncated-nxsb", trunc, "input_too_short_or_refused"))
    bad_block = out_dir / "negative-invalid-block-size.bin"
    data = bytearray(base.read_bytes())
    data[36:40] = (123).to_bytes(4, "little")
    bad_block.write_bytes(data)
    results.append(manifest("negative-invalid-block-size", bad_block, "invalid_block_size_refusal"))
    manifest_dir = ROOT / "fixtures/manifests"
    manifest_dir.mkdir(exist_ok=True)
    for item in results:
        (manifest_dir / f"{item['fixture_id']}.json").write_text(json.dumps(item, indent=2) + "\n", encoding="utf-8")
    return results


def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--out-dir", type=Path, default=ROOT / "fixtures/negative")
    args = p.parse_args()
    results = generate(args.out_dir)
    print(f"synthetic-negative-fixture-generator: generated {len(results)} fixtures in {args.out_dir}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
