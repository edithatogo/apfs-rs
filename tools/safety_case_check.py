#!/usr/bin/env python3
"""Validate that the APFS-RS safety case documents critical hazards and mitigations."""
from __future__ import annotations
import argparse, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
REQUIRED_TERMS = [
    'raw physical-device write',
    'metadata corruption',
    'secret leakage',
    'path traversal',
    'unsupported encryption',
    'panic on corrupt input',
    'read-only default',
    'fixture evidence',
    'Conductor',
    'Codev',
]

def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument('--root', type=Path, default=ROOT)
    args = parser.parse_args()
    root = args.root.resolve()
    path = root / 'SAFETY_CASE.md'
    if not path.exists():
        print('safety-case-check: ERROR: SAFETY_CASE.md is missing', file=sys.stderr)
        return 1
    text = path.read_text(encoding='utf-8')
    missing = [term for term in REQUIRED_TERMS if term.lower() not in text.lower()]
    if missing:
        print('safety-case-check: ERROR: missing terms: ' + ', '.join(missing), file=sys.stderr)
        return 1
    print(f'safety-case-check: passed ({len(REQUIRED_TERMS)} required topics)')
    return 0

if __name__ == '__main__':
    raise SystemExit(main())
