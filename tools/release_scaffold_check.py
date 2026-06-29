#!/usr/bin/env python3
"""Validate release/provenance scaffold without building binaries."""
from __future__ import annotations
import sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
REQUIRED_TERMS = {
    'RELEASE_SCAFFOLD.md': ['SBOM','artifact attestations','SHA-256','WinFsp'],
    '.github/workflows/release.yml': ['attest-build-provenance','cargo test --workspace'],
    'packaging/windows/winget/apfs-rs.yaml': ['PackageIdentifier','PackageVersion'],
}
def fail(msg: str) -> None:
    print(f'release-scaffold-check: ERROR: {msg}', file=sys.stderr)
    raise SystemExit(1)
def main() -> int:
    for rel, terms in REQUIRED_TERMS.items():
        p=ROOT/rel
        if not p.exists(): fail(f'missing {rel}')
        text=p.read_text(encoding='utf-8', errors='ignore')
        for term in terms:
            if term not in text:
                fail(f'{rel} missing term {term!r}')
    print('release-scaffold-check: passed')
    return 0
if __name__ == '__main__':
    raise SystemExit(main())
