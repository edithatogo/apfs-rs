#!/usr/bin/env python3
"""Verify SHA256SUMS for the handoff source tree."""
from __future__ import annotations
import hashlib, json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
EXCLUDE_FILES={'HANDOFF_MANIFEST_VERIFY.json','HANDOFF_MANIFEST_VERIFY.md','PACKAGE_INTEGRITY_AUDIT.json','PACKAGE_INTEGRITY_AUDIT.md'}
def sha256(path: Path) -> str:
    h=hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda:f.read(1024*1024), b''): h.update(chunk)
    return h.hexdigest()
def main():
    manifest=ROOT/'SHA256SUMS.txt'; issues=[]; entries={}
    if not manifest.exists(): issues.append('SHA256SUMS.txt is missing')
    else:
        for line in manifest.read_text(encoding='utf-8').splitlines():
            if not line.strip(): continue
            parts=line.split(maxsplit=1)
            if len(parts)!=2: issues.append(f'malformed SHA256SUMS line: {line}'); continue
            rel=parts[1].strip()
            if rel not in EXCLUDE_FILES:
                entries[rel]=parts[0].strip()
    checked=0
    for rel,digest in sorted(entries.items()):
        p=ROOT/rel
        if not p.exists(): issues.append(f'manifest entry missing file: {rel}'); continue
        actual=sha256(p); checked+=1
        if actual != digest: issues.append(f'hash mismatch for {rel}: expected {digest}, got {actual}')
    status='passed' if not issues else 'failed'
    report={'schema_version':'0.27.0','status':status,'checked_entries':checked,'issues':issues}
    (ROOT/'HANDOFF_MANIFEST_VERIFY.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    md=['# Handoff Manifest Verification','',f"Status: `{status}`",f"Checked entries: `{checked}`",'', '## Issues'] + ([f'- {i}' for i in issues] or ['- None'])
    (ROOT/'HANDOFF_MANIFEST_VERIFY.md').write_text('\n'.join(md)+'\n',encoding='utf-8')
    print(f'handoff-manifest-verify: {status} ({checked} entries)')
    return 0 if not issues else 1
if __name__=='__main__': raise SystemExit(main())
