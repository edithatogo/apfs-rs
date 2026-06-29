#!/usr/bin/env python3
"""Validate local handoff, platform setup, and release scaffold files."""
from __future__ import annotations
import json, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
REQUIRED = [
    'LOCAL_HANDOFF.md','CARGO_TRIAGE.md','PLATFORM_SETUP.md','READY_FOR_LOCAL.md','RELEASE_SCAFFOLD.md',
    'packaging/README.md','packaging/windows/README.md','packaging/windows/winget/apfs-rs.yaml','packaging/sbom/README.md',
    '.github/workflows/release.yml','.github/workflows/local-handoff.yml','tools/cargo_error_to_tracks.py',
    'LOCAL_FIRST_RUN.md','KNOWN_UNCOMPILED_RISKS.md','HANDOFF_STATUS.md','REPO_MANIFEST.md',
    'rust-toolchain.toml','.cargo/config.toml','deny.toml','.config/nextest.toml','.devcontainer/devcontainer.json',
    'tools/config_sanity_check.py','tools/local_env_doctor.py','tools/handoff_status.py','tools/repo_manifest.py','tools/known_risk_check.py'
]
CRATES = ['apfs-fuse','apfs-android','apfs-crypto','apfs-write-lab']
def fail(msg: str) -> None:
    print(f'handoff-readiness-check: ERROR: {msg}', file=sys.stderr)
    raise SystemExit(1)
def main() -> int:
    for rel in REQUIRED:
        p=ROOT/rel
        if not p.exists() or not p.read_text(encoding='utf-8', errors='ignore').strip():
            fail(f'missing or empty {rel}')
    for crate in CRATES:
        for rel in [f'crates/{crate}/Cargo.toml', f'crates/{crate}/src/lib.rs']:
            p=ROOT/rel
            if not p.exists() or '#![forbid(unsafe_code)]' not in (ROOT/f'crates/{crate}/src/lib.rs').read_text(encoding='utf-8'):
                fail(f'missing safe scaffold for {crate}')
    print('handoff-readiness-check: passed')
    return 0
if __name__ == '__main__':
    raise SystemExit(main())
